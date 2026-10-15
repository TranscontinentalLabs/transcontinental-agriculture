use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_AIR_VOLUME_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// Universal gas constant, J/(mol·K).
pub const UNIVERSAL_GAS_CONSTANT_J_MOL_K: f64 = 8.314_462_618;

/// Molar masses, kg/mol.
pub const DRY_AIR_MOLAR_MASS_KG_MOL: f64 = 0.028_965_4;
pub const WATER_MOLAR_MASS_KG_MOL: f64 = 0.018_015_28;
pub const CO2_MOLAR_MASS_KG_MOL: f64 = 0.044_009_5;

/// Zone volumes derived from Model 2 reference dimensions:
/// MainAir = 1036.8 m² * 6.97 m gutter height = 7226.496 m³.
/// TopAir  = 1036.8 m² * (7.78 - 6.97)/2 m   = 419.904 m³.
pub const MAIN_AIR_VOLUME_M3: f64 = 7_226.496;
pub const TOP_AIR_VOLUME_M3: f64 = 419.904;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareAirVolumeInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–6 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AirZoneGeometryOutput {
    pub total_enclosed_volume_m3: f64,
    pub main_air_volume_m3: f64,
    pub top_air_volume_m3: f64,
    pub screen_separation_height_m: f64,
    pub process_sensor_height_m: f64,
    pub vertical_gradient_sensor_heights_m: [f64; 4],
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedClimateValidationSummaryOutput {
    pub evaluation_days: u16,
    pub sampling_interval_s: u32,
    pub main_air_temperature_rmse_c: f64,
    pub main_air_relative_humidity_rmse_pct_points: f64,
    pub days_with_temperature_rmse_below_1_6c_pct: f64,
    pub days_with_rh_rmse_below_10pct_pct: f64,
    pub winter_timestamps_within_5pct_rh_pct: f64,
    pub vertical_gradient_below_0_5c_pct: f64,
    pub vertical_gradient_below_1_0c_pct: f64,
    pub vertical_gradient_below_2_0c_pct: f64,
    pub days_exceeding_4_0c_vertical_gradient: u8,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AirZoneStateOutput {
    pub volume_m3: f64,
    pub temperature_c: f64,
    pub temperature_k: f64,

    /// Relative humidity as a fraction (0–1) and as a percentage (0–100%).
    pub relative_humidity_fraction: f64,
    pub relative_humidity_pct: f64,

    /// Total and partial pressures, Pa (p ~= p_dryair + p_vapour).
    pub total_pressure_pa: f64,
    pub dry_air_partial_pressure_pa: f64,
    pub vapour_partial_pressure_pa: f64,
    pub saturation_vapour_pressure_pa: f64,
    pub co2_partial_pressure_pa: f64,
    pub co2_ppm: f64,

    /// Humidity ratio x_w = M_w / M_dryair, kg water vapour per kg dry air.
    pub humidity_ratio_kg_w_per_kg_da: f64,

    /// Stored gas masses inside the fixed zone volume, kg.
    pub dry_air_mass_kg: f64,
    pub water_vapour_mass_kg: f64,
    pub co2_mass_kg: f64,

    /// Moist air density (M_dryair + M_w) / V, kg/m³.
    pub moist_air_density_kg_m3: f64,

    /// Moist air specific enthalpy per kg dry air (J/kg_da, 0 °C reference),
    /// total enthalpy H = M_dryair * h (J), and internal energy U = H - p*V (J).
    pub specific_enthalpy_j_per_kg_da: f64,
    pub total_enthalpy_j: f64,
    pub internal_energy_j: f64,

    /// Air-to-air vapour pressure deficit: p_sat(T_air) - p_v, Pa.
    pub vpd_air_pa: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedAirVolumeRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Section 27 API fields alongside full zone states.
    pub main_air: AirZoneStateOutput,
    pub top_air: AirZoneStateOutput,

    /// Canopy leaf temperature (Section 27: `crop.leaf_temperature_c`) used
    /// to distinguish VPD_leaf-air from VPD_air.
    pub leaf_temperature_c: f64,
    pub leaf_temperature_k: f64,
    pub leaf_saturation_vapour_pressure_pa: f64,

    /// VPD_leaf-air = p_sat(T_leaf) - p_v,MainAir, Pa.
    pub vpd_leaf_air_pa: f64,

    /// Temperature difference T_MainAir - T_TopAir, °C.
    pub main_minus_top_temperature_c: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareAirVolumeOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub zone_geometry: AirZoneGeometryOutput,
    pub published_validation_summary: PublishedClimateValidationSummaryOutput,
    pub records: Vec<PreparedAirVolumeRecord>,
}

fn reference_records() -> Vec<PreparedAirVolumeRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC, Fig. 8 window: thermal screen closed, MainAir ~17.2 °C, TopAir ~11.4 °C)
        PreparedAirVolumeRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            main_air: AirZoneStateOutput {
                volume_m3: MAIN_AIR_VOLUME_M3,
                temperature_c: 17.2,
                temperature_k: 290.35,
                relative_humidity_fraction: 0.78,
                relative_humidity_pct: 78.0,
                total_pressure_pa: 101_798.0,
                dry_air_partial_pressure_pa: 100_267.358_107_979_5,
                vapour_partial_pressure_pa: 1_530.641_892_020_5,
                saturation_vapour_pressure_pa: 1_962.361_400_026_2,
                co2_partial_pressure_pa: 44.791_12,
                co2_ppm: 440.0,
                humidity_ratio_kg_w_per_kg_da: 0.009_494_574_6,
                dry_air_mass_kg: 8_693.824_192_579_5,
                water_vapour_mass_kg: 82.544_162_024_9,
                co2_mass_kg: 5.900_789_303_6,
                moist_air_density_kg_m3: 1.214_470_796_7,
                specific_enthalpy_j_per_kg_da: 41_352.881_408_815_6,
                total_enthalpy_j: 359_514_680.824_831_84,
                internal_energy_j: -376_128_158.983_168_1,
                vpd_air_pa: 431.719_508_005_8,
            },
            top_air: AirZoneStateOutput {
                volume_m3: TOP_AIR_VOLUME_M3,
                temperature_c: 11.4,
                temperature_k: 284.55,
                relative_humidity_fraction: 0.86,
                relative_humidity_pct: 86.0,
                total_pressure_pa: 101_795.0,
                dry_air_partial_pressure_pa: 100_635.733_904_984_6,
                vapour_partial_pressure_pa: 1_159.266_095_015_4,
                saturation_vapour_pressure_pa: 1_347.983_831_413_3,
                co2_partial_pressure_pa: 42.244_925,
                co2_ppm: 415.0,
                humidity_ratio_kg_w_per_kg_da: 0.007_164_607_5,
                dry_air_mass_kg: 517.355_398_424_6,
                water_vapour_mass_kg: 3.706_648_358_9,
                co2_mass_kg: 0.329_972_855_2,
                moist_air_density_kg_m3: 1.240_907_556_9,
                specific_enthalpy_j_per_kg_da: 29_539.001_652_079_1,
                total_enthalpy_j: 15_282_161.968_776_757,
                internal_energy_j: -27_461_965.711_223_267,
                vpd_air_pa: 188.717_736_397_9,
            },
            leaf_temperature_c: 16.8,
            leaf_temperature_k: 289.95,
            leaf_saturation_vapour_pressure_pa: 1_913.243_045_337_8,
            vpd_leaf_air_pa: 382.601_153_317_3,
            main_minus_top_temperature_c: 5.8,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC, Fig. 8 window: screens open, CO2 enriched to 680 ppm)
        PreparedAirVolumeRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            main_air: AirZoneStateOutput {
                volume_m3: MAIN_AIR_VOLUME_M3,
                temperature_c: 20.4,
                temperature_k: 293.55,
                relative_humidity_fraction: 0.75,
                relative_humidity_pct: 75.0,
                total_pressure_pa: 101_748.0,
                dry_air_partial_pressure_pa: 99_950.451_052_932_4,
                vapour_partial_pressure_pa: 1_797.548_947_067_6,
                saturation_vapour_pressure_pa: 2_396.731_929_423_4,
                co2_partial_pressure_pa: 69.188_64,
                co2_ppm: 680.0,
                humidity_ratio_kg_w_per_kg_da: 0.011_185_552_8,
                dry_air_mass_kg: 8_571.874_135_484_5,
                water_vapour_mass_kg: 95.881_150_885_2,
                co2_mass_kg: 9.015_560_36,
                moist_air_density_kg_m3: 1.199_440_958_2,
                specific_enthalpy_j_per_kg_da: 48_921.892_212_022_2,
                total_enthalpy_j: 419_352_302.511_194_1,
                internal_energy_j: -315_929_212.496_805_8,
                vpd_air_pa: 599.182_982_355_9,
            },
            top_air: AirZoneStateOutput {
                volume_m3: TOP_AIR_VOLUME_M3,
                temperature_c: 18.9,
                temperature_k: 292.05,
                relative_humidity_fraction: 0.77,
                relative_humidity_pct: 77.0,
                total_pressure_pa: 101_746.0,
                dry_air_partial_pressure_pa: 100_064.589_239_891_6,
                vapour_partial_pressure_pa: 1_681.410_760_108_4,
                saturation_vapour_pressure_pa: 2_183.650_337_803_1,
                co2_partial_pressure_pa: 52.907_92,
                co2_ppm: 520.0,
                humidity_ratio_kg_w_per_kg_da: 0.010_450_928_9,
                dry_air_mass_kg: 501.208_656_696_6,
                water_vapour_mass_kg: 5.238_096_016,
                co2_mass_kg: 0.402_648_178_5,
                moist_air_density_kg_m3: 1.206_101_282,
                specific_enthalpy_j_per_kg_da: 45_518.565_036_241_8,
                total_enthalpy_j: 22_814_298.836_570_792,
                internal_energy_j: -19_909_253.547_429_234,
                vpd_air_pa: 502.239_577_694_7,
            },
            leaf_temperature_c: 21.1,
            leaf_temperature_k: 294.25,
            leaf_saturation_vapour_pressure_pa: 2_502.240_819_576_9,
            vpd_leaf_air_pa: 704.691_872_509_3,
            main_minus_top_temperature_c: 1.5,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC)
        PreparedAirVolumeRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            main_air: AirZoneStateOutput {
                volume_m3: MAIN_AIR_VOLUME_M3,
                temperature_c: 23.6,
                temperature_k: 296.75,
                relative_humidity_fraction: 0.72,
                relative_humidity_pct: 72.0,
                total_pressure_pa: 101_498.0,
                dry_air_partial_pressure_pa: 99_400.692_116_132_4,
                vapour_partial_pressure_pa: 2_097.307_883_867_6,
                saturation_vapour_pressure_pa: 2_912.927_616_482_8,
                co2_partial_pressure_pa: 73.078_56,
                co2_ppm: 720.0,
                humidity_ratio_kg_w_per_kg_da: 0.013_123_034_4,
                dry_air_mass_kg: 8_432.799_849_945_2,
                water_vapour_mass_kg: 110.663_922_503,
                co2_mass_kg: 9.419_747_673_2,
                moist_air_density_kg_m3: 1.182_241_541_7,
                specific_enthalpy_j_per_kg_da: 57_138.357_747_544_7,
                total_enthalpy_j: 481_836_334.639_609_63,
                internal_energy_j: -251_638_556.368_390_26,
                vpd_air_pa: 815.619_732_615_2,
            },
            top_air: AirZoneStateOutput {
                volume_m3: TOP_AIR_VOLUME_M3,
                temperature_c: 22.8,
                temperature_k: 295.95,
                relative_humidity_fraction: 0.70,
                relative_humidity_pct: 70.0,
                total_pressure_pa: 101_496.0,
                dry_air_partial_pressure_pa: 99_553.121_756_125,
                vapour_partial_pressure_pa: 1_942.878_243_875,
                saturation_vapour_pressure_pa: 2_775.540_348_392_8,
                co2_partial_pressure_pa: 54.807_84,
                co2_ppm: 540.0,
                humidity_ratio_kg_w_per_kg_da: 0.012_138_141_3,
                dry_air_mass_kg: 492.075_673_685_3,
                water_vapour_mass_kg: 5.972_884_049_8,
                co2_mass_kg: 0.411_610_650_4,
                moist_air_density_kg_m3: 1.186_101_008_2,
                specific_enthalpy_j_per_kg_da: 53_809.045_648_037_1,
                total_enthalpy_j: 26_478_122.387_618_363,
                internal_energy_j: -16_140_453.996_381_663,
                vpd_air_pa: 832.662_104_517_8,
            },
            leaf_temperature_c: 24.5,
            leaf_temperature_k: 297.65,
            leaf_saturation_vapour_pressure_pa: 3_074.548_374_750_3,
            vpd_leaf_air_pa: 977.240_490_882_7,
            main_minus_top_temperature_c: 0.8,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC, Fig. 9 window: shading screen unfolded, TopAir ~34.6 °C)
        PreparedAirVolumeRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            main_air: AirZoneStateOutput {
                volume_m3: MAIN_AIR_VOLUME_M3,
                temperature_c: 31.2,
                temperature_k: 304.35,
                relative_humidity_fraction: 0.62,
                relative_humidity_pct: 62.0,
                total_pressure_pa: 101_323.0,
                dry_air_partial_pressure_pa: 98_505.812_505_125_4,
                vapour_partial_pressure_pa: 2_817.187_494_874_6,
                saturation_vapour_pressure_pa: 4_543.850_798_184_8,
                co2_partial_pressure_pa: 41.542_43,
                co2_ppm: 410.0,
                humidity_ratio_kg_w_per_kg_da: 0.017_787_519_1,
                dry_air_mass_kg: 8_148.199_680_708_2,
                water_vapour_mass_kg: 144.936_257_831_9,
                co2_mass_kg: 5.221_059_069_5,
                moist_air_density_kg_m3: 1.147_601_263_3,
                specific_enthalpy_j_per_kg_da: 76_906.030_697_085_3,
                total_enthalpy_j: 626_645_694.770_524_5,
                internal_energy_j: -105_564_559.437_475_44,
                vpd_air_pa: 1_726.663_303_310_2,
            },
            top_air: AirZoneStateOutput {
                volume_m3: TOP_AIR_VOLUME_M3,
                temperature_c: 34.6,
                temperature_k: 307.75,
                relative_humidity_fraction: 0.54,
                relative_humidity_pct: 54.0,
                total_pressure_pa: 101_321.0,
                dry_air_partial_pressure_pa: 98_351.335_571_010_2,
                vapour_partial_pressure_pa: 2_969.664_428_989_8,
                saturation_vapour_pressure_pa: 5_499.378_572_203_3,
                co2_partial_pressure_pa: 41.035_005,
                co2_ppm: 405.0,
                humidity_ratio_kg_w_per_kg_da: 0.018_779_697_9,
                dry_air_mass_kg: 467.495_631_764_6,
                water_vapour_mass_kg: 8.779_426_726_1,
                co2_mass_kg: 0.296_359_388_7,
                moist_air_density_kg_m3: 1.134_247_491_1,
                specific_enthalpy_j_per_kg_da: 82_984.210_642_213_5,
                total_enthalpy_j: 38_794_755.980_666_65,
                internal_energy_j: -3_750_337.203_333_370_4,
                vpd_air_pa: 2_529.714_143_213_5,
            },
            leaf_temperature_c: 30.4,
            leaf_temperature_k: 303.55,
            leaf_saturation_vapour_pressure_pa: 4_341.248_483_417_4,
            vpd_leaf_air_pa: 1_524.060_988_542_8,
            main_minus_top_temperature_c: -3.4,
        },
    ]
}

