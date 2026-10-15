use std::f64::consts::{FRAC_PI_2, PI, TAU};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::prepare_solar_geometry_service::{
    prepare_solar_geometry_service,
    PrepareSolarGeometryInput,
    UncoatedGlassInput,
    VenloRoofInput,
};

// Reference diffuse-table generator uses 5° integration steps.
const STEP_RAD: f64 = PI / 36.0;

// Azimuth: 0°, 5°, ..., 360°.
const AZIMUTH_COUNT: usize = 73;

// Zenith: 0°, 5°, ..., 90°.
const ZENITH_COUNT: usize = 19;

// Numerical regularization used by the reference.
const DENOMINATOR_OFFSET: f64 = 0.001;

#[derive(Debug, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum DirectionalRoofInput {
    /// Values sampled from the reference direct-transmission table.
    ///
    /// Rows: azimuth 0°, 5°, ..., 360°.
    /// Columns: zenith 0°, 5°, ..., 90°.
    ///
    /// This is NOT the complete 1° source table.
    ReferenceGrid {
        source: String,
        tau: Vec<Vec<f64>>,
    },

    /// Calculate directional transmission using our model 4 service.
    ///
    /// These values are not rounded to the reference table's
    /// three significant digits.
    Calculate {
        roof: VenloRoofInput,
        glass: UncoatedGlassInput,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffuseSolarRecordInput {
    /// Same simulation time axis used by prepared weather.
    pub time_s: f64,

    /// Sun's zenith angle, radians: 0 overhead, pi/2 at horizon.
    pub solar_zenith_rad: f64,

    /// Buildings solar-azimuth convention used by TauDiff:
    /// south=0, east=-pi/2, west=+pi/2.
    ///
    /// This differs from model 4's north-based azimuth.
    pub solar_azimuth_south_rad: f64,

    pub global_horizontal_irradiance_w_m2: f64,
    pub diffuse_horizontal_irradiance_w_m2: f64,

    /// Model 4 result for this timestamp and the SAME roof.
    pub tau_roof_direct: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareDiffuseSolarGeometryInput {
    pub directional_roof: DirectionalRoofInput,

    /// Site elevation above sea level, metres.
    /// Used by the reference relative-air-mass calculation.
    pub site_altitude_m: f64,

    pub records: Vec<DiffuseSolarRecordInput>,
}

#[derive(Debug, Serialize)]
pub struct PreparedDiffuseSolarRecord {
    pub time_s: f64,

    pub tau_roof_diffuse: f64,
    pub tau_roof_direct: f64,

    /// Reference definition: GHI - DHI.
    pub direct_horizontal_irradiance_w_m2: f64,

    /// All three irradiances below are per horizontal area.
    pub transmitted_direct_horizontal_w_m2: f64,
    pub transmitted_diffuse_horizontal_w_m2: f64,
    pub transmitted_total_horizontal_w_m2: f64,

    /// Diagnostics for the sky integration.
    pub integration_numerator: f64,
    pub integration_denominator: f64,
}

#[derive(Debug, Serialize)]
pub struct PrepareDiffuseSolarGeometryOutput {
    pub sky_model: String,
    pub directional_grid_source: String,
    pub azimuth_step_deg: u8,
    pub zenith_step_deg: u8,
    pub records: Vec<PreparedDiffuseSolarRecord>,
}

#[derive(Clone, Copy)]
struct IgawaSky {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    solar_zenith_rad: f64,
    solar_azimuth_south_rad: f64,
    normalization: f64,
}

fn bad_request(message: impl Into<String>) -> AppError {
    AppError::BadRequest(message.into())
}

fn record_error(index: usize, message: impl Into<String>) -> AppError {
    bad_request(format!(
        "diffuse solar record {index}: {}",
        message.into()
    ))
}

/// Buildings regStep.
fn reg_step(x: f64, y1: f64, y2: f64, width: f64) -> f64 {
    if x > width {
        y1
    } else if x < -width {
        y2
    } else {
        let scaled = x / width;

        scaled * (scaled * scaled - 3.0) * (y2 - y1) / 4.0
            + (y1 + y2) / 2.0
    }
}

/// Buildings smoothMin.
fn smooth_min(x1: f64, x2: f64, width: f64) -> f64 {
    reg_step(x2 - x1, x1, x2, width)
}

/// Buildings spliceFunction.
fn splice(pos: f64, neg: f64, x: f64, width: f64) -> f64 {
    let scaled = x / width;
    let limit = 0.999_999_999_9;

    if scaled <= -limit {
        neg
    } else if scaled >= limit {
        pos
    } else {
        let weight =
            ((scaled * FRAC_PI_2).tan().tanh() + 1.0) / 2.0;

        pos * weight + neg * (1.0 - weight)
    }
}

fn relative_air_mass(
    solar_zenith_rad: f64,
    altitude_m: f64,
) -> f64 {
    let limited_zenith =
        smooth_min(solar_zenith_rad, FRAC_PI_2, 0.01);

    let zenith_deg = limited_zenith.to_degrees();

    (-0.0001184 * altitude_m).exp()
        / (
        limited_zenith.cos()
            + 0.15 * (93.9 - zenith_deg).powf(-1.253)
    )
}

fn validate_grid(tau: &[Vec<f64>]) -> Result<(), AppError> {
    if tau.len() != AZIMUTH_COUNT {
        return Err(bad_request(
            "directional grid must contain 73 azimuth rows",
        ));
    }

    for (row_index, row) in tau.iter().enumerate() {
        if row.len() != ZENITH_COUNT {
            return Err(bad_request(format!(
                "directional grid row {row_index} must contain \
                 19 zenith values"
            )));
        }

        for (column_index, value) in row.iter().enumerate() {
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err(bad_request(format!(
                    "directional grid [{row_index}][{column_index}] \
                     must be finite and between 0 and 1"
                )));
            }
        }
    }

    // 0° and 360° represent the same direction.
    for column in 0..ZENITH_COUNT {
        if (tau[0][column] - tau[AZIMUTH_COUNT - 1][column]).abs()
            > 1e-9
        {
            return Err(bad_request(
                "directional grid rows for 0° and 360° must agree",
            ));
        }
    }

    Ok(())
}

fn prepare_directional_grid(
    input: DirectionalRoofInput,
) -> Result<(Vec<Vec<f64>>, String), AppError> {
    match input {
        DirectionalRoofInput::ReferenceGrid { source, tau } => {
            if source.trim().is_empty() {
                return Err(bad_request(
                    "reference grid must identify its source",
                ));
            }

            validate_grid(&tau)?;

            Ok((tau, source))
        }

        DirectionalRoofInput::Calculate { roof, glass } => {
            let mut tau =
                vec![vec![0.0; ZENITH_COUNT]; AZIMUTH_COUNT];

            // Calculate 0° through 355°. Copy 0° to 360° afterward.
            for azimuth_index in 0..AZIMUTH_COUNT - 1 {
                let azimuth =
                    azimuth_index as f64 * STEP_RAD;

                for zenith_index in 0..ZENITH_COUNT {
                    let zenith =
                        zenith_index as f64 * STEP_RAD;

                    let elevation = FRAC_PI_2 - zenith;

                    let output = prepare_solar_geometry_service(
                        PrepareSolarGeometryInput {
                            roof: roof.clone(),
                            glass: glass.clone(),
                            solar_elevation_rad: elevation,
                            solar_azimuth_rad: azimuth,

                            // Transmission coefficients do not depend
                            // on the beam's magnitude.
                            //
                            // Use zero at the horizon to satisfy
                            // model 4's input validation.
                            direct_horizontal_irradiance_w_m2:
                            if elevation > 0.0 { 1.0 } else { 0.0 },
                        },
                    )?;

                    // Do not hide model 4 domain warnings.
                    if !output.warnings.is_empty() {
                        return Err(bad_request(format!(
                            "directional roof calculation at azimuth={}°, \
                             zenith={}° requires review: {}",
                            azimuth.to_degrees(),
                            zenith.to_degrees(),
                            output.warnings.join("; ")
                        )));
                    }

                    tau[azimuth_index][zenith_index] =
                        output.tau_roof_direct;
                }
            }

            tau[AZIMUTH_COUNT - 1] = tau[0].clone();

            validate_grid(&tau)?;

            Ok((
                tau,
                "Rust model 4; unrounded 5-degree directional samples"
                    .into(),
            ))
        }
    }
}

fn validate_record(
    index: usize,
    record: &DiffuseSolarRecordInput,
) -> Result<(), AppError> {
    let fields = [
        ("time_s", record.time_s),
        ("solar_zenith_rad", record.solar_zenith_rad),
        (
            "solar_azimuth_south_rad",
            record.solar_azimuth_south_rad,
        ),
        (
            "global_horizontal_irradiance_w_m2",
            record.global_horizontal_irradiance_w_m2,
        ),
        (
            "diffuse_horizontal_irradiance_w_m2",
            record.diffuse_horizontal_irradiance_w_m2,
        ),
        ("tau_roof_direct", record.tau_roof_direct),
    ];

    for (name, value) in fields {
        if !value.is_finite() {
            return Err(record_error(
                index,
                format!("{name} must be finite"),
            ));
        }
    }

    if !(0.0..=PI).contains(&record.solar_zenith_rad) {
        return Err(record_error(
            index,
            "solar_zenith_rad must be between 0 and pi",
        ));
    }

    if !(-PI..=PI).contains(&record.solar_azimuth_south_rad) {
        return Err(record_error(
            index,
            "solar_azimuth_south_rad must be between -pi and pi",
        ));
    }

    let ghi = record.global_horizontal_irradiance_w_m2;
    let dhi = record.diffuse_horizontal_irradiance_w_m2;

    if ghi < 0.0 || dhi < 0.0 || dhi > ghi {
        return Err(record_error(
            index,
            "require GHI >= DHI >= 0; investigate inconsistent \
             measurements before running this model",
        ));
    }

    if !(0.0..=1.0).contains(&record.tau_roof_direct) {
        return Err(record_error(
            index,
            "tau_roof_direct must be between 0 and 1",
        ));
    }

    if record.solar_zenith_rad >= FRAC_PI_2 && ghi > dhi {
        return Err(record_error(
            index,
            "positive direct horizontal irradiance below \
             the horizon is inconsistent",
        ));
    }

    Ok(())
}

impl IgawaSky {
    fn new(
        record: &DiffuseSolarRecordInput,
        altitude_m: f64,
    ) -> Result<Self, AppError> {
        let mass = relative_air_mass(
            record.solar_zenith_rad,
            altitude_m,
        );

        if !mass.is_finite() || mass <= 0.0 {
            return Err(bad_request(
                "relative air mass must be finite and positive",
            ));
        }

        // Reference extraterrestrial normal irradiance: 1367 W/m².
        let standard_global =
            0.84 * (1367.0 / mass) * (-0.0675 * mass).exp();

        if !standard_global.is_finite() || standard_global <= 0.0 {
            return Err(bad_request(
                "Igawa standard global irradiance is invalid",
            ));
        }

        let ghi = record.global_horizontal_irradiance_w_m2;
        let dhi = record.diffuse_horizontal_irradiance_w_m2;

        let clear_sky_index =
            (ghi / standard_global).clamp(0.0, 1.001);

        let cloud_ratio = dhi / (0.0001 + ghi);

        let standard_cloud_ratio =
            0.01299
                + 0.07698 * mass
                - 0.003857 * mass.powi(2)
                + 0.0001054 * mass.powi(3)
                - 0.000001031 * mass.powi(4);

        let cloud_denominator = 1.0 - standard_cloud_ratio;

        if !cloud_denominator.is_finite()
            || cloud_denominator == 0.0
        {
            return Err(bad_request(
                "Igawa cloud-index denominator is invalid",
            ));
        }

        let cloud_index =
            ((1.0 - cloud_ratio) / cloud_denominator)
                .clamp(0.0, 1.001);

        let sky_index = clear_sky_index + cloud_index.sqrt();

        let a =
            4.5 / (1.0 + 0.15 * (3.4 * sky_index).exp()) - 1.04;

        let b =
            -1.0 / (1.0 + 0.17 * (1.3 * sky_index).exp()) - 0.05;

        let c =
            1.77
                * (1.22 * sky_index).powf(3.56)
                * (0.2 * sky_index).exp()
                * (2.1 - sky_index).max(0.0).powf(0.8);

        let d =
            -3.05 / (1.0 + 10.6 * (-3.4 * sky_index).exp());

        let e =
            0.48 / (1.0 + 245.0 * (-4.13 * sky_index).exp());

        let sun_zenith = record.solar_zenith_rad;

        let normalization =
            (1.0 + a * b.exp())
                * (
                1.0
                    + c * (
                    (d * sun_zenith).exp()
                        - (d * FRAC_PI_2).exp()
                )
                    + e * sun_zenith.cos().powi(2)
            );

        if [a, b, c, d, e, normalization]
            .iter()
            .any(|value| !value.is_finite())
            || normalization <= 0.0
        {
            return Err(bad_request(
                "Igawa sky coefficients or normalization are invalid",
            ));
        }

        Ok(Self {
            a,
            b,
            c,
            d,
            e,
            solar_zenith_rad: sun_zenith,
            solar_azimuth_south_rad:
            record.solar_azimuth_south_rad,
            normalization,
        })
    }

    fn relative_radiance(
        &self,
        grid_azimuth_rad: f64,
        sky_zenith_rad: f64,
    ) -> f64 {
        // Preserve the angle expression in reference TauDiff.
        // Its sky calculation uses Buildings' south-based
        // solar azimuth directly against the integration grid.
        let separation_cos =
            self.solar_zenith_rad.cos() * sky_zenith_rad.cos()
                + self.solar_zenith_rad.sin()
                * sky_zenith_rad.sin()
                * (
                grid_azimuth_rad
                    - self.solar_azimuth_south_rad
            )
                .cos();

        let separation = separation_cos.clamp(-1.0, 1.0).acos();

        let gradation =
            1.0
                + self.a
                * (
                self.b / (0.01 + sky_zenith_rad.cos())
            )
                .exp();

        let scattering =
            1.0
                + self.c
                * (
                (self.d * separation).exp()
                    - (self.d * FRAC_PI_2).exp()
            )
                + self.e * separation.cos().powi(2);

        gradation * scattering / self.normalization
    }
}

/// The reference uses an endpoint-inclusive rectangular sum.
/// It is not a trapezoidal or midpoint quadrature.
fn integrate_sky(
    tau: &[Vec<f64>],
    relative_radiance: impl Fn(f64, f64) -> f64,
) -> Result<(f64, f64), AppError> {
    let mut numerator = 0.0;
    let mut denominator = 0.0;

    // Zenith outermost, matching the source summation order.
    for zenith_index in 0..ZENITH_COUNT {
        let zenith = zenith_index as f64 * STEP_RAD;

        let projected_solid_angle =
            zenith.sin() * zenith.cos() * STEP_RAD * STEP_RAD;

        for azimuth_index in 0..AZIMUTH_COUNT {
            let azimuth = azimuth_index as f64 * STEP_RAD;

            let radiance = relative_radiance(azimuth, zenith);

            if !radiance.is_finite() || radiance < 0.0 {
                return Err(bad_request(
                    "sky model produced invalid relative radiance",
                ));
            }

            let weight = radiance * projected_solid_angle;

            denominator += weight;
            numerator += tau[azimuth_index][zenith_index] * weight;
        }
    }

    if !numerator.is_finite()
        || !denominator.is_finite()
        || denominator <= 0.0
    {
        return Err(bad_request(
            "diffuse sky integration is non-finite or has zero weight",
        ));
    }

    Ok((numerator, denominator))
}

pub fn prepare_diffuse_solar_geometry_service(
    payload: PrepareDiffuseSolarGeometryInput,
) -> Result<PrepareDiffuseSolarGeometryOutput, AppError> {
    let PrepareDiffuseSolarGeometryInput {
        directional_roof,
        site_altitude_m,
        records: input_records,
    } = payload;

    if !site_altitude_m.is_finite() {
        return Err(bad_request(
            "site_altitude_m must be finite",
        ));
    }

    if input_records.is_empty() {
        return Err(bad_request(
            "at least one diffuse solar record is required",
        ));
    }

    // Validate all records before calculating the directional grid.
    let mut previous_time: Option<f64> = None;

    for (index, record) in input_records.iter().enumerate() {
        validate_record(index, record)?;

        if let Some(previous) = previous_time {
            if record.time_s <= previous {
                return Err(record_error(
                    index,
                    "timestamps must be strictly increasing",
                ));
            }
        }

        previous_time = Some(record.time_s);
    }

    // Prepare once for the entire batch, not once per timestamp.
    let (tau, directional_grid_source) =
        prepare_directional_grid(directional_roof)?;

    let mut records = Vec::with_capacity(input_records.len());

    for (index, record) in input_records.into_iter().enumerate() {
        let sky = IgawaSky::new(&record, site_altitude_m)
            .map_err(|error| {
                record_error(index, format!("{error:?}"))
            })?;

        let (numerator, denominator) = integrate_sky(
            &tau,
            |azimuth, zenith| sky.relative_radiance(azimuth, zenith),
        )
            .map_err(|error| {
                record_error(index, format!("{error:?}"))
            })?;

        let raw_tau =
            numerator / (DENOMINATOR_OFFSET + denominator);

        let elevation = FRAC_PI_2 - record.solar_zenith_rad;

        let tau_diffuse =
            splice(raw_tau, 0.0, elevation, 0.01);

        // Preserve reference radiation-component definition.
        let direct_horizontal =
            record.global_horizontal_irradiance_w_m2
                - record.diffuse_horizontal_irradiance_w_m2;

        let transmitted_direct =
            record.tau_roof_direct * direct_horizontal;

        let transmitted_diffuse =
            tau_diffuse * record.diffuse_horizontal_irradiance_w_m2;

        let transmitted_total =
            transmitted_direct + transmitted_diffuse;

        if [
            tau_diffuse,
            transmitted_direct,
            transmitted_diffuse,
            transmitted_total,
        ]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(record_error(
                index,
                "transmitted radiation calculation is non-finite",
            ));
        }

        records.push(PreparedDiffuseSolarRecord {
            time_s: record.time_s,
            tau_roof_diffuse: tau_diffuse,
            tau_roof_direct: record.tau_roof_direct,
            direct_horizontal_irradiance_w_m2: direct_horizontal,
            transmitted_direct_horizontal_w_m2: transmitted_direct,
            transmitted_diffuse_horizontal_w_m2: transmitted_diffuse,
            transmitted_total_horizontal_w_m2: transmitted_total,
            integration_numerator: numerator,
            integration_denominator: denominator,
        });
    }

    Ok(PrepareDiffuseSolarGeometryOutput {
        sky_model: "igawa_reference_equations".into(),
        directional_grid_source,
        azimuth_step_deg: 5,
        zenith_step_deg: 5,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_transmission_preserves_reference_regularization() {
        let tau = vec![
            vec![0.7; ZENITH_COUNT];
            AZIMUTH_COUNT
        ];

        let (numerator, denominator) =
            integrate_sky(&tau, |_, _| 1.0).unwrap();

        assert!((numerator - 0.7 * denominator).abs() < 1e-10);

        let result =
            numerator / (DENOMINATOR_OFFSET + denominator);

        let expected =
            0.7 * denominator
                / (DENOMINATOR_OFFSET + denominator);

        assert!((result - expected).abs() < 1e-12);

        // The source's +0.001 deliberately makes this slightly
        // lower than the ideal normalized average.
        assert!(result < 0.7);
        assert!(result > 0.69);
    }

    #[test]
    fn rejects_inconsistent_periodic_grid() {
        let mut tau = vec![
            vec![0.7; ZENITH_COUNT];
            AZIMUTH_COUNT
        ];

        tau[AZIMUTH_COUNT - 1][5] = 0.6;

        assert!(matches!(
            validate_grid(&tau),
            Err(AppError::BadRequest(_))
        ));
    }

    #[test]
    fn zero_transmission_blocks_all_diffuse_light() {
        let tau = vec![
            vec![0.0; ZENITH_COUNT];
            AZIMUTH_COUNT
        ];

        let (numerator, denominator) =
            integrate_sky(&tau, |_, _| 1.0).unwrap();

        assert_eq!(numerator, 0.0);
        assert!(denominator > 0.0);
    }

    #[test]
    fn nighttime_transition_reaches_zero() {
        let tau = splice(0.7, 0.0, -0.02, 0.01);

        assert_eq!(tau, 0.0);
    }

    #[test]
    fn combines_direct_and_diffuse_irradiance() {
        let result = prepare_diffuse_solar_geometry_service(
            PrepareDiffuseSolarGeometryInput {
                directional_roof: DirectionalRoofInput::ReferenceGrid {
                    source: "synthetic constant-transmission test grid"
                        .into(),
                    tau: vec![
                        vec![0.7; ZENITH_COUNT];
                        AZIMUTH_COUNT
                    ],
                },
                site_altitude_m: 30.0,
                records: vec![DiffuseSolarRecordInput {
                    time_s: 0.0,
                    solar_zenith_rad: PI / 4.0,
                    solar_azimuth_south_rad: 0.0,
                    global_horizontal_irradiance_w_m2: 600.0,
                    diffuse_horizontal_irradiance_w_m2: 200.0,
                    tau_roof_direct: 0.8,
                }],
            },
        )
            .unwrap();

        let record = &result.records[0];

        // Direct horizontal = 600 - 200 = 400 W/m².
        // Transmitted direct = 0.8 × 400 = 320 W/m².
        assert!(
            (record.transmitted_direct_horizontal_w_m2 - 320.0).abs()
                < 1e-10
        );

        assert!(
            (
                record.transmitted_total_horizontal_w_m2
                    - 320.0
                    - record.transmitted_diffuse_horizontal_w_m2
            )
                .abs()
                < 1e-10
        );

        assert!(record.tau_roof_diffuse > 0.69);
        assert!(record.tau_roof_diffuse < 0.7);
    }
}