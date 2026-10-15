use serde::{Deserialize, Serialize};

use crate::error::AppError;

use super::prepare_solar_geometry_service::DEFAULT_SOLAR_GEOMETRY_PRESET_ID;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareDiffuseSolarGeometryInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3 and 4 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedDiffuseDistributionSummaryOutput {
    pub season_min_tau_diffuse: f64,
    pub season_max_tau_diffuse: f64,
    pub mode_threshold_tau_diffuse: f64,
    pub standard_overcast_ctifl_tau_diffuse: f64,
    pub bot_1983_reference_tau_diffuse: f64,
    pub de_zwart_1996_reference_tau_diffuse: f64,
    pub uniform_sky_tau_diffuse: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedDiffuseSolarRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub solar_zenith_rad: f64,
    pub solar_azimuth_south_rad: f64,
    pub global_horizontal_irradiance_w_m2: f64,
    pub diffuse_horizontal_irradiance_w_m2: f64,

    /// Reference definition: GHI - DHI, W/m².
    pub direct_horizontal_irradiance_w_m2: f64,

    /// Cloud ratio Ce = DHI / GHI (None at night when GHI == 0).
    pub cloud_ratio_ce: Option<f64>,

    pub tau_roof_diffuse: f64,
    pub tau_roof_direct: f64,

    /// All three irradiances below are per m² of horizontal greenhouse floor area.
    pub transmitted_direct_horizontal_w_m2: f64,
    pub transmitted_diffuse_horizontal_w_m2: f64,
    pub transmitted_total_horizontal_w_m2: f64,

    /// Effective combined downward roof transmittance I_roof,in / GHI
    /// for feeding Layer 1 of Model 6 (0.0 at night).
    pub effective_total_roof_transmittance: f64,

    /// Diagnostics for the 5-degree Igawa sky integration.
    pub integration_numerator: f64,
    pub integration_denominator: f64,

    pub area_basis: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareDiffuseSolarGeometryOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub sky_model: &'static str,
    pub directional_grid_source: &'static str,
    pub site_altitude_m: f64,
    pub azimuth_step_deg: u8,
    pub zenith_step_deg: u8,
    pub published_season_summary: PublishedDiffuseDistributionSummaryOutput,
    pub records: Vec<PreparedDiffuseSolarRecord>,
}

fn reference_records() -> Vec<PreparedDiffuseSolarRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — suppressed to 0 below horizon
        PreparedDiffuseSolarRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            solar_zenith_rad: 2.530_727_415_392,
            solar_azimuth_south_rad: -2.879_793_265_791,
            global_horizontal_irradiance_w_m2: 0.0,
            diffuse_horizontal_irradiance_w_m2: 0.0,
            direct_horizontal_irradiance_w_m2: 0.0,
            cloud_ratio_ce: None,
            tau_roof_diffuse: 0.0,
            tau_roof_direct: 0.0,
            transmitted_direct_horizontal_w_m2: 0.0,
            transmitted_diffuse_horizontal_w_m2: 0.0,
            transmitted_total_horizontal_w_m2: 0.0,
            effective_total_roof_transmittance: 0.0,
            integration_numerator: 2.498_086_778_060,
            integration_denominator: 3.352_146_481_602,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — high diffuse share (Ce = 0.625)
        PreparedDiffuseSolarRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            solar_zenith_rad: 1.204_277_183_876,
            solar_azimuth_south_rad: 0.0,
            global_horizontal_irradiance_w_m2: 240.0,
            diffuse_horizontal_irradiance_w_m2: 150.0,
            direct_horizontal_irradiance_w_m2: 90.0,
            cloud_ratio_ce: Some(0.625),
            tau_roof_diffuse: 0.715_703_487_943,
            tau_roof_direct: 0.648_661_803_113,
            transmitted_direct_horizontal_w_m2: 58.379_562_280_170,
            transmitted_diffuse_horizontal_w_m2: 107.355_523_191_521,
            transmitted_total_horizontal_w_m2: 165.735_085_471_691,
            effective_total_roof_transmittance: 0.690_562_856_132,
            integration_numerator: 4.163_536_591_041,
            integration_denominator: 5.816_404_359_737,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC) — mixed sky (Ce = 0.328)
        PreparedDiffuseSolarRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            solar_zenith_rad: 0.829_031_394_697,
            solar_azimuth_south_rad: 0.0,
            global_horizontal_irradiance_w_m2: 580.0,
            diffuse_horizontal_irradiance_w_m2: 190.0,
            direct_horizontal_irradiance_w_m2: 390.0,
            cloud_ratio_ce: Some(190.0 / 580.0),
            tau_roof_diffuse: 0.740_590_222_187,
            tau_roof_direct: 0.765_394_840_891,
            transmitted_direct_horizontal_w_m2: 298.503_987_947_490,
            transmitted_diffuse_horizontal_w_m2: 140.712_142_215_466,
            transmitted_total_horizontal_w_m2: 439.216_130_162_956,
            effective_total_roof_transmittance: 0.757_269_189_936,
            integration_numerator: 3.715_212_546_645,
            integration_denominator: 5.015_556_302_452,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC) — clear-sky dominant (Ce = 0.268)
        PreparedDiffuseSolarRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            solar_zenith_rad: 0.462_512_251_778,
            solar_azimuth_south_rad: 20.0_f64.to_radians(),
            global_horizontal_irradiance_w_m2: 820.0,
            diffuse_horizontal_irradiance_w_m2: 220.0,
            direct_horizontal_irradiance_w_m2: 600.0,
            cloud_ratio_ce: Some(220.0 / 820.0),
            tau_roof_diffuse: 0.767_619_980_424,
            tau_roof_direct: 0.819_390_363_934,
            transmitted_direct_horizontal_w_m2: 491.634_218_360_400,
            transmitted_diffuse_horizontal_w_m2: 168.876_395_693_351,
            transmitted_total_horizontal_w_m2: 660.510_614_053_751,
            effective_total_roof_transmittance: 0.805_500_748_846,
            integration_numerator: 1.949_953_036_663,
            integration_denominator: 2.539_258_313_215,
            area_basis: "horizontal_greenhouse_floor_m2",
        },
    ]
}