pub fn prepare_air_volume_service(
    payload: PrepareAirVolumeInput,
) -> Result<PrepareAirVolumeOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_AIR_VOLUME_PRESET_ID);

    if preset != DEFAULT_AIR_VOLUME_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation air volume preset '{preset}'; \
             supported preset is '{DEFAULT_AIR_VOLUME_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedAirVolumeRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedAirVolumeRecord> = all_records
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

    Ok(PrepareAirVolumeOutput {
        preset_id: DEFAULT_AIR_VOLUME_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.3.2, Section 3.3, Section 4.2 (Figs. 8-10, Table 3) & TiGwerLib 0.7.0 MainAir/TopAir",
        zone_geometry: AirZoneGeometryOutput {
            total_enclosed_volume_m3: MAIN_AIR_VOLUME_M3 + TOP_AIR_VOLUME_M3,
            main_air_volume_m3: MAIN_AIR_VOLUME_M3,
            top_air_volume_m3: TOP_AIR_VOLUME_M3,
            screen_separation_height_m: 6.97,
            process_sensor_height_m: 3.7,
            vertical_gradient_sensor_heights_m: [0.65, 1.5, 3.7, 4.7],
        },
        published_validation_summary: PublishedClimateValidationSummaryOutput {
            evaluation_days: 332,
            sampling_interval_s: 300,
            main_air_temperature_rmse_c: 1.3,
            main_air_relative_humidity_rmse_pct_points: 8.1,
            days_with_temperature_rmse_below_1_6c_pct: 84.1,
            days_with_rh_rmse_below_10pct_pct: 81.4,
            winter_timestamps_within_5pct_rh_pct: 71.4,
            vertical_gradient_below_0_5c_pct: 35.0,
            vertical_gradient_below_1_0c_pct: 77.5,
            vertical_gradient_below_2_0c_pct: 97.1,
            days_exceeding_4_0c_vertical_gradient: 11,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zone_volumes_and_gas_identities_hold_across_all_records() {
        let output =
            prepare_air_volume_service(PrepareAirVolumeInput::default())
                .expect("default air volume preset should succeed");

        assert!(
            (output.zone_geometry.total_enclosed_volume_m3 - 7_646.4).abs()
                < 1e-9
        );
        assert_eq!(output.records.len(), 4);

        let molar_ratio = WATER_MOLAR_MASS_KG_MOL / DRY_AIR_MOLAR_MASS_KG_MOL;

        for record in &output.records {
            for zone in [&record.main_air, &record.top_air] {
                // Dalton's law: p ~= p_dryair + p_vapour.
                assert!(
                    (zone.total_pressure_pa
                        - (zone.dry_air_partial_pressure_pa
                        + zone.vapour_partial_pressure_pa))
                        .abs()
                        < 1e-6
                );

                // RH = p_v / p_sat(T).
                let expected_rh = zone.vapour_partial_pressure_pa
                    / zone.saturation_vapour_pressure_pa;
                assert!(
                    (zone.relative_humidity_fraction - expected_rh).abs()
                        < 1e-9
                );

                // x_w = M_w / M_dryair = (M_water,molar / M_dryair,molar) * p_v / (p - p_v).
                let expected_xw_from_mass =
                    zone.water_vapour_mass_kg / zone.dry_air_mass_kg;
                let expected_xw_from_pressure = molar_ratio
                    * zone.vapour_partial_pressure_pa
                    / (zone.total_pressure_pa - zone.vapour_partial_pressure_pa);
                assert!(
                    (zone.humidity_ratio_kg_w_per_kg_da - expected_xw_from_mass)
                        .abs()
                        < 1e-9
                );
                assert!(
                    (zone.humidity_ratio_kg_w_per_kg_da
                        - expected_xw_from_pressure)
                        .abs()
                        < 1e-9
                );

                // U = H - p*V and H = M_dryair * h.
                let expected_h =
                    zone.dry_air_mass_kg * zone.specific_enthalpy_j_per_kg_da;
                let expected_u =
                    expected_h - zone.total_pressure_pa * zone.volume_m3;
                assert!((zone.total_enthalpy_j - expected_h).abs() < 1e-3);
                assert!((zone.internal_energy_j - expected_u).abs() < 1e-3);

                // VPD_air = p_sat(T_air) - p_v.
                assert!(
                    (zone.vpd_air_pa
                        - (zone.saturation_vapour_pressure_pa
                        - zone.vapour_partial_pressure_pa))
                        .abs()
                        < 1e-6
                );
            }

            // VPD_leaf-air = p_sat(T_leaf) - p_v,MainAir (distinct from VPD_air).
            let expected_vpd_leaf = record.leaf_saturation_vapour_pressure_pa
                - record.main_air.vapour_partial_pressure_pa;
            assert!(
                (record.vpd_leaf_air_pa - expected_vpd_leaf).abs() < 1e-6
            );
            assert!(
                (record.vpd_leaf_air_pa - record.main_air.vpd_air_pa).abs()
                    > 1.0
            );
        }
    }
}