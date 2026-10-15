use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CANOPY_HEAT_BALANCE_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const CULTIVATED_AREA_M2: f64 = 943.488;

/// Reference leaf heat capacity per unit leaf area C_leaf, J/(m²_leaf·K).
pub const LEAF_HEAT_CAPACITY_J_M2_LEAF_K: f64 = 1_200.0;

/// Photosynthetic energy conversion fraction eta_photo and PAR share f_PAR.
pub const PHOTOSYNTHETIC_ENERGY_FRACTION_ETA: f64 = 0.01;
pub const PAR_SOLAR_FRACTION: f64 = 0.50;

/// Latent heat of transpiration lambda, J/kg.
pub const LATENT_HEAT_TRANSPIRATION_J_KG: f64 = 2_450_000.0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareCanopyHeatBalanceInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–16 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanopyHeatParametersOutput {
    pub floor_area_m2: f64,
    pub cultivated_area_a_crop_m2: f64,
    pub leaf_heat_capacity_c_leaf_j_m2_k: f64,
    pub photosynthetic_energy_fraction_eta: f64,
    pub par_solar_fraction_f_par: f64,
    pub solar_heat_retention_factor: f64,
    pub latent_heat_transpiration_j_kg: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCanopyHeatBalanceRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Section 27: `crop.leaf_area_index` (m²_leaf / m²_cultivated).
    pub leaf_area_index: f64,

    /// Total one-sided leaf area LAI * A_crop, m²_leaf.
    pub total_leaf_area_m2: f64,

    /// Total canopy heat capacity C_leaf * LAI * A_crop, J/K.
    pub total_canopy_heat_capacity_j_k: f64,

    /// Section 27: `crop.leaf_temperature_c` and `climate.main_air_temperature_c`.
    pub leaf_temperature_c: f64,
    pub leaf_temperature_k: f64,
    pub main_air_temperature_c: f64,
    pub leaf_minus_air_temperature_c: f64,

    pub leaf_convective_coeff_h_leaf_w_m2_k: f64,

    /// Absorbed shortwave radiation from Model 6 (W/m²_floor and whole-canopy W).
    pub absorbed_solar_flux_floor_w_m2: f64,
    pub q_solar_absorbed_w: f64,

    /// Q_solar,heat = Q_solar,absorbed * (1 - eta_photo * f_PAR), W.
    pub q_solar_heat_w: f64,

    /// Net longwave radiation gained by the canopy (pipes + floor + screens - roof), W.
    pub q_longwave_net_w: f64,

    /// Q_canopy->air = 2 * LAI * h_leaf * A_crop * (T_leaf - T_air), W.
    /// Positive means heat transfers from leaves to MainAir.
    pub q_conv_canopy_to_air_w: f64,

    /// Transpiration mass flow E (kg/s) and latent cooling lambda * E (W).
    pub transpiration_mass_flow_e_kg_s: f64,
    pub q_latent_transpiration_w: f64,

    /// Net heat accumulation rate C_leaf * LAI * A_crop * dT_leaf/dt, W.
    pub net_canopy_heat_accumulation_w: f64,

    /// Leaf temperature derivative dT_leaf/dt, K/s (and K/h).
    pub dt_leaf_dt_k_per_s: f64,
    pub dt_leaf_dt_k_per_h: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareCanopyHeatBalanceOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: CanopyHeatParametersOutput,
    pub records: Vec<PreparedCanopyHeatBalanceRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    leaf_area_index: f64,
    leaf_temperature_c: f64,
    main_air_temperature_c: f64,
    leaf_convective_coeff_h_leaf_w_m2_k: f64,
    absorbed_solar_flux_floor_w_m2: f64,
    q_longwave_net_w: f64,
    transpiration_mass_flow_e_kg_s: f64,
}

