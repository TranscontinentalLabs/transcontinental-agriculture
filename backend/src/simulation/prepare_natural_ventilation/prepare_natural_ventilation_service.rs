use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_NATURAL_VENTILATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const GRAVITY_M_S2: f64 = 9.81;
pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const TOTAL_VOLUME_M3: f64 = 7_646.4;
pub const TOP_AIR_VOLUME_M3: f64 = 419.904;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareNaturalVentilationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–7 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct RoofVentConfigurationOutput {
    pub correlation_source: &'static str,
    pub discharge_coefficient_cf: f64,
    pub vent_length_m: f64,
    pub vent_height_m: f64,
    pub roof_slope_deg: f64,
    pub roof_slope_rad: f64,
    pub max_opening_angle_deg: f64,
    pub closed_crack_virtual_height_m: f64,
    pub closed_crack_equivalent_angle_deg: f64,
    pub leeward_vent_count_total: u16,
    pub windward_vent_count_total: u16,
    pub leeward_vents_per_floor_m2: f64,
    pub windward_vents_per_floor_m2: f64,
    pub mixing_zone_target: &'static str,
    pub published_water_loss_share_via_vents_pct: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct VentSideCalculationOutput {
    pub commanded_angle_deg: f64,
    pub effective_angle_deg: f64,
    pub effective_opening_height_m: f64,
    pub wind_function_g: f64,
    pub single_vent_buoyancy_flow_m3_s: f64,
    pub single_vent_wind_flow_m3_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedNaturalVentilationRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub roof_wind_speed_m_s: f64,
    pub top_air_density_kg_m3: f64,
    pub outdoor_air_density_kg_m3: f64,
    pub reference_mean_density_kg_m3: f64,

    pub leeward: VentSideCalculationOutput,
    pub windward: VentSideCalculationOutput,

    /// Combined buoyancy and wind terms per m² of greenhouse floor, m³/(s·m²).
    pub buoyancy_rate_per_floor_m2_s: f64,
    pub wind_rate_per_floor_m2_s: f64,
    pub combined_vent_rate_per_floor_m2_s: f64,

    /// Whole-compartment volumetric airflow F_whole = A_floor * f_vents, m³/s.
    pub whole_greenhouse_airflow_m3_s: f64,

    /// Air changes per hour on whole-greenhouse volume (7646.4 m³)
    /// and TopAir volume (419.904 m³).
    pub ach_whole_greenhouse_per_h: f64,
    pub ach_top_air_zone_per_h: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareNaturalVentilationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub configuration: RoofVentConfigurationOutput,
    pub records: Vec<PreparedNaturalVentilationRecord>,
}

fn reference_records() -> Vec<PreparedNaturalVentilationRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — vents commanded closed (0°), 6 mm virtual crack active
        PreparedNaturalVentilationRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            roof_wind_speed_m_s: 3.2,
            top_air_density_kg_m3: 1.240_907_556_900,
            outdoor_air_density_kg_m3: 1.273_791_541_755,
            reference_mean_density_kg_m3: 1.257_349_549_328,
            leeward: VentSideCalculationOutput {
                commanded_angle_deg: 0.0,
                effective_angle_deg: 0.322_047_266_943,
                effective_opening_height_m: 0.006,
                wind_function_g: 0.000_540_347_038,
                single_vent_buoyancy_flow_m3_s: 0.000_132_614_507,
                single_vent_wind_flow_m3_s: 0.005_170_040_457,
            },
            windward: VentSideCalculationOutput {
                commanded_angle_deg: 0.0,
                effective_angle_deg: 0.322_047_266_943,
                effective_opening_height_m: 0.006,
                wind_function_g: 0.000_716_205_202,
                single_vent_buoyancy_flow_m3_s: 0.000_132_614_507,
                single_vent_wind_flow_m3_s: 0.006_852_651_376,
            },
            buoyancy_rate_per_floor_m2_s: 0.000_009_209_341,
            wind_rate_per_floor_m2_s: 0.000_417_454_578,
            combined_vent_rate_per_floor_m2_s: 0.000_417_556_147,
            whole_greenhouse_airflow_m3_s: 0.432_922_213_651,
            ach_whole_greenhouse_per_h: 0.203_824_017_726,
            ach_top_air_zone_per_h: 3.711_610_199_340,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — leeward vents slightly opened (4°), windward closed
        PreparedNaturalVentilationRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            roof_wind_speed_m_s: 4.1,
            top_air_density_kg_m3: 1.206_101_282_000,
            outdoor_air_density_kg_m3: 1.251_303_462_769,
            reference_mean_density_kg_m3: 1.228_702_372_384,
            leeward: VentSideCalculationOutput {
                commanded_angle_deg: 4.0,
                effective_angle_deg: 4.0,
                effective_opening_height_m: 0.075_428_038_897,
                wind_function_g: 0.005_930_626_995,
                single_vent_buoyancy_flow_m3_s: 0.007_010_605_409,
                single_vent_wind_flow_m3_s: 0.072_703_556_331,
            },
            windward: VentSideCalculationOutput {
                commanded_angle_deg: 0.0,
                effective_angle_deg: 0.322_047_266_943,
                effective_opening_height_m: 0.006,
                wind_function_g: 0.000_716_205_202,
                single_vent_buoyancy_flow_m3_s: 0.000_157_283_504,
                single_vent_wind_flow_m3_s: 0.008_779_959_575,
            },
            buoyancy_rate_per_floor_m2_s: 0.000_248_885_032,
            wind_rate_per_floor_m2_s: 0.002_829_288_747,
            combined_vent_rate_per_floor_m2_s: 0.002_840_214_529,
            whole_greenhouse_airflow_m3_s: 2.944_734_423_792,
            ach_whole_greenhouse_per_h: 1.386_409_804_045,
            ach_top_air_zone_per_h: 25.246_351_369_962,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC) — leeward 18°, windward 6°
        PreparedNaturalVentilationRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            roof_wind_speed_m_s: 3.5,
            top_air_density_kg_m3: 1.186_101_008_200,
            outdoor_air_density_kg_m3: 1.223_060_754_962,
            reference_mean_density_kg_m3: 1.204_580_881_581,
            leeward: VentSideCalculationOutput {
                commanded_angle_deg: 18.0,
                effective_angle_deg: 18.0,
                effective_opening_height_m: 0.350_577_637_623,
                wind_function_g: 0.017_490_960_576,
                single_vent_buoyancy_flow_m3_s: 0.064_153_858_118,
                single_vent_wind_flow_m3_s: 0.183_042_902_427,
            },
            windward: VentSideCalculationOutput {
                commanded_angle_deg: 6.0,
                effective_angle_deg: 6.0,
                effective_opening_height_m: 0.113_814_623_239,
                wind_function_g: 0.012_699_600_000,
                single_vent_buoyancy_flow_m3_s: 0.011_867_071_571,
                single_vent_wind_flow_m3_s: 0.132_901_314_000,
            },
            buoyancy_rate_per_floor_m2_s: 0.002_639_615_614,
            wind_rate_per_floor_m2_s: 0.010_970_285_293,
            combined_vent_rate_per_floor_m2_s: 0.011_283_382_914,
            whole_greenhouse_airflow_m3_s: 11.698_611_405_590,
            ach_whole_greenhouse_per_h: 5.507_820_812_425,
            ach_top_air_zone_per_h: 100.296_737_016_379,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC) — both leeward (42°) and windward (35°) wide open
        PreparedNaturalVentilationRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            roof_wind_speed_m_s: 2.6,
            top_air_density_kg_m3: 1.134_247_491_100,
            outdoor_air_density_kg_m3: 1.162_473_831_619,
            reference_mean_density_kg_m3: 1.148_360_661_359,
            leeward: VentSideCalculationOutput {
                commanded_angle_deg: 42.0,
                effective_angle_deg: 42.0,
                effective_opening_height_m: 0.824_120_747_253,
                wind_function_g: 0.023_241_753_243,
                single_vent_buoyancy_flow_m3_s: 0.206_954_254_987,
                single_vent_wind_flow_m3_s: 0.180_681_389_711,
            },
            windward: VentSideCalculationOutput {
                commanded_angle_deg: 35.0,
                effective_angle_deg: 35.0,
                effective_opening_height_m: 0.689_491_294_924,
                wind_function_g: 0.054_897_500_000,
                single_vent_buoyancy_flow_m3_s: 0.158_373_045_993,
                single_vent_wind_flow_m3_s: 0.426_773_165_000,
            },
            buoyancy_rate_per_floor_m2_s: 0.012_684_975_728,
            wind_rate_per_floor_m2_s: 0.021_092_172_039,
            combined_vent_rate_per_floor_m2_s: 0.024_612_767_633,
            whole_greenhouse_airflow_m3_s: 25.518_517_481_493,
            ach_whole_greenhouse_per_h: 12.014_367_929_140,
            ach_top_air_zone_per_h: 218.780_156_734_340,
        },
    ]
}

