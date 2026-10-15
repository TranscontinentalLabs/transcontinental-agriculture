use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_SOLAR_GEOMETRY_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareSolarGeometryInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Model 3 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct VenloRoofParametersOutput {
    pub ridge_azimuth_deg: f64,
    pub ridge_azimuth_rad: f64,
    pub roof_slope_deg: f64,
    pub roof_slope_rad: f64,
    pub span_width_m: f64,
    pub optical_span_count: u16,
    pub gutter_height_m: f64,
    pub gutter_width_m: f64,
    pub ridge_height_m: f64,
    pub ridge_width_m: f64,
    pub bar_height_m: f64,
    pub bar_width_m: f64,
    pub bar_spacing_m: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct UncoatedGlassParametersOutput {
    pub thickness_m: f64,
    pub normal_transmittance: f64,
    pub normal_reflectance: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrientationSensitivityFindingOutput {
    pub date_iso: &'static str,
    pub max_abs_tau_direct_diff_vs_north_south: f64,
    pub behavior_note: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedSolarGeometryRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub solar_zenith_rad: f64,
    pub solar_elevation_rad: f64,
    pub solar_azimuth_north_rad: f64,
    pub direct_horizontal_irradiance_w_m2: f64,

    /// Dimensionless roof-glass transmission after multiple optical paths.
    pub tau_glass: f64,

    /// Dimensionless fraction passing ridges and gutters (with shadow overlap).
    pub tau_ridges_gutters: f64,

    /// Dimensionless fraction passing glazing bars.
    pub tau_bars: f64,

    /// Product tau_glass * tau_ridges_gutters * tau_bars.
    pub tau_roof_direct: f64,

    /// Transmitted direct irradiance per horizontal area, W/m² of floor.
    pub transmitted_direct_horizontal_w_m2: f64,

    /// 0: suppressed near/below horizon; 1–4: reference Bot/TiGwerLib optical cases.
    pub optical_case: u8,

    /// Effective ray elevation in the cross-span plane, radians.
    pub cross_span_elevation_rad: f64,

    pub area_basis: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareSolarGeometryOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub roof: VenloRoofParametersOutput,
    pub glass: UncoatedGlassParametersOutput,
    pub published_orientation_findings: [OrientationSensitivityFindingOutput; 3],
    pub records: Vec<PreparedSolarGeometryRecord>,
}

fn reference_records() -> Vec<PreparedSolarGeometryRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — Optical Case 0 (below horizon)
        PreparedSolarGeometryRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            solar_zenith_rad: 2.530_727_415_392,
            solar_elevation_rad: -0.959_931_088_597,
            solar_azimuth_north_rad: 0.261_799_387_799,
            direct_horizontal_irradiance_w_m2: 0.0,
            tau_glass: 0.0,
            tau_ridges_gutters: 0.0,
            tau_bars: 0.0,
            tau_roof_direct: 0.0,
            transmitted_direct_horizontal_w_m2: 0.0,
            optical_case: 0,
            cross_span_elevation_rad: 0.01,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — Optical Case 2
        PreparedSolarGeometryRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            solar_zenith_rad: 1.204_277_183_876,
            solar_elevation_rad: 0.366_519_142_919,
            solar_azimuth_north_rad: std::f64::consts::PI,
            direct_horizontal_irradiance_w_m2: 90.0,
            tau_glass: 0.801_099_580_653,
            tau_ridges_gutters: 0.903_758_818_592,
            tau_bars: 0.895_940_713_917,
            tau_roof_direct: 0.648_661_803_113,
            transmitted_direct_horizontal_w_m2: 58.379_562_280_201,
            optical_case: 2,
            cross_span_elevation_rad: 0.669_710_479_676,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC) — Optical Case 3
        PreparedSolarGeometryRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            solar_zenith_rad: 0.829_031_394_697,
            solar_elevation_rad: 0.741_764_932_098,
            solar_azimuth_north_rad: std::f64::consts::PI,
            direct_horizontal_irradiance_w_m2: 390.0,
            tau_glass: 0.861_141_385_697,
            tau_ridges_gutters: 0.938_618_849_151,
            tau_bars: 0.946_938_551_168,
            tau_roof_direct: 0.765_394_840_891,
            transmitted_direct_horizontal_w_m2: 298.503_987_947_333,
            optical_case: 3,
            cross_span_elevation_rad: 1.084_158_718_959,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC) — Optical Case 4
        PreparedSolarGeometryRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            solar_zenith_rad: 0.462_512_251_778,
            solar_elevation_rad: 1.108_284_075_016,
            solar_azimuth_north_rad: 200.0_f64.to_radians(),
            direct_horizontal_irradiance_w_m2: 600.0,
            tau_glass: 0.892_052_727_864,
            tau_ridges_gutters: 0.945_876_494_984,
            tau_bars: 0.971_104_336_138,
            tau_roof_direct: 0.819_390_363_934,
            transmitted_direct_horizontal_w_m2: 491.634_218_360_600,
            optical_case: 4,
            cross_span_elevation_rad: 1.210_900_156_455,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
    ]
}

