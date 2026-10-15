use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CARBOHYDRATE_ALLOCATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const SECONDS_PER_DAY: f64 = 86_400.0;

/// Growth respiration coefficients c_{i,g} (-).
pub const C_FRUIT_GROWTH_RESP: f64 = 0.27;
pub const C_LEAF_GROWTH_RESP: f64 = 0.28;
pub const C_STEM_GROWTH_RESP: f64 = 0.30;

/// Maintenance respiration coefficients c_{i,m} (s⁻¹), Q_10 (-), and c_RGR (s).
pub const C_FRUIT_MAINT_RESP_S_INV: f64 = 1.16e-7;
pub const C_LEAF_MAINT_RESP_S_INV: f64 = 3.47e-7;
pub const C_STEM_MAINT_RESP_S_INV: f64 = 1.47e-7;
pub const Q10_MAINT_RESP: f64 = 2.0;
pub const C_RGR_S: f64 = 2.85e6;

/// Potential organ growth rate coefficients r_i (mg CH2O/(m²·s)).
pub const R_FRUIT_POTENTIAL_MG_M2_S: f64 = 0.32;
pub const R_LEAF_POTENTIAL_MG_M2_S: f64 = 0.095;
pub const R_STEM_POTENTIAL_MG_M2_S: f64 = 0.075;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareCarbohydrateAllocationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–19 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CarbohydrateAllocationParametersOutput {
    pub cultivar: &'static str,
    pub seconds_per_day: f64,
    pub c_fruit_growth_resp: f64,
    pub c_leaf_growth_resp: f64,
    pub c_stem_growth_resp: f64,
    pub c_fruit_maint_resp_s_inv: f64,
    pub c_leaf_maint_resp_s_inv: f64,
    pub c_stem_maint_resp_s_inv: f64,
    pub q10_maint_resp: f64,
    pub c_rgr_s: f64,
    pub r_fruit_potential_mg_m2_s: f64,
    pub r_leaf_potential_mg_m2_s: f64,
    pub r_stem_potential_mg_m2_s: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrganMaintenanceRespirationOutput {
    pub organ_name: &'static str,
    pub maintenance_coeff_c_m_s_inv: f64,
    pub organ_carbohydrate_mass_c_i_mg_m2: f64,
    pub relative_growth_rate_rgr_s_inv: f64,
    pub r_maintenance_mg_m2_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCarbohydrateAllocationRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub leaf_temperature_c: f64,

    /// First-order filtered 24-h leaf temperature state T_24 (°C) and derivative dT_24/dt (°C/s).
    pub t_24_c: f64,
    pub dt_24_dt_c_per_s: f64,

    /// Accumulated thermal time T_sum (°C·d) and derivative dT_sum/dt (°C/s).
    pub t_sum_c_days: f64,
    pub dt_sum_dt_c_per_s: f64,

    /// g_{T24} = 0.047 * T_24 + 0.06 (-).
    pub g_t24: f64,

    /// Growth inhibition factors in [0, 1] (-).
    pub h_b_buffer_availability: f64,
    pub h_t_instantaneous_temperature: f64,
    pub h_t24_mean_temperature: f64,
    pub h_development_stage: f64,

    /// Carbohydrate input from Model 19 (F_photosynthesis), mg CH2O/(m²·s).
    pub f_photosynthesis_mg_m2_s: f64,

    /// Organ carbohydrate allocation rates, mg CH2O/(m²·s).
    pub f_fruit_mg_m2_s: f64,
    pub f_leaf_mg_m2_s: f64,
    pub f_stem_mg_m2_s: f64,

    /// Growth respiration R_growth = c_{fruit,g}*F_fruit + c_{leaf,g}*F_leaf + c_{stem,g}*F_stem, mg/(m²·s).
    pub r_growth_mg_m2_s: f64,

    /// Carbohydrate buffer state C_buffer (mg/m²) and derivative dC_buffer/dt (mg/(m²·s)).
    pub carbohydrate_buffer_mg_m2: f64,
    pub dc_buffer_dt_mg_m2_s: f64,

    /// Organ maintenance respiration terms, mg CH2O/(m²·s).
    pub fruit_maintenance: OrganMaintenanceRespirationOutput,
    pub leaf_maintenance: OrganMaintenanceRespirationOutput,
    pub stem_maintenance: OrganMaintenanceRespirationOutput,
    pub total_maintenance_respiration_mg_m2_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareCarbohydrateAllocationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: CarbohydrateAllocationParametersOutput,
    pub records: Vec<PreparedCarbohydrateAllocationRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    leaf_temperature_c: f64,
    t_24_c: f64,
    t_sum_c_days: f64,
    h_b_buffer_availability: f64,
    h_t_instantaneous_temperature: f64,
    h_t24_mean_temperature: f64,
    h_development_stage: f64,
    f_photosynthesis_mg_m2_s: f64,
    carbohydrate_buffer_mg_m2: f64,
    c_fruit_mass_mg_m2: f64,
    c_leaf_mass_mg_m2: f64,
    c_stem_mass_mg_m2: f64,
}

fn evaluate_organ_maintenance(
    organ_name: &'static str,
    c_m_s_inv: f64,
    t_24_c: f64,
    c_i_mg_m2: f64,
    f_i_mg_m2_s: f64,
) -> OrganMaintenanceRespirationOutput {
    let rgr_s_inv = if c_i_mg_m2 > 0.0 {
        f_i_mg_m2_s / c_i_mg_m2
    } else {
        0.0
    };

    let q10_factor = Q10_MAINT_RESP.powf((t_24_c - 25.0) / 10.0);
    let rgr_factor = 1.0 - (-C_RGR_S * rgr_s_inv).exp();
    let r_maintenance_mg_m2_s = c_m_s_inv * q10_factor * c_i_mg_m2 * rgr_factor;

    OrganMaintenanceRespirationOutput {
        organ_name,
        maintenance_coeff_c_m_s_inv: c_m_s_inv,
        organ_carbohydrate_mass_c_i_mg_m2: c_i_mg_m2,
        relative_growth_rate_rgr_s_inv: rgr_s_inv,
        r_maintenance_mg_m2_s,
    }
}

fn evaluate_record(spec: RecordSpec) -> PreparedCarbohydrateAllocationRecord {
    let dt_24_dt_c_per_s =
        (spec.leaf_temperature_c - spec.t_24_c) / SECONDS_PER_DAY;
    let dt_sum_dt_c_per_s = spec.leaf_temperature_c / SECONDS_PER_DAY;

    let g_t24 = 0.047 * spec.t_24_c + 0.06;

    let f_fruit_mg_m2_s = spec.h_b_buffer_availability
        * spec.h_t_instantaneous_temperature
        * spec.h_t24_mean_temperature
        * spec.h_development_stage
        * g_t24
        * R_FRUIT_POTENTIAL_MG_M2_S;

    let f_leaf_mg_m2_s = spec.h_b_buffer_availability
        * spec.h_t24_mean_temperature
        * g_t24
        * R_LEAF_POTENTIAL_MG_M2_S;

    let f_stem_mg_m2_s = spec.h_b_buffer_availability
        * spec.h_t24_mean_temperature
        * g_t24
        * R_STEM_POTENTIAL_MG_M2_S;

    let r_growth_mg_m2_s = C_FRUIT_GROWTH_RESP * f_fruit_mg_m2_s
        + C_LEAF_GROWTH_RESP * f_leaf_mg_m2_s
        + C_STEM_GROWTH_RESP * f_stem_mg_m2_s;

    let dc_buffer_dt_mg_m2_s = spec.f_photosynthesis_mg_m2_s
        - f_fruit_mg_m2_s
        - f_leaf_mg_m2_s
        - f_stem_mg_m2_s
        - r_growth_mg_m2_s;

    let fruit_maintenance = evaluate_organ_maintenance(
        "Fruit",
        C_FRUIT_MAINT_RESP_S_INV,
        spec.t_24_c,
        spec.c_fruit_mass_mg_m2,
        f_fruit_mg_m2_s,
    );
    let leaf_maintenance = evaluate_organ_maintenance(
        "Leaf",
        C_LEAF_MAINT_RESP_S_INV,
        spec.t_24_c,
        spec.c_leaf_mass_mg_m2,
        f_leaf_mg_m2_s,
    );
    let stem_maintenance = evaluate_organ_maintenance(
        "Stem",
        C_STEM_MAINT_RESP_S_INV,
        spec.t_24_c,
        spec.c_stem_mass_mg_m2,
        f_stem_mg_m2_s,
    );

    let total_maintenance_respiration_mg_m2_s =
        fruit_maintenance.r_maintenance_mg_m2_s
            + leaf_maintenance.r_maintenance_mg_m2_s
            + stem_maintenance.r_maintenance_mg_m2_s;

    PreparedCarbohydrateAllocationRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        leaf_temperature_c: spec.leaf_temperature_c,
        t_24_c: spec.t_24_c,
        dt_24_dt_c_per_s,
        t_sum_c_days: spec.t_sum_c_days,
        dt_sum_dt_c_per_s,
        g_t24,
        h_b_buffer_availability: spec.h_b_buffer_availability,
        h_t_instantaneous_temperature: spec.h_t_instantaneous_temperature,
        h_t24_mean_temperature: spec.h_t24_mean_temperature,
        h_development_stage: spec.h_development_stage,
        f_photosynthesis_mg_m2_s: spec.f_photosynthesis_mg_m2_s,
        f_fruit_mg_m2_s,
        f_leaf_mg_m2_s,
        f_stem_mg_m2_s,
        r_growth_mg_m2_s,
        carbohydrate_buffer_mg_m2: spec.carbohydrate_buffer_mg_m2,
        dc_buffer_dt_mg_m2_s,
        fruit_maintenance,
        leaf_maintenance,
        stem_maintenance,
        total_maintenance_respiration_mg_m2_s,
    }
}