pub fn prepare_natural_ventilation_service(
    payload: PrepareNaturalVentilationInput,
) -> Result<PrepareNaturalVentilationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_NATURAL_VENTILATION_PRESET_ID);

    if preset != DEFAULT_NATURAL_VENTILATION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation natural ventilation preset '{preset}'; \
             supported preset is '{DEFAULT_NATURAL_VENTILATION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedNaturalVentilationRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedNaturalVentilationRecord> = all_records
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

    Ok(PrepareNaturalVentilationOutput {
        preset_id: DEFAULT_NATURAL_VENTILATION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.4.1 (Eqs. 6-10), Section 3.4.2, Section 4.4.1 (Fig. 13) & De Jong (1990)",
        configuration: RoofVentConfigurationOutput {
            correlation_source: "De Jong (1990) alternate Venlo roof vents (Eqs. 6-10)",
            discharge_coefficient_cf: 0.65,
            vent_length_m: 2.60,
            vent_height_m: 1.15,
            roof_slope_deg: 22.0,
            roof_slope_rad: 0.383_972_435_438_75,
            max_opening_angle_deg: 44.0,
            closed_crack_virtual_height_m: 0.006,
            closed_crack_equivalent_angle_deg: 0.322_047_266_943,
            leeward_vent_count_total: 36,
            windward_vent_count_total: 36,
            leeward_vents_per_floor_m2: 36.0 / FLOOR_AREA_M2,
            windward_vents_per_floor_m2: 36.0 / FLOOR_AREA_M2,
            mixing_zone_target:
            "TopAir (airflow mixes with TopAir above screens regardless of screen deployment, Section 3.4.1)",
            published_water_loss_share_via_vents_pct: 83.3,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equations_6_to_10_and_ach_identities_hold_across_all_records() {
        let output = prepare_natural_ventilation_service(
            PrepareNaturalVentilationInput::default(),
        )
            .expect("default natural ventilation preset should succeed");

        let cfg = &output.configuration;
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for (side, is_leeward) in
                [(&record.leeward, true), (&record.windward, false)]
            {
                let delta_rad = side.effective_angle_deg.to_radians();
                let expected_ho = cfg.vent_height_m
                    * (cfg.roof_slope_rad.sin()
                    - (cfg.roof_slope_rad - delta_rad).sin());
                assert!(
                    (side.effective_opening_height_m - expected_ho).abs()
                        < 1e-11
                );

                let expected_g = if is_leeward {
                    0.0246 * (1.0 - (-side.effective_angle_deg / 14.5).exp())
                } else {
                    -1.89e-5 * side.effective_angle_deg.powi(2)
                        + 2.23e-3 * side.effective_angle_deg
                };
                assert!((side.wind_function_g - expected_g).abs() < 1e-11);

                let expected_f_buoy = (cfg.discharge_coefficient_cf
                    * cfg.vent_length_m
                    * expected_ho.powf(1.5)
                    / 3.0)
                    * (GRAVITY_M_S2
                    * (record.top_air_density_kg_m3
                    - record.outdoor_air_density_kg_m3)
                    .abs()
                    / record.reference_mean_density_kg_m3)
                    .sqrt();
                assert!(
                    (side.single_vent_buoyancy_flow_m3_s - expected_f_buoy)
                        .abs()
                        < 1e-11
                );

                let expected_f_wind = cfg.vent_length_m
                    * cfg.vent_height_m
                    * expected_g
                    * record.roof_wind_speed_m_s;
                assert!(
                    (side.single_vent_wind_flow_m3_s - expected_f_wind).abs()
                        < 1e-11
                );
            }

            let expected_f_vents = record
                .buoyancy_rate_per_floor_m2_s
                .hypot(record.wind_rate_per_floor_m2_s);
            assert!(
                (record.combined_vent_rate_per_floor_m2_s - expected_f_vents)
                    .abs()
                    < 1e-11
            );

            let expected_whole =
                FLOOR_AREA_M2 * record.combined_vent_rate_per_floor_m2_s;
            assert!(
                (record.whole_greenhouse_airflow_m3_s - expected_whole).abs()
                    < 1e-9
            );

            let expected_ach = 3_600.0 * expected_whole / TOTAL_VOLUME_M3;
            assert!(
                (record.ach_whole_greenhouse_per_h - expected_ach).abs() < 1e-9
            );
        }
    }
}