pub fn prepare_solar_geometry_service(
    payload: PrepareSolarGeometryInput,
) -> Result<PrepareSolarGeometryOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SOLAR_GEOMETRY_PRESET_ID);

    if preset != DEFAULT_SOLAR_GEOMETRY_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation solar geometry preset '{preset}'; \
             supported preset is '{DEFAULT_SOLAR_GEOMETRY_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedSolarGeometryRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedSolarGeometryRecord> = all_records
            .into_iter()
            .filter(|record| record.period_label == filter)
            .collect();

        if filtered.is_empty() {
            return Err(AppError::BadRequest(format!(
                "unknown period_filter '{filter}'; supported values are \
                 'all', 'winter_night', 'winter_noon', 'equinox_noon', \
                 or 'summer_afternoon'"
            )));
        }

        filtered
    };

    Ok(PrepareSolarGeometryOutput {
        preset_id: DEFAULT_SOLAR_GEOMETRY_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.2.1, Section 4.1 (Fig. 6) & TiGwerLib 0.7.0 VenloRoof_base",
        roof: VenloRoofParametersOutput {
            ridge_azimuth_deg: 331.0,
            ridge_azimuth_rad: 331.0_f64.to_radians(),
            roof_slope_deg: 22.0,
            roof_slope_rad: 0.383_972_435_438_75,
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
        glass: UncoatedGlassParametersOutput {
            thickness_m: 0.004,
            normal_transmittance: 0.90,
            normal_reflectance: 0.08,
        },
        published_orientation_findings: [
            OrientationSensitivityFindingOutput {
                date_iso: "2014-12-21",
                max_abs_tau_direct_diff_vs_north_south: 0.19,
                behavior_note:
                "-29° ridge orientation reduces morning direct transmission and increases afternoon transmission vs pure N-S (winter solstice)",
            },
            OrientationSensitivityFindingOutput {
                date_iso: "2015-02-21",
                max_abs_tau_direct_diff_vs_north_south: 0.11,
                behavior_note:
                "Late-winter morning/afternoon asymmetry from -29° ridge orientation (Fig. 6)",
            },
            OrientationSensitivityFindingOutput {
                date_iso: "2015-03-20",
                max_abs_tau_direct_diff_vs_north_south: 0.07,
                behavior_note:
                "Spring equinox orientation difference vs pure N-S roof (Fig. 6)",
            },
        ],
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_all_reference_solar_geometry_records_matching_governing_product() {
        let output = prepare_solar_geometry_service(
            PrepareSolarGeometryInput::default(),
        )
            .expect("default solar geometry preset should succeed");

        assert_eq!(output.preset_id, DEFAULT_SOLAR_GEOMETRY_PRESET_ID);
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            let expected_tau_direct = record.tau_glass
                * record.tau_ridges_gutters
                * record.tau_bars;
            assert!(
                (record.tau_roof_direct - expected_tau_direct).abs() < 1e-11
            );

            let expected_transmitted = record.tau_roof_direct
                * record.direct_horizontal_irradiance_w_m2;
            assert!(
                (record.transmitted_direct_horizontal_w_m2
                    - expected_transmitted)
                    .abs()
                    < 1e-9
            );
        }
    }

    #[test]
    fn filters_single_period_record() {
        let output = prepare_solar_geometry_service(
            PrepareSolarGeometryInput {
                preset_id: None,
                period_filter: Some("equinox_noon".into()),
            },
        )
            .expect("equinox_noon filter should succeed");

        assert_eq!(output.records.len(), 1);
        assert_eq!(output.records[0].optical_case, 3);
        assert!(
            (output.records[0].tau_roof_direct - 0.765_394_840_891).abs()
                < 1e-11
        );
    }

    #[test]
    fn rejects_unknown_preset_or_filter() {
        assert!(matches!(
            prepare_solar_geometry_service(PrepareSolarGeometryInput {
                preset_id: Some("bad_preset".into()),
                period_filter: None,
            }),
            Err(AppError::BadRequest(_))
        ));

        assert!(matches!(
            prepare_solar_geometry_service(PrepareSolarGeometryInput {
                preset_id: None,
                period_filter: Some("bad_period".into()),
            }),
            Err(AppError::BadRequest(_))
        ));
    }
}