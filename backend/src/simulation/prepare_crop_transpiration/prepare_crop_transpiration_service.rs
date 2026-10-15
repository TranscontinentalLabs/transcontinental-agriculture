use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CROP_TRANSPIRATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const CULTIVATED_AREA_M2: f64 = 943.488;
pub const CP_AIR_J_KG_K: f64 = 1_006.0;
pub const LATENT_HEAT_VAPORIZATION_J_KG: f64 = 2_450_000.0;

/// Reference psychrometric constant gamma (Pa/K) near sea level.
/// Must be reviewed for higher-altitude sites such as Witeithie (~1,520 m).
pub const REFERENCE_PSYCHROMETRIC_CONSTANT_PA_K: f64 = 66.5;

pub const LEWIS_NUMBER: f64 = 0.89;
pub const STANGHELLINI_R_MIN_S_M: f64 = 82.0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareCropTranspirationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–17 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TranspirationModelParametersOutput {
    pub cultivated_area_a_crop_m2: f64,
    pub floor_area_m2: f64,
    pub cp_air_j_kg_k: f64,
    pub latent_heat_lambda_j_kg: f64,
    pub reference_psychrometric_constant_gamma_pa_k: f64,
    pub lewis_number: f64,
    pub minimum_stomatal_resistance_r_min_s_m: f64,
    pub altitude_pressure_warning: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedWaterValidationSummaryOutput {
    pub evaluation_window: &'static str,
    pub measured_net_irrigation_m3: f64,
    pub simulated_water_consumption_min_m3: f64,
    pub simulated_water_consumption_max_m3: f64,
    pub full_season_overestimation_min_pct: f64,
    pub full_season_overestimation_max_pct: f64,
    pub pre_august_5_overestimation_min_pct: f64,
    pub pre_august_5_overestimation_max_pct: f64,
    pub stem_leaf_dry_matter_range_pct: [f64; 2],
    pub fruit_dry_matter_pct: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCropTranspirationRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub is_daytime: bool,
    pub leaf_area_index: f64,
    pub main_air_density_kg_m3: f64,
    pub leaf_temperature_c: f64,
    pub main_air_co2_ppm: f64,

    /// Absorbed shortwave radiation per m² of cultivated area R_abs, W/m²_cultivated.
    pub absorbed_shortwave_cultivated_w_m2: f64,

    /// VPD_leaf-air = p_sat(T_leaf) - p_v,air, Pa (from Model 7).
    pub vpd_leaf_air_pa: f64,

    /// Boundary-layer heat resistance r_{a,h} and vapour resistance r_{a,v} = Le^(2/3) * r_{a,h}, s/m.
    pub boundary_layer_heat_resistance_rah_s_m: f64,
    pub boundary_layer_vapour_resistance_rav_s_m: f64,

    /// Stomatal resistance multiplicative factors (-).
    pub f_light: f64,
    pub f_temperature: f64,
    pub f_co2: f64,
    pub f_vapour: f64,

    /// Stomatal resistance r_{s,v} = r_min * f_light * f_temperature * f_CO2 * f_vapour, s/m.
    pub stomatal_resistance_rsv_s_m: f64,

    /// Section 27: `water.transpiration_kg_s` (CanTranspi.mW_flow), kg/s for whole cultivated crop.
    pub transpiration_kg_s: f64,

    /// Transpiration rate per m² of cultivated area and per m² of total floor, kg/(s·m²) and mm/h.
    pub transpiration_per_cultivated_m2_kg_s: f64,
    pub transpiration_per_floor_m2_kg_s: f64,
    pub transpiration_rate_cultivated_mm_h: f64,

    /// Latent cooling power removed from the canopy: lambda * E, W.
    pub canopy_latent_cooling_w: f64,

    /// Section 27: `water.cumulative_transpiration_kg` (TotalCanopyTranspiration)
    /// integrated from simulation start up to this timestamp, kg.
    pub cumulative_transpiration_kg: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareCropTranspirationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: TranspirationModelParametersOutput,
    pub published_water_validation: PublishedWaterValidationSummaryOutput,
    pub records: Vec<PreparedCropTranspirationRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    is_daytime: bool,
    leaf_area_index: f64,
    main_air_density_kg_m3: f64,
    leaf_temperature_c: f64,
    main_air_co2_ppm: f64,
    absorbed_shortwave_cultivated_w_m2: f64,
    vpd_leaf_air_pa: f64,
    boundary_layer_heat_resistance_rah_s_m: f64,
    f_co2: f64,
    f_vapour: f64,
    cumulative_transpiration_kg: f64,
}

fn evaluate_record(spec: RecordSpec) -> PreparedCropTranspirationRecord {
    let boundary_layer_vapour_resistance_rav_s_m =
        LEWIS_NUMBER.powf(2.0 / 3.0) * spec.boundary_layer_heat_resistance_rah_s_m;

    let r_abs_per_two_lai =
        spec.absorbed_shortwave_cultivated_w_m2 / (2.0 * spec.leaf_area_index);
    let f_light = (r_abs_per_two_lai + 4.3) / (r_abs_per_two_lai + 0.54);

    let f_temperature = if spec.is_daytime {
        1.0 + 0.023 * (spec.leaf_temperature_c - 24.5).powi(2)
    } else {
        1.0 + 0.005 * (spec.leaf_temperature_c - 33.6).powi(2)
    };

    let stomatal_resistance_rsv_s_m = STANGHELLINI_R_MIN_S_M
        * f_light
        * f_temperature
        * spec.f_co2
        * spec.f_vapour;

    let numerator =
        2.0 * spec.main_air_density_kg_m3 * CP_AIR_J_KG_K * spec.leaf_area_index;
    let denominator = LATENT_HEAT_VAPORIZATION_J_KG
        * REFERENCE_PSYCHROMETRIC_CONSTANT_PA_K
        * (boundary_layer_vapour_resistance_rav_s_m + stomatal_resistance_rsv_s_m);

    let transpiration_per_cultivated_m2_kg_s =
        (numerator / denominator) * spec.vpd_leaf_air_pa;
    let transpiration_kg_s =
        CULTIVATED_AREA_M2 * transpiration_per_cultivated_m2_kg_s;
    let transpiration_per_floor_m2_kg_s = transpiration_kg_s / FLOOR_AREA_M2;
    let transpiration_rate_cultivated_mm_h =
        transpiration_per_cultivated_m2_kg_s * 3_600.0;

    let canopy_latent_cooling_w =
        LATENT_HEAT_VAPORIZATION_J_KG * transpiration_kg_s;

    PreparedCropTranspirationRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        is_daytime: spec.is_daytime,
        leaf_area_index: spec.leaf_area_index,
        main_air_density_kg_m3: spec.main_air_density_kg_m3,
        leaf_temperature_c: spec.leaf_temperature_c,
        main_air_co2_ppm: spec.main_air_co2_ppm,
        absorbed_shortwave_cultivated_w_m2: spec
            .absorbed_shortwave_cultivated_w_m2,
        vpd_leaf_air_pa: spec.vpd_leaf_air_pa,
        boundary_layer_heat_resistance_rah_s_m: spec
            .boundary_layer_heat_resistance_rah_s_m,
        boundary_layer_vapour_resistance_rav_s_m,
        f_light,
        f_temperature,
        f_co2: spec.f_co2,
        f_vapour: spec.f_vapour,
        stomatal_resistance_rsv_s_m,
        transpiration_kg_s,
        transpiration_per_cultivated_m2_kg_s,
        transpiration_per_floor_m2_kg_s,
        transpiration_rate_cultivated_mm_h,
        canopy_latent_cooling_w,
        cumulative_transpiration_kg: spec.cumulative_transpiration_kg,
    }
}