fn reference_records() -> Vec<PreparedCarbohydrateAllocationRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): F_photosynthesis = 0, plant draws down C_buffer for nighttime growth
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            leaf_temperature_c: 16.8,
            t_24_c: 18.2,
            t_sum_c_days: 685.0,
            h_b_buffer_availability: 0.94,
            h_t_instantaneous_temperature: 0.88,
            h_t24_mean_temperature: 0.92,
            h_development_stage: 0.65,
            f_photosynthesis_mg_m2_s: 0.0,
            carbohydrate_buffer_mg_m2: 4_200.0,
            c_fruit_mass_mg_m2: 18_500.0,
            c_leaf_mass_mg_m2: 41_250.0,
            c_stem_mass_mg_m2: 26_000.0,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): daytime photosynthesis (0.3714 mg/(m²·s) from Model 19) recharges buffer
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            leaf_temperature_c: 21.1,
            t_24_c: 18.5,
            t_sum_c_days: 692.5,
            h_b_buffer_availability: 0.98,
            h_t_instantaneous_temperature: 0.97,
            h_t24_mean_temperature: 0.93,
            h_development_stage: 0.66,
            f_photosynthesis_mg_m2_s: 0.371_413_412_069,
            carbohydrate_buffer_mg_m2: 7_800.0,
            c_fruit_mass_mg_m2: 19_100.0,
            c_leaf_mass_mg_m2: 41_250.0,
            c_stem_mass_mg_m2: 26_400.0,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): strong photosynthesis (1.2981 mg/(m²·s)) and full fruit allocation
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            leaf_temperature_c: 24.5,
            t_24_c: 20.6,
            t_sum_c_days: 1_980.0,
            h_b_buffer_availability: 0.99,
            h_t_instantaneous_temperature: 1.00,
            h_t24_mean_temperature: 0.98,
            h_development_stage: 1.00,
            f_photosynthesis_mg_m2_s: 1.298_093_762_991,
            carbohydrate_buffer_mg_m2: 12_500.0,
            c_fruit_mass_mg_m2: 142_000.0,
            c_leaf_mass_mg_m2: 120_000.0,
            c_stem_mass_mg_m2: 95_000.0,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): high T_24 (24.2 °C) increases maintenance respiration and g_T24
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            leaf_temperature_c: 30.4,
            t_24_c: 24.2,
            t_sum_c_days: 4_420.0,
            h_b_buffer_availability: 0.99,
            h_t_instantaneous_temperature: 0.89,
            h_t24_mean_temperature: 0.96,
            h_development_stage: 1.00,
            f_photosynthesis_mg_m2_s: 1.217_484_605_421,
            carbohydrate_buffer_mg_m2: 15_200.0,
            c_fruit_mass_mg_m2: 185_000.0,
            c_leaf_mass_mg_m2: 172_500.0,
            c_stem_mass_mg_m2: 168_000.0,
        }),
    ]
}