fn evaluate_record(spec: RecordSpec) -> PreparedCanopyHeatBalanceRecord {
    let total_leaf_area_m2 = spec.leaf_area_index * CULTIVATED_AREA_M2;
    let total_canopy_heat_capacity_j_k =
        LEAF_HEAT_CAPACITY_J_M2_LEAF_K * total_leaf_area_m2;

    let leaf_minus_air_temperature_c =
        spec.leaf_temperature_c - spec.main_air_temperature_c;

    let q_solar_absorbed_w =
        spec.absorbed_solar_flux_floor_w_m2 * FLOOR_AREA_M2;
    let q_solar_heat_w = q_solar_absorbed_w
        * (1.0 - PHOTOSYNTHETIC_ENERGY_FRACTION_ETA * PAR_SOLAR_FRACTION);

    let q_conv_canopy_to_air_w = 2.0
        * spec.leaf_area_index
        * spec.leaf_convective_coeff_h_leaf_w_m2_k
        * CULTIVATED_AREA_M2
        * leaf_minus_air_temperature_c;

    let q_latent_transpiration_w =
        LATENT_HEAT_TRANSPIRATION_J_KG * spec.transpiration_mass_flow_e_kg_s;

    let net_canopy_heat_accumulation_w = q_solar_heat_w + spec.q_longwave_net_w
        - q_conv_canopy_to_air_w
        - q_latent_transpiration_w;

    let dt_leaf_dt_k_per_s =
        net_canopy_heat_accumulation_w / total_canopy_heat_capacity_j_k;

    PreparedCanopyHeatBalanceRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        leaf_area_index: spec.leaf_area_index,
        total_leaf_area_m2,
        total_canopy_heat_capacity_j_k,
        leaf_temperature_c: spec.leaf_temperature_c,
        leaf_temperature_k: spec.leaf_temperature_c + 273.15,
        main_air_temperature_c: spec.main_air_temperature_c,
        leaf_minus_air_temperature_c,
        leaf_convective_coeff_h_leaf_w_m2_k: spec
            .leaf_convective_coeff_h_leaf_w_m2_k,
        absorbed_solar_flux_floor_w_m2: spec.absorbed_solar_flux_floor_w_m2,
        q_solar_absorbed_w,
        q_solar_heat_w,
        q_longwave_net_w: spec.q_longwave_net_w,
        q_conv_canopy_to_air_w,
        transpiration_mass_flow_e_kg_s: spec.transpiration_mass_flow_e_kg_s,
        q_latent_transpiration_w,
        net_canopy_heat_accumulation_w,
        dt_leaf_dt_k_per_s,
        dt_leaf_dt_k_per_h: dt_leaf_dt_k_per_s * 3_600.0,
    }
}

fn reference_records() -> Vec<PreparedCanopyHeatBalanceRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): leaves slightly cooler than air (16.8 °C vs 17.2 °C),
        //    gaining heat from MainAir convection and pipe longwave while losing latent heat to night transpiration.
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            leaf_area_index: 1.1,
            leaf_temperature_c: 16.8,
            main_air_temperature_c: 17.2,
            leaf_convective_coeff_h_leaf_w_m2_k: 2.85,
            absorbed_solar_flux_floor_w_m2: 0.0,
            q_longwave_net_w: 2_950.0,
            transpiration_mass_flow_e_kg_s: 0.002_18,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): solar absorption (69.51 W/m²_floor from Model 6) warms leaves above air (21.1 °C vs 20.4 °C)
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            leaf_area_index: 1.1,
            leaf_temperature_c: 21.1,
            main_air_temperature_c: 20.4,
            leaf_convective_coeff_h_leaf_w_m2_k: 3.20,
            absorbed_solar_flux_floor_w_m2: 69.505_102_336_460,
            q_longwave_net_w: -4_200.0,
            transpiration_mass_flow_e_kg_s: 0.025_60,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): 285.37 W/m²_floor absorbed solar heat, LAI = 3.2, T_leaf = 24.5 °C
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            leaf_area_index: 3.2,
            leaf_temperature_c: 24.5,
            main_air_temperature_c: 23.6,
            leaf_convective_coeff_h_leaf_w_m2_k: 3.45,
            absorbed_solar_flux_floor_w_m2: 285.368_071_371_611,
            q_longwave_net_w: -9_800.0,
            transpiration_mass_flow_e_kg_s: 0.108_50,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): strong transpiration evaporative cooling keeps T_leaf (30.4 °C) below T_air (31.2 °C)
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            leaf_area_index: 4.6,
            leaf_temperature_c: 30.4,
            main_air_temperature_c: 31.2,
            leaf_convective_coeff_h_leaf_w_m2_k: 3.60,
            absorbed_solar_flux_floor_w_m2: 310.726_703_060_629,
            q_longwave_net_w: 8_400.0,
            transpiration_mass_flow_e_kg_s: 0.147_20,
        }),
    ]
}

