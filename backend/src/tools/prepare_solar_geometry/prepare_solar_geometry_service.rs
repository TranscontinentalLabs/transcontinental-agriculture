use std::f64::consts::{FRAC_PI_2, PI, TAU};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

const ZETA_MIN: f64 = 0.01;
const DEG: f64 = PI / 180.0;

// Buildings uses ten directional samples: 0°, 10°, ..., 90°.
const GLASS_SAMPLES: usize = 10;

/// Model 4: direct transmission through a regular Venlo roof.
///
/// Equation sources:
/// - TiGwerLib 0.7.0:
///   VenloRoof_base and VenloRoof_explAngles.
/// - Buildings 9.1.0:
///   glassPropertyUncoated, smoothInterpolation, spliceFunction.
///
/// Scope: one uncoated, optically symmetric glass pane.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareSolarGeometryInput {
    pub roof: VenloRoofInput,
    pub glass: UncoatedGlassInput,

    /// Solar elevation above the horizon, radians.
    /// Negative at night.
    pub solar_elevation_rad: f64,

    /// Clockwise from north: north=0, east=π/2.
    pub solar_azimuth_rad: f64,

    /// Direct irradiance on a horizontal receiving area, W/m².
    /// This is NOT direct normal irradiance.
    pub direct_horizontal_irradiance_w_m2: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VenloRoofInput {
    /// Ridge/span axis bearing, clockwise from north, radians.
    /// Opposite bearings describe the same roof axis.
    pub ridge_azimuth_rad: f64,

    /// Roof inclination above horizontal, radians.
    pub roof_slope_rad: f64,

    /// Horizontal width of one roof span, metres.
    pub span_width_m: f64,

    /// Span count used to cap repeated optical paths.
    /// Preserve the reference optical model's value explicitly.
    pub optical_span_count: u16,

    /// Cross-sectional dimensions of the gutter beam.
    /// These are NOT heights above the greenhouse floor.
    pub gutter_height_m: f64,
    pub gutter_width_m: f64,

    /// Cross-sectional dimensions of the ridge beam.
    pub ridge_height_m: f64,
    pub ridge_width_m: f64,

    /// Cross-sectional dimensions of glazing bars.
    pub bar_height_m: f64,
    pub bar_width_m: f64,

    /// Distance between consecutive glazing bars.
    pub bar_spacing_m: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UncoatedGlassInput {
    pub thickness_m: f64,

    /// Whole-pane transmission at normal incidence.
    pub normal_transmittance: f64,

    /// Whole-pane reflection at normal incidence.
    /// Equal on the front and back for this implementation.
    pub normal_reflectance: f64,
}

#[derive(Debug, Serialize)]
pub struct PrepareSolarGeometryOutput {
    /// Dimensionless roof-glass transmission after optical paths.
    pub tau_glass: f64,

    /// Dimensionless fraction passing ridges and gutters.
    pub tau_ridges_gutters: f64,

    /// Dimensionless fraction passing glazing bars.
    pub tau_bars: f64,

    /// Product of the three factors.
    pub tau_roof_direct: f64,

    /// Transmitted direct irradiance per horizontal area, W/m².
    /// Not whole-greenhouse power.
    pub transmitted_direct_horizontal_w_m2: f64,

    /// 0: suppressed near/below horizon.
    /// 1–4: reference optical cases.
    pub optical_case: u8,

    /// Effective ray elevation in the cross-span plane.
    pub cross_span_elevation_rad: f64,

    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct PaneOptics {
    tau: f64,
    rho: f64,
}

/// Directional tables with:
/// - a leading interpolation dummy,
/// - ten directional samples,
/// - one hemispherical value,
/// - a trailing interpolation dummy.
struct GlassTable {
    transmission: [f64; 13],
    absorption: [f64; 13],
}

fn bad_request(message: impl Into<String>) -> AppError {
    AppError::BadRequest(message.into())
}

fn validate_input(
    input: &PrepareSolarGeometryInput,
) -> Result<(), AppError> {
    let roof = &input.roof;
    let glass = &input.glass;

    let positive_values = [
        ("span_width_m", roof.span_width_m),
        ("gutter_height_m", roof.gutter_height_m),
        ("gutter_width_m", roof.gutter_width_m),
        ("ridge_height_m", roof.ridge_height_m),
        ("ridge_width_m", roof.ridge_width_m),
        ("bar_height_m", roof.bar_height_m),
        ("bar_width_m", roof.bar_width_m),
        ("bar_spacing_m", roof.bar_spacing_m),
        ("glass.thickness_m", glass.thickness_m),
    ];

    for (name, value) in positive_values {
        if !value.is_finite() || value <= 0.0 {
            return Err(bad_request(format!(
                "{name} must be finite and greater than zero"
            )));
        }
    }

    if roof.optical_span_count == 0 {
        return Err(bad_request(
            "optical_span_count must be at least 1",
        ));
    }

    // Explicit domain for this initial regular-Venlo implementation.
    if !roof.roof_slope_rad.is_finite()
        || roof.roof_slope_rad <= 0.0
        || roof.roof_slope_rad >= PI / 4.0
    {
        return Err(bad_request(
            "roof_slope_rad must be greater than 0 and less than pi/4",
        ));
    }

    for (name, value) in [
        ("ridge_azimuth_rad", roof.ridge_azimuth_rad),
        ("solar_azimuth_rad", input.solar_azimuth_rad),
    ] {
        if !value.is_finite() || !(0.0..TAU).contains(&value) {
            return Err(bad_request(format!(
                "{name} must be in [0, 2*pi)"
            )));
        }
    }

    if !input.solar_elevation_rad.is_finite()
        || !(-FRAC_PI_2..=FRAC_PI_2)
        .contains(&input.solar_elevation_rad)
    {
        return Err(bad_request(
            "solar_elevation_rad must be between -pi/2 and pi/2",
        ));
    }

    if !input.direct_horizontal_irradiance_w_m2.is_finite()
        || input.direct_horizontal_irradiance_w_m2 < 0.0
    {
        return Err(bad_request(
            "direct_horizontal_irradiance_w_m2 must be finite \
             and non-negative",
        ));
    }

    if input.solar_elevation_rad <= 0.0
        && input.direct_horizontal_irradiance_w_m2 > 0.0
    {
        return Err(bad_request(
            "positive direct horizontal irradiance requires \
             the Sun to be above the horizon",
        ));
    }

    let tau = glass.normal_transmittance;
    let rho = glass.normal_reflectance;

    if !tau.is_finite()
        || !rho.is_finite()
        || tau <= 0.0
        || rho <= 0.0
        || tau + rho > 1.0
    {
        return Err(bad_request(
            "glass transmission and reflection must be positive, \
             finite, and sum to no more than 1",
        ));
    }

    if roof.gutter_width_m + roof.ridge_width_m >= roof.span_width_m {
        return Err(bad_request(
            "combined gutter and ridge widths must be less \
             than the span width",
        ));
    }

    if roof.bar_width_m >= roof.bar_spacing_m {
        return Err(bad_request(
            "bar_width_m must be less than bar_spacing_m",
        ));
    }

    Ok(())
}

/// Reference-style smooth transition.
///
/// `width` is half the transition interval.
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

fn safe_acos(value: f64) -> f64 {
    value.clamp(-1.0, 1.0).acos()
}

/// Sum q^0 + ... + q^(count-1).
fn geometric_sum(q: f64, count: i32) -> f64 {
    let mut sum = 0.0;
    let mut power = 1.0;

    for _ in 0..count.max(0) {
        sum += power;
        power *= q;
    }

    sum
}

/// Sum of successive partial geometric sums.
///
/// Equivalent to:
/// sum(s=0..count-1, sum(n=0..s, q^n)).
fn nested_geometric_sum(q: f64, count: i32) -> f64 {
    let mut result = 0.0;
    let mut partial_sum = 0.0;
    let mut power = 1.0;

    for _ in 0..count.max(0) {
        partial_sum += power;
        result += partial_sum;
        power *= q;
    }

    result
}

fn interpolate_glass(values: &[f64; 13], angle: f64) -> f64 {
    // Zero-based equivalent of Buildings' interpolation index.
    let x = 1.0 + 9.0 * angle.clamp(0.0, FRAC_PI_2) / FRAC_PI_2;
    let index = (x.floor() as usize).min(10);
    let u = x - index as f64;

    let y0 = values[index];
    let y1 = values[index + 1];

    let slope0 = (values[index + 1] - values[index - 1]) / 2.0;
    let slope1 = (values[index + 2] - values[index]) / 2.0;

    let u2 = u * u;
    let u3 = u2 * u;

    (2.0 * u3 - 3.0 * u2 + 1.0) * y0
        + (u3 - 2.0 * u2 + u) * slope0
        + (-2.0 * u3 + 3.0 * u2) * y1
        + (u3 - u2) * slope1
}

impl GlassTable {
    fn new(glass: &UncoatedGlassInput) -> Result<Self, AppError> {
        let t0 = glass.normal_transmittance;
        let r0 = glass.normal_reflectance;

        let beta = t0 * t0 - r0 * r0 + 2.0 * r0 + 1.0;
        let discriminant = beta * beta - 4.0 * (2.0 - r0) * r0;

        if !discriminant.is_finite() || discriminant < 0.0 {
            return Err(bad_request(
                "glass properties cannot define the uncoated-glass model",
            ));
        }

        let interface_rho =
            0.5 * (beta - discriminant.sqrt()) / (2.0 - r0);

        let extinction_ratio = (r0 - interface_rho) / (interface_rho * t0);

        if !interface_rho.is_finite()
            || interface_rho <= 0.0
            || interface_rho >= 1.0
            || !extinction_ratio.is_finite()
            || extinction_ratio <= 0.0
        {
            return Err(bad_request(
                "glass properties produce invalid optical coefficients",
            ));
        }

        let absorption_coefficient =
            -extinction_ratio.ln() / glass.thickness_m;

        let root_rho = interface_rho.sqrt();
        let refractive_index = (1.0 + root_rho) / (1.0 - root_rho);

        if !absorption_coefficient.is_finite()
            || absorption_coefficient < -1e-10
            || !refractive_index.is_finite()
        {
            return Err(bad_request(
                "glass properties produce an invalid absorption \
                 coefficient or refractive index",
            ));
        }

        let mut transmission = [0.0; 13];
        let mut absorption = [0.0; 13];

        for sample in 0..GLASS_SAMPLES {
            let angle = sample as f64 * FRAC_PI_2 / 9.0;

            let (tau, rho) = if sample == 0 {
                (t0, r0)
            } else if sample == GLASS_SAMPLES - 1 {
                (0.0, 1.0)
            } else {
                let refracted_angle =
                    (angle.sin() / refractive_index).asin();

                let cos_air = angle.cos();
                let cos_glass = refracted_angle.cos();

                let rho1 = (
                    (refractive_index * cos_air - cos_glass)
                        / (refractive_index * cos_air + cos_glass)
                )
                    .powi(2);

                let rho2 = (
                    (refractive_index * cos_glass - cos_air)
                        / (refractive_index * cos_glass + cos_air)
                )
                    .powi(2);

                let attenuation = (
                    -absorption_coefficient * glass.thickness_m
                        / cos_glass
                )
                    .exp();

                let tau1 = (1.0 - rho1).powi(2) * attenuation
                    / (1.0 - rho1.powi(2) * attenuation.powi(2));

                let tau2 = (1.0 - rho2).powi(2) * attenuation
                    / (1.0 - rho2.powi(2) * attenuation.powi(2));

                let reflectance1 = rho1 * (1.0 + tau1 * attenuation);
                let reflectance2 = rho2 * (1.0 + tau2 * attenuation);

                (
                    0.5 * (tau1 + tau2),
                    0.5 * (reflectance1 + reflectance2),
                )
            };

            transmission[sample + 1] = tau;
            absorption[sample + 1] = 1.0 - tau - rho;
        }

        // Hemispherical values are retained because the reference
        // interpolation uses them in the endpoint stencil.
        let step = FRAC_PI_2 / 9.0;

        for sample in 0..GLASS_SAMPLES {
            let angle = sample as f64 * step;

            let trapezoid_weight =
                if sample == 0 || sample == GLASS_SAMPLES - 1 {
                    0.5
                } else {
                    1.0
                };

            let weight =
                trapezoid_weight * step * 2.0 * angle.cos() * angle.sin();

            transmission[11] += transmission[sample + 1] * weight;
            absorption[11] += absorption[sample + 1] * weight;
        }

        transmission[0] = transmission[1];
        absorption[0] = absorption[1];

        transmission[12] = transmission[11];
        absorption[12] = absorption[11];

        if transmission.iter().chain(absorption.iter())
            .any(|value| !value.is_finite())
        {
            return Err(bad_request(
                "glass properties produce non-finite angular values",
            ));
        }

        Ok(Self {
            transmission,
            absorption,
        })
    }

    fn at(&self, angle: f64) -> PaneOptics {
        let tau = interpolate_glass(&self.transmission, angle).max(0.0);
        let absorption =
            interpolate_glass(&self.absorption, angle).max(0.0);

        PaneOptics {
            tau,
            rho: (1.0 - tau - absorption).max(0.0),
        }
    }
}

pub fn prepare_solar_geometry_service(
    input: PrepareSolarGeometryInput,
) -> Result<PrepareSolarGeometryOutput, AppError> {
    validate_input(&input)?;

    let glass = GlassTable::new(&input.glass)?;

    if input.solar_elevation_rad <= 0.0 {
        return Ok(PrepareSolarGeometryOutput {
            tau_glass: 0.0,
            tau_ridges_gutters: 0.0,
            tau_bars: 0.0,
            tau_roof_direct: 0.0,
            transmitted_direct_horizontal_w_m2: 0.0,
            optical_case: 0,
            cross_span_elevation_rad: ZETA_MIN,
            warnings: Vec::new(),
        });
    }

    let roof = &input.roof;
    let gamma = input.solar_elevation_rad;
    let slope = roof.roof_slope_rad;
    let width = roof.span_width_m;
    let spans = i32::from(roof.optical_span_count);

    // Both bearings use the same north-based convention.
    let relative_azimuth =
        input.solar_azimuth_rad - roof.ridge_azimuth_rad;

    let (sin_gamma, cos_gamma) = gamma.sin_cos();
    let (sin_slope, cos_slope) = slope.sin_cos();

    let across = cos_gamma * relative_azimuth.sin();

    // Incidence on the four oriented surfaces in the reference.
    let angle_a = safe_acos(
        across * sin_slope + sin_gamma * cos_slope,
    );
    let angle_b = safe_acos(
        -across * sin_slope + sin_gamma * cos_slope,
    );
    let angle_c = safe_acos(
        -across * sin_slope - sin_gamma * cos_slope,
    );
    let angle_d = safe_acos(
        across * sin_slope - sin_gamma * cos_slope,
    );

    let kp = across * cos_slope + sin_gamma * sin_slope;
    let kq = across * sin_slope - sin_gamma * cos_slope;
    let ku = across * cos_slope - sin_gamma * sin_slope;
    let kv = -(across * sin_slope + sin_gamma * cos_slope);

    let reflected_b_to_a = safe_acos(
        (kp * cos_slope - kq * sin_slope) * sin_slope
            + (kp * sin_slope + kq * cos_slope) * cos_slope,
    );

    let reflected_a_to_b = safe_acos(
        -(ku * cos_slope + kv * sin_slope) * sin_slope
            + (-ku * sin_slope + kv * cos_slope) * cos_slope,
    );

    let (s1_angle, s2_angle, s4_angle, reflected_angle) =
        if relative_azimuth.sin() >= 0.0 {
            (angle_a, angle_b, angle_d, reflected_b_to_a)
        } else {
            (angle_b, angle_a, angle_c, reflected_a_to_b)
        };

    let s1 = glass.at(s1_angle);
    let s2 = glass.at(s2_angle);
    let s4 = glass.at(s4_angle);
    let s1_ref = glass.at(reflected_angle);

    // The reflected beam reaches S4 at the S2 incidence angle.
    // The glass is symmetric, so the inverted pane is identical.
    let s4_ref = glass.at(s2_angle);

    let bounded_gamma =
        gamma.clamp(-0.999 * FRAC_PI_2, 0.999 * FRAC_PI_2);

    let raw_zeta = (
        sin_gamma
            / (
            bounded_gamma.cos()
                * relative_azimuth.sin().abs().max(1e-3)
        )
    )
        .atan();

    let zeta = splice(raw_zeta, 0.0, gamma - DEG, DEG)
        .max(ZETA_MIN);

    let l0 = width / (2.0 * cos_slope);
    let l1 = l0 * (zeta + slope).sin();
    let l2 = l0 * (zeta - slope).sin();

    // ---------------------------------------------------------
    // Glass transmission: reference optical cases 0–4.
    // ---------------------------------------------------------

    let (tau_glass, optical_case) = if zeta - ZETA_MIN <= 1e-6 {
        (0.0, 0)
    } else if zeta < slope {
        let m = (
            0.5 * (1.0 + slope.tan() / zeta.tan())
        )
            .min(spans as f64)
            .floor() as i32;

        let la = (
            l0 * (
                m as f64 * (slope + zeta).sin()
                    - (m + 1) as f64 * (slope - zeta).sin()
            )
        )
            .max(0.0);

        let lb = l0 * (
            (1 - m) as f64 * (slope + zeta).sin()
                + m as f64 * (slope - zeta).sin()
        );

        let q = s1.tau * s4.tau;
        let k1 = geometric_sum(q, m - 1);
        let k2 = geometric_sum(q, m);

        let tau = (
            la * s1.tau * (s4.rho * k1 + q.powi(m - 1))
                + lb * s1.tau * (s4.rho * k2 + q.powi(m))
        ) / (la + lb).max(1e-6);

        (tau, 1)
    } else if zeta < 3.0 * slope {
        let direct =
            (l1 * s1.tau + l2 * s2.tau) / (l1 + l2).max(1e-6);

        let khi = (2.0 * slope - zeta).abs();

        let m = (
            0.5 * (1.0 + slope.tan() / khi.max(1e-4).tan())
        )
            .min(spans as f64)
            .floor() as i32;

        let q = s1_ref.tau * s4_ref.tau;
        let denominator = (l1 + l2).max(l0 / 1000.0);

        if zeta < 2.0 * slope {
            let la = l0 * (
                m as f64 * (zeta - slope).sin()
                    - (m - 1) as f64 * (3.0 * slope - zeta).sin()
            );

            let lb = l0 * (
                -(zeta - slope).sin()
                    + (3.0 * slope - zeta).sin()
            );

            let k1 = geometric_sum(q, m - 1);
            let k2 = nested_geometric_sum(q, m - 2);

            let reflected = (
                la * s2.rho * s1_ref.tau
                    * (s4_ref.rho * k1 + q.powi(m - 1))
                    + lb * s2.rho * s1_ref.tau
                    * (s4_ref.rho * k2 + k1)
            ) / denominator;

            (direct + reflected, 2)
        } else {
            let la = l0 * (
                -(m - 1) as f64 * (zeta - slope).sin()
                    + m as f64 * (3.0 * slope - zeta).sin()
            );

            let lb = l0 * (
                (zeta - slope).sin()
                    - (3.0 * slope - zeta).sin()
            );

            let k1 = geometric_sum(q, m);
            let k2 = nested_geometric_sum(q, m - 1);

            let reflected = (
                la * s2.rho * s1_ref.tau * s4_ref.rho * k1
                    + lb * s2.rho * s1_ref.tau * s4_ref.rho * k2
            ) / denominator;

            (direct + reflected, 3)
        }
    } else {
        let tau =
            (l1 * s1.tau + l2 * s2.tau) / (l1 + l2).max(l0 / 1000.0);

        (tau, 4)
    };

    // ---------------------------------------------------------
    // Ridge and gutter shadows, including overlap.
    // ---------------------------------------------------------

    let hr = roof.ridge_height_m;
    let wr = roof.ridge_width_m;
    let hg = roof.gutter_height_m;
    let wg = roof.gutter_width_m;

    let lr = hr * zeta.cos() + wr * zeta.sin();
    let lg = hg * zeta.cos() + wg * zeta.sin();

    let m2 = 1 + (
        (0.5 * slope.tan() / zeta.tan())
            .max(0.0)
            .min((spans - 1) as f64)
            .floor() as i32
    );

    let distance = (2 * m2 - 1) as f64 * width;
    let rise_twice = width * slope.tan();

    let zeta1 =
        ((rise_twice - hr - hg) / (distance + wr + wg)).atan();
    let zeta2 =
        ((rise_twice + hr + hg) / (distance - wr - wg)).atan();
    let zeta3 =
        ((rise_twice + hr - hg) / (distance - wr + wg)).atan();
    let zeta4 =
        ((rise_twice - hr + hg) / (distance + wr - wg)).atan();

    let overlap = if zeta - ZETA_MIN <= 1e-6 {
        0.0
    } else if zeta > zeta3 && zeta <= zeta4 {
        lr
    } else if zeta > zeta1 && zeta <= zeta3 {
        let le = 0.5 * (distance + wg) + 0.5 * hg / zeta.tan();
        let ld = (0.5 * rise_twice - 0.5 * hr) / zeta.tan()
            - 0.5 * wr;

        (le - ld) * zeta.sin()
    } else if zeta > zeta4 && zeta < zeta2 {
        let le = (0.5 * rise_twice + 0.5 * hr) / zeta.tan()
            + 0.5 * wr;
        let ld = 0.5 * (distance - wg) - 0.5 * hg / zeta.tan();

        (le - ld) * zeta.sin()
    } else {
        0.0
    };

    let structural_transmission = (
        1.0 - (lg + lr - overlap).max(0.0) / (width * zeta.sin())
    )
        .max(0.0);

    let tau_ridges_gutters = splice(
        structural_transmission,
        0.0,
        gamma - DEG,
        DEG,
    );

    // ---------------------------------------------------------
    // Glazing-bar shadows.
    // ---------------------------------------------------------

    let nu = splice(
        (
            gamma.tan()
                / relative_azimuth.cos().abs().max(1e-5)
        )
            .atan(),
        1e-4,
        gamma - 0.1 * DEG,
        0.1 * DEG,
    );

    let hs = l0 * sin_slope;

    let x_plus = 0.5 * width + hs / zeta.tan();
    let x_minus = 0.5 * width - hs / zeta.tan();
    let y = hs / nu.tan();

    let lx = x_plus.hypot(y);
    let ly = x_minus.hypot(y);

    let bar_x = roof.bar_height_m / zeta.tan();
    let bar_y = roof.bar_width_m + roof.bar_height_m / nu.tan();

    let ab = bar_x.hypot(bar_y);

    // Use atan of the ratios, matching the reference branches.
    let bad = (bar_y / bar_x).atan() - (y / x_plus).atan();
    let fed = PI - (bar_y / bar_x).atan() - (y / x_minus).atan();

    let d1 = ab * bad.sin().abs();
    let d2 = ab * fed.sin().abs();

    let denominator = roof.bar_spacing_m * width;
    let shadow1 = lx * d1 / denominator;
    let shadow2 = ly * d2 / denominator;

    let bars_high = 1.0 - shadow1 - shadow2;
    let bars_low = (1.0 - shadow1) * (1.0 - shadow2);

    let tau_bars = splice(
        splice(bars_high, bars_low, zeta - slope, DEG),
        0.0,
        gamma - 2.0 * DEG,
        2.0 * DEG,
    )
        .max(0.0);

    // ---------------------------------------------------------
    // Combined direct roof transmission.
    // ---------------------------------------------------------

    let tau_roof_direct =
        tau_glass * tau_ridges_gutters * tau_bars;

    let transmitted_direct_horizontal_w_m2 =
        tau_roof_direct * input.direct_horizontal_irradiance_w_m2;

    let calculated_values = [
        tau_glass,
        tau_ridges_gutters,
        tau_bars,
        tau_roof_direct,
        transmitted_direct_horizontal_w_m2,
        zeta,
    ];

    if calculated_values.iter().any(|value| !value.is_finite()) {
        return Err(AppError::Internal(
            "Venlo optics produced a non-finite result; \
             inspect the geometry and angular boundary case"
                .into(),
        ));
    }

    let mut warnings = Vec::new();

    if lr > lg {
        warnings.push(
            "The ridge shadow exceeds the gutter shadow. \
             The reference shadow model assumes the opposite; \
             this configuration is outside that assumption."
                .into(),
        );
    }

    for (name, value) in [
        ("tau_glass", tau_glass),
        ("tau_ridges_gutters", tau_ridges_gutters),
        ("tau_bars", tau_bars),
        ("tau_roof_direct", tau_roof_direct),
    ] {
        if !(0.0..=1.0).contains(&value) {
            warnings.push(format!(
                "{name}={value} lies outside [0, 1]; \
                 retained for diagnosis rather than silently clipped"
            ));
        }
    }

    Ok(PrepareSolarGeometryOutput {
        tau_glass,
        tau_ridges_gutters,
        tau_bars,
        tau_roof_direct,
        transmitted_direct_horizontal_w_m2,
        optical_case,
        cross_span_elevation_rad: zeta,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_configuration() -> PrepareSolarGeometryInput {
        PrepareSolarGeometryInput {
            roof: VenloRoofInput {
                ridge_azimuth_rad: 331.0_f64.to_radians(),
                roof_slope_rad: 0.38397243543875,
                span_width_m: 4.0,
                optical_span_count: 10,
                gutter_height_m: 0.155,
                gutter_width_m: 0.105,
                ridge_height_m: 0.035,
                ridge_width_m: 0.040,
                bar_height_m: 0.052,
                bar_width_m: 0.022,
                bar_spacing_m: 1.35,
            },
            glass: UncoatedGlassInput {
                thickness_m: 0.004,
                normal_transmittance: 0.90,
                normal_reflectance: 0.08,
            },

            // Illustrative solar condition, not a research observation.
            solar_elevation_rad: 45.0_f64.to_radians(),
            solar_azimuth_rad: 180.0_f64.to_radians(),
            direct_horizontal_irradiance_w_m2: 600.0,
        }
    }

    #[test]
    fn glass_matches_supplied_normal_incidence_properties() {
        let input = reference_configuration();
        let table = GlassTable::new(&input.glass).unwrap();

        let normal = table.at(0.0);

        assert!((normal.tau - 0.90).abs() < 1e-12);
        assert!((normal.rho - 0.08).abs() < 1e-12);
    }

    #[test]
    fn glass_blocks_transmission_at_grazing_incidence() {
        let input = reference_configuration();
        let table = GlassTable::new(&input.glass).unwrap();

        let grazing = table.at(FRAC_PI_2);

        assert!(grazing.tau.abs() < 1e-12);
        assert!((grazing.rho - 1.0).abs() < 1e-12);
    }

    #[test]
    fn night_has_no_transmitted_direct_radiation() {
        let mut input = reference_configuration();
        input.solar_elevation_rad = -10.0_f64.to_radians();
        input.direct_horizontal_irradiance_w_m2 = 0.0;

        let output = prepare_solar_geometry_service(input).unwrap();

        assert_eq!(output.optical_case, 0);
        assert_eq!(output.tau_roof_direct, 0.0);
        assert_eq!(output.transmitted_direct_horizontal_w_m2, 0.0);
    }

    #[test]
    fn reversing_ridge_bearing_preserves_transmission() {
        let input = reference_configuration();
        let mut reversed = input.clone();

        reversed.roof.ridge_azimuth_rad =
            (reversed.roof.ridge_azimuth_rad + PI).rem_euclid(TAU);

        let original_output =
            prepare_solar_geometry_service(input).unwrap();
        let reversed_output =
            prepare_solar_geometry_service(reversed).unwrap();

        assert!(
            (
                original_output.tau_roof_direct
                    - reversed_output.tau_roof_direct
            )
                .abs()
                < 1e-10
        );
    }

    #[test]
    fn reference_geometry_stays_finite_across_sky_grid() {
        for elevation_deg in 1..=90 {
            for azimuth_deg in (0..360).step_by(15) {
                let mut input = reference_configuration();

                input.solar_elevation_rad =
                    (elevation_deg as f64).to_radians();
                input.solar_azimuth_rad =
                    (azimuth_deg as f64).to_radians();

                let output =
                    prepare_solar_geometry_service(input).unwrap();

                assert!(output.tau_roof_direct.is_finite());
                assert!(output.tau_roof_direct >= 0.0);
                assert!(output.tau_roof_direct <= 1.0 + 1e-9);
            }
        }
    }
}