fn reference_records() -> Vec<PreparedCropTranspirationRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): stomata nearly closed in darkness (f_light = 4.3/0.54 = 7.963)
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            is_daytime: false,
            leaf_area_index: 1.1,
            main_air_density_kg_m3: 1.214_470_796_7,
            leaf_temperature_c: 16.8,
            main_air_co2_ppm: 440.0,
            absorbed_shortwave_cultivated_w_m2: 0.0,
            vpd_leaf_air_pa: 382.601_153_317_3,
            boundary_layer_heat_resistance_rah_s_m: 195.0,
            f_co2: 1.00,
            f_vapour: 1.05,
            cumulative_transpiration_kg: 31_420.0,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): moderate winter sunlight opens stomata
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            is_daytime: true,
            leaf_area_index: 1.1,
            main_air_density_kg_m3: 1.199_440_958_2,
            leaf_temperature_c: 21.1,
            main_air_co2_ppm: 680.0,
            absorbed_shortwave_cultivated_w_m2: 69.505_102_336_460 / 0.91,
            vpd_leaf_air_pa: 704.691_872_509_3,
            boundary_layer_heat_resistance_rah_s_m: 165.0,
            f_co2: 1.14,
            f_vapour: 1.12,
            cumulative_transpiration_kg: 31_890.0,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): optimal leaf temperature (24.5 °C -> f_T,day = 1.0)
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            is_daytime: true,
            leaf_area_index: 3.2,
            main_air_density_kg_m3: 1.182_241_541_7,
            leaf_temperature_c: 24.5,
            main_air_co2_ppm: 720.0,
            absorbed_shortwave_cultivated_w_m2: 285.368_071_371_611 / 0.91,
            vpd_leaf_air_pa: 977.240_490_882_7,
            boundary_layer_heat_resistance_rah_s_m: 145.0,
            f_co2: 1.16,
            f_vapour: 1.18,
            cumulative_transpiration_kg: 198_650.0,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): high LAI (4.6) and high VPD_leaf-air (1,524 Pa) drive strong transpiration
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            is_daytime: true,
            leaf_area_index: 4.6,
            main_air_density_kg_m3: 1.147_601_263_3,
            leaf_temperature_c: 30.4,
            main_air_co2_ppm: 410.0,
            absorbed_shortwave_cultivated_w_m2: 310.726_703_060_629 / 0.91,
            vpd_leaf_air_pa: 1_524.060_988_542_8,
            boundary_layer_heat_resistance_rah_s_m: 135.0,
            f_co2: 1.01,
            f_vapour: 1.28,
            cumulative_transpiration_kg: 684_300.0,
        }),
    ]
}