pub fn prepare_canopy_heat_balance_service(
    payload: PrepareCanopyHeatBalanceInput,
) -> Result<PrepareCanopyHeatBalanceOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CANOPY_HEAT_BALANCE_PRESET_ID);

    if preset != DEFAULT_CANOPY_HEAT_BALANCE_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation canopy heat balance preset '{preset}'; \
             supported preset is '{DEFAULT_CANOPY_HEAT_BALANCE_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCanopyHeatBalanceRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCanopyHeatBalanceRecord> = all_records
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

    Ok(PrepareCanopyHeatBalanceOutput {
        preset_id: DEFAULT_CANOPY_HEAT_BALANCE_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.1, Vanthoor (2011) & TiGwerLib 0.7.0 Canopy",
        parameters: CanopyHeatParametersOutput {
            floor_area_m2: FLOOR_AREA_M2,
            cultivated_area_a_crop_m2: CULTIVATED_AREA_M2,
            leaf_heat_capacity_c_leaf_j_m2_k: LEAF_HEAT_CAPACITY_J_M2_LEAF_K,
            photosynthetic_energy_fraction_eta:
            PHOTOSYNTHETIC_ENERGY_FRACTION_ETA,
            par_solar_fraction_f_par: PAR_SOLAR_FRACTION,
            solar_heat_retention_factor: 1.0
                - PHOTOSYNTHETIC_ENERGY_FRACTION_ETA * PAR_SOLAR_FRACTION,
            latent_heat_transpiration_j_kg: LATENT_HEAT_TRANSPIRATION_J_KG,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canopy_heat_balance_convection_and_solar_reduction_hold_across_all_records() {
        let output = prepare_canopy_heat_balance_service(
            PrepareCanopyHeatBalanceInput::default(),
        )
            .expect("default canopy heat balance preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // Q_solar,heat = Q_solar,absorbed * (1 - eta_photo * f_PAR)
            let expected_q_sol_heat = record.q_solar_absorbed_w
                * (1.0
                - PHOTOSYNTHETIC_ENERGY_FRACTION_ETA * PAR_SOLAR_FRACTION);
            assert!(
                (record.q_solar_heat_w - expected_q_sol_heat).abs() < 1e-9
            );

            // Q_canopy->air = 2 * LAI * h_leaf * A_crop * (T_leaf - T_air)
            let expected_q_conv = 2.0
                * record.leaf_area_index
                * record.leaf_convective_coeff_h_leaf_w_m2_k
                * CULTIVATED_AREA_M2
                * (record.leaf_temperature_c - record.main_air_temperature_c);
            assert!(
                (record.q_conv_canopy_to_air_w - expected_q_conv).abs() < 1e-9
            );

            // C_leaf * LAI * A_crop * dT_leaf/dt = Q_solar,heat + Q_longwave,net - Q_conv,out - lambda * E
            let expected_c_tot = LEAF_HEAT_CAPACITY_J_M2_LEAF_K
                * record.leaf_area_index
                * CULTIVATED_AREA_M2;
            let expected_rhs = record.q_solar_heat_w + record.q_longwave_net_w
                - record.q_conv_canopy_to_air_w
                - LATENT_HEAT_TRANSPIRATION_J_KG
                * record.transpiration_mass_flow_e_kg_s;

            assert!(
                (expected_c_tot * record.dt_leaf_dt_k_per_s - expected_rhs)
                    .abs()
                    < 1e-9
            );
        }
    }
}