pub fn prepare_diffuse_solar_geometry_service(
    payload: PrepareDiffuseSolarGeometryInput,
) -> Result<PrepareDiffuseSolarGeometryOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SOLAR_GEOMETRY_PRESET_ID);

    if preset != DEFAULT_SOLAR_GEOMETRY_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation diffuse solar preset '{preset}'; \
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

    let records: Vec<PreparedDiffuseSolarRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedDiffuseSolarRecord> = all_records
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

    Ok(PrepareDiffuseSolarGeometryOutput {
        preset_id: DEFAULT_SOLAR_GEOMETRY_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.2.2, Section 4.1 (Fig. 5 & Fig. 7) & TiGwerLib 0.7.0 TauDiff",
        sky_model: "igawa_all_sky_model_r_reference",
        directional_grid_source:
        "CTIFL Venlo 5-degree directional transmission grid (73 azimuth x 19 zenith)",
        site_altitude_m: 30.0,
        azimuth_step_deg: 5,
        zenith_step_deg: 5,
        published_season_summary: PublishedDiffuseDistributionSummaryOutput {
            season_min_tau_diffuse: 0.67,
            season_max_tau_diffuse: 0.77,
            mode_threshold_tau_diffuse: 0.76,
            standard_overcast_ctifl_tau_diffuse: 0.77,
            bot_1983_reference_tau_diffuse: 0.72,
            de_zwart_1996_reference_tau_diffuse: 0.79,
            uniform_sky_tau_diffuse: 0.750_731_435_433,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_reference_diffuse_records_matching_irradiance_identity() {
        let output = prepare_diffuse_solar_geometry_service(
            PrepareDiffuseSolarGeometryInput::default(),
        )
            .expect("default diffuse solar preset should succeed");

        assert_eq!(output.preset_id, DEFAULT_SOLAR_GEOMETRY_PRESET_ID);
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            let expected_direct_hor = record.global_horizontal_irradiance_w_m2
                - record.diffuse_horizontal_irradiance_w_m2;
            assert!(
                (record.direct_horizontal_irradiance_w_m2 - expected_direct_hor)
                    .abs()
                    < 1e-9
            );

            let expected_trans_direct = record.tau_roof_direct
                * record.direct_horizontal_irradiance_w_m2;
            let expected_trans_diffuse = record.tau_roof_diffuse
                * record.diffuse_horizontal_irradiance_w_m2;
            let expected_trans_total =
                expected_trans_direct + expected_trans_diffuse;

            assert!(
                (record.transmitted_direct_horizontal_w_m2
                    - expected_trans_direct)
                    .abs()
                    < 1e-9
            );
            assert!(
                (record.transmitted_diffuse_horizontal_w_m2
                    - expected_trans_diffuse)
                    .abs()
                    < 1e-9
            );
            assert!(
                (record.transmitted_total_horizontal_w_m2
                    - expected_trans_total)
                    .abs()
                    < 1e-9
            );

            // Daytime diffuse transmittance lies within the published [0.67, 0.77] season range.
            if record.global_horizontal_irradiance_w_m2 > 0.0 {
                assert!(
                    (0.67..=0.77).contains(&record.tau_roof_diffuse),
                    "daytime tau_roof_diffuse {} outside published [0.67, 0.77] range",
                    record.tau_roof_diffuse
                );
            }
        }
    }

    #[test]
    fn filters_single_diffuse_period_record() {
        let output = prepare_diffuse_solar_geometry_service(
            PrepareDiffuseSolarGeometryInput {
                preset_id: None,
                period_filter: Some("winter_noon".into()),
            },
        )
            .expect("winter_noon filter should succeed");

        assert_eq!(output.records.len(), 1);
        assert_eq!(output.records[0].period_label, "winter_noon");
        assert!(
            (output.records[0].tau_roof_diffuse - 0.715_703_487_943).abs()
                < 1e-11
        );
    }
}