pub fn prepare_crop_transpiration_service(
    payload: PrepareCropTranspirationInput,
) -> Result<PrepareCropTranspirationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CROP_TRANSPIRATION_PRESET_ID);

    if preset != DEFAULT_CROP_TRANSPIRATION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation crop transpiration preset '{preset}'; \
             supported preset is '{DEFAULT_CROP_TRANSPIRATION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCropTranspirationRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCropTranspirationRecord> = all_records
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

    Ok(PrepareCropTranspirationOutput {
        preset_id: DEFAULT_CROP_TRANSPIRATION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.6, Section 4.4.1 (Fig. 13), Stanghellini (1987) & TiGwerLib 0.7.0 CanTranspi",
        parameters: TranspirationModelParametersOutput {
            cultivated_area_a_crop_m2: CULTIVATED_AREA_M2,
            floor_area_m2: FLOOR_AREA_M2,
            cp_air_j_kg_k: CP_AIR_J_KG_K,
            latent_heat_lambda_j_kg: LATENT_HEAT_VAPORIZATION_J_KG,
            reference_psychrometric_constant_gamma_pa_k:
            REFERENCE_PSYCHROMETRIC_CONSTANT_PA_K,
            lewis_number: LEWIS_NUMBER,
            minimum_stomatal_resistance_r_min_s_m: STANGHELLINI_R_MIN_S_M,
            altitude_pressure_warning:
            "The reference implementation uses a fixed sea-level psychrometric constant gamma. \
                 For high-altitude sites like Witeithie (~1,520 m, lower atmospheric pressure), \
                 gamma = c_p * p / (epsilon * lambda) should be pressure-adapted (Section 18).",
        },
        published_water_validation: PublishedWaterValidationSummaryOutput {
            evaluation_window: "2014-12-08 to 2015-10-26 (recorded net irrigation period)",
            measured_net_irrigation_m3: 897.0,
            simulated_water_consumption_min_m3: 1_014.0,
            simulated_water_consumption_max_m3: 1_023.0,
            full_season_overestimation_min_pct: 13.1,
            full_season_overestimation_max_pct: 14.1,
            pre_august_5_overestimation_min_pct: 8.8,
            pre_august_5_overestimation_max_pct: 9.7,
            stem_leaf_dry_matter_range_pct: [7.5, 13.6],
            fruit_dry_matter_pct: 5.4,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stanghellini_resistances_and_transpiration_equations_hold_across_all_records() {
        let output = prepare_crop_transpiration_service(
            PrepareCropTranspirationInput::default(),
        )
            .expect("default crop transpiration preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // r_{a,v} = Le^(2/3) * r_{a,h}
            let expected_rav = LEWIS_NUMBER.powf(2.0 / 3.0)
                * record.boundary_layer_heat_resistance_rah_s_m;
            assert!(
                (record.boundary_layer_vapour_resistance_rav_s_m - expected_rav)
                    .abs()
                    < 1e-12
            );

            // f_light = (R_abs/(2*LAI) + 4.3) / (R_abs/(2*LAI) + 0.54)
            let x = record.absorbed_shortwave_cultivated_w_m2
                / (2.0 * record.leaf_area_index);
            let expected_flight = (x + 4.3) / (x + 0.54);
            assert!((record.f_light - expected_flight).abs() < 1e-12);

            // Day vs night temperature response
            let expected_ft = if record.is_daytime {
                1.0 + 0.023 * (record.leaf_temperature_c - 24.5).powi(2)
            } else {
                1.0 + 0.005 * (record.leaf_temperature_c - 33.6).powi(2)
            };
            assert!((record.f_temperature - expected_ft).abs() < 1e-12);

            // r_{s,v} = r_min * f_light * f_temperature * f_CO2 * f_vapour
            let expected_rsv = STANGHELLINI_R_MIN_S_M
                * expected_flight
                * expected_ft
                * record.f_co2
                * record.f_vapour;
            assert!(
                (record.stomatal_resistance_rsv_s_m - expected_rsv).abs()
                    < 1e-11
            );

            // Main Stanghellini equation for E
            let expected_e = CULTIVATED_AREA_M2
                * ((2.0
                * record.main_air_density_kg_m3
                * CP_AIR_J_KG_K
                * record.leaf_area_index)
                / (LATENT_HEAT_VAPORIZATION_J_KG
                * REFERENCE_PSYCHROMETRIC_CONSTANT_PA_K
                * (expected_rav + expected_rsv)))
                * record.vpd_leaf_air_pa;
            assert!((record.transpiration_kg_s - expected_e).abs() < 1e-12);
        }

        // At equinox_noon (T_leaf = 24.5 °C), f_T,day equals exactly 1.0.
        assert!((output.records[2].f_temperature - 1.0).abs() < 1e-12);
    }
}