pub fn prepare_carbohydrate_allocation_service(
    payload: PrepareCarbohydrateAllocationInput,
) -> Result<PrepareCarbohydrateAllocationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CARBOHYDRATE_ALLOCATION_PRESET_ID);

    if preset != DEFAULT_CARBOHYDRATE_ALLOCATION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation carbohydrate allocation preset '{preset}'; \
             supported preset is '{DEFAULT_CARBOHYDRATE_ALLOCATION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCarbohydrateAllocationRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCarbohydrateAllocationRecord> = all_records
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

    Ok(PrepareCarbohydrateAllocationOutput {
        preset_id: DEFAULT_CARBOHYDRATE_ALLOCATION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.6, Section 3.7.1, Vanthoor et al. (2011) & TiGwerLib 0.7.0 TYM",
        parameters: CarbohydrateAllocationParametersOutput {
            cultivar: "Clodano F1 Hybrid (Syngenta)",
            seconds_per_day: SECONDS_PER_DAY,
            c_fruit_growth_resp: C_FRUIT_GROWTH_RESP,
            c_leaf_growth_resp: C_LEAF_GROWTH_RESP,
            c_stem_growth_resp: C_STEM_GROWTH_RESP,
            c_fruit_maint_resp_s_inv: C_FRUIT_MAINT_RESP_S_INV,
            c_leaf_maint_resp_s_inv: C_LEAF_MAINT_RESP_S_INV,
            c_stem_maint_resp_s_inv: C_STEM_MAINT_RESP_S_INV,
            q10_maint_resp: Q10_MAINT_RESP,
            c_rgr_s: C_RGR_S,
            r_fruit_potential_mg_m2_s: R_FRUIT_POTENTIAL_MG_M2_S,
            r_leaf_potential_mg_m2_s: R_LEAF_POTENTIAL_MG_M2_S,
            r_stem_potential_mg_m2_s: R_STEM_POTENTIAL_MG_M2_S,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_balance_allocation_and_respiration_equations_hold_across_all_records() {
        let output = prepare_carbohydrate_allocation_service(
            PrepareCarbohydrateAllocationInput::default(),
        )
            .expect("default carbohydrate allocation preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // dT_24/dt = (T_leaf,C - T_24)/86400 and dT_sum/dt = T_leaf,C/86400
            assert!(
                (record.dt_24_dt_c_per_s
                    - (record.leaf_temperature_c - record.t_24_c) / 86_400.0)
                    .abs()
                    < 1e-15
            );
            assert!(
                (record.dt_sum_dt_c_per_s - record.leaf_temperature_c / 86_400.0)
                    .abs()
                    < 1e-15
            );

            // g_{T24} = 0.047 * T_24 + 0.06
            let expected_g = 0.047 * record.t_24_c + 0.06;
            assert!((record.g_t24 - expected_g).abs() < 1e-12);

            // Organ allocations
            let expected_f_fruit = record.h_b_buffer_availability
                * record.h_t_instantaneous_temperature
                * record.h_t24_mean_temperature
                * record.h_development_stage
                * expected_g
                * R_FRUIT_POTENTIAL_MG_M2_S;
            let expected_f_leaf = record.h_b_buffer_availability
                * record.h_t24_mean_temperature
                * expected_g
                * R_LEAF_POTENTIAL_MG_M2_S;
            let expected_f_stem = record.h_b_buffer_availability
                * record.h_t24_mean_temperature
                * expected_g
                * R_STEM_POTENTIAL_MG_M2_S;

            assert!((record.f_fruit_mg_m2_s - expected_f_fruit).abs() < 1e-12);
            assert!((record.f_leaf_mg_m2_s - expected_f_leaf).abs() < 1e-12);
            assert!((record.f_stem_mg_m2_s - expected_f_stem).abs() < 1e-12);

            // R_growth
            let expected_r_growth = C_FRUIT_GROWTH_RESP * expected_f_fruit
                + C_LEAF_GROWTH_RESP * expected_f_leaf
                + C_STEM_GROWTH_RESP * expected_f_stem;
            assert!((record.r_growth_mg_m2_s - expected_r_growth).abs() < 1e-12);

            // dC_buffer/dt = F_photosynthesis - F_fruit - F_leaf - F_stem - R_growth
            let expected_dc_buf = record.f_photosynthesis_mg_m2_s
                - expected_f_fruit
                - expected_f_leaf
                - expected_f_stem
                - expected_r_growth;
            assert!(
                (record.dc_buffer_dt_mg_m2_s - expected_dc_buf).abs() < 1e-12
            );

            // Maintenance respiration equation check
            for maint in [
                &record.fruit_maintenance,
                &record.leaf_maintenance,
                &record.stem_maintenance,
            ] {
                let expected_r_maint = maint.maintenance_coeff_c_m_s_inv
                    * Q10_MAINT_RESP.powf((record.t_24_c - 25.0) / 10.0)
                    * maint.organ_carbohydrate_mass_c_i_mg_m2
                    * (1.0
                    - (-C_RGR_S * maint.relative_growth_rate_rgr_s_inv)
                    .exp());
                assert!(
                    (maint.r_maintenance_mg_m2_s - expected_r_maint).abs()
                        < 1e-12
                );
            }
        }

        // At winter_night, photosynthesis is zero but organ growth continues by drawing down C_buffer.
        assert_eq!(output.records[0].f_photosynthesis_mg_m2_s, 0.0);
        assert!(output.records[0].f_fruit_mg_m2_s > 0.0);
        assert!(output.records[0].dc_buffer_dt_mg_m2_s < 0.0);
    }
}