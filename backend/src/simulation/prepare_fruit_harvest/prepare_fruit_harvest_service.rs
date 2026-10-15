use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_FRUIT_HARVEST_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const CULTIVATED_AREA_M2: f64 = 943.488;
pub const NUM_DEVELOPMENT_STAGES: u16 = 50;
pub const SECONDS_PER_DAY: f64 = 86_400.0;

/// Fruit development rate coefficients r_dev = c_1 + c_2 * T_24 (s⁻¹).
pub const C1_DEV_RATE_S_INV: f64 = -7.64e-8;
pub const C2_DEV_RATE_PER_C_S: f64 = 1.16e-8;

/// Potential fruit set parameters: F_set,max = n_plants * (a + b * T_24) (fruits/(m²·s)).
pub const PLANT_DENSITY_PER_M2: f64 = 2.60;
pub const FRUIT_SET_A_S_INV: f64 = -1.70e-7;
pub const FRUIT_SET_B_PER_C_S: f64 = 3.60e-8;

/// Gompertz potential growth parameters.
pub const GOMPERTZ_G_MAX_MG_FRUIT_S: f64 = 0.042;
pub const GOMPERTZ_B_PER_DAY: f64 = 0.088;

/// Carbohydrate-to-dry-matter conversion factor eta_{C->DM} (-) and fruit dry-matter fraction f_DM (-).
pub const ETA_CH2O_TO_DM: f64 = 1.00;
pub const REFERENCE_FRUIT_DRY_MATTER_FRACTION: f64 = 0.054;

/// Default external marketable fraction assumption and practical first-picking threshold (kg/m²).
pub const DEFAULT_MARKETABLE_FRACTION: f64 = 0.92;
pub const PRACTICAL_FIRST_HARVEST_THRESHOLD_KG_M2: f64 = 0.50;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareFruitHarvestInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–21 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct FruitHarvestParametersOutput {
    pub cultivar: &'static str,
    pub cultivated_area_a_crop_m2: f64,
    pub num_development_stages: u16,
    pub plant_density_per_m2: f64,
    pub c1_dev_rate_s_inv: f64,
    pub c2_dev_rate_per_c_s: f64,
    pub eta_ch2o_to_dm: f64,
    pub reference_dry_matter_fraction_f_dm: f64,
    pub default_marketable_fraction: f64,
    pub practical_first_harvest_threshold_kg_m2: f64,
    pub marketable_scope_note: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedYieldValidationSummaryOutput {
    pub fresh_yield_rmse_kg_m2: f64,
    pub fresh_yield_mae_kg_m2: f64,
    pub fruit_count_rmse_fruits_m2: f64,
    pub fruit_count_mae_fruits_m2: f64,
    pub season_final_fresh_yield_kg_m2: f64,
    pub season_final_whole_greenhouse_fresh_kg: f64,
    pub season_final_fruit_count_m2: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct StageTransferSnapshotOutput {
    pub stage_index_j: u16,
    pub stage_age_days_tj: f64,
    pub fruit_count_nj_per_m2: f64,
    pub fruit_carbohydrate_cj_mg_m2: f64,

    /// Gompertz potential growth rate GR_j = G_max * exp(-exp(-B*(t_j - M))) * B * exp(-B*(t_j - M)), mg/(fruit·s).
    pub gompertz_gr_j_mg_fruit_s: f64,

    pub f_n_in_fruits_m2_s: f64,
    pub f_n_out_fruits_m2_s: f64,
    pub dn_j_dt_fruits_m2_s: f64,

    pub f_buffer_to_j_mg_m2_s: f64,
    pub f_c_in_mg_m2_s: f64,
    pub f_c_out_mg_m2_s: f64,
    pub r_j_maintenance_mg_m2_s: f64,
    pub dc_j_dt_mg_m2_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedFruitHarvestRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub t_24_c: f64,
    pub h_development: f64,

    /// Development rate r_dev = c_1 + c_2 * T_24 (s⁻¹) and Fruit Growth Period FGP = 1/(86400 * r_dev) (days).
    pub r_dev_s_inv: f64,
    pub fruit_growth_period_fgp_days: f64,
    pub gompertz_inflection_m_days: f64,

    /// Potential fruit set F_set,max = n_plants * (a + b * T_24), fruits/(m²·s).
    pub f_set_max_fruits_m2_s: f64,

    /// Snapshot of Final Stage 50 transferring into harvest.
    pub final_stage_50: StageTransferSnapshotOutput,

    /// dDM_harvest/dt = eta_{C->DM} * F^C_{50->harvest}, mg DM/(m²·s).
    pub d_dm_harvest_dt_mg_m2_s: f64,

    /// Section 27: `harvest.dry_matter_mg_m2` (TYM.DM_Har), mg DM/m²_cultivated.
    pub dry_matter_mg_m2: f64,

    /// Section 27: `harvest.fruit_count_m2` (TYM.N_Har), fruits/m²_cultivated.
    pub fruit_count_m2: f64,

    /// Section 27: `harvest.fresh_kg_m2` (HarvestedFruitsFreshWeightPerSqMeter_simu),
    /// H_{fresh,kg/m²} = DM_{harvest,mg/m²} / (10^6 * f_DM).
    pub fresh_kg_m2: f64,

    /// H_{whole,kg} = A_crop * H_{fresh,kg/m²}, kg.
    pub whole_greenhouse_fresh_kg: f64,

    /// H_marketable = f_marketable * H_fresh (per m² cultivated and whole greenhouse), kg.
    pub marketable_fresh_kg_m2: f64,
    pub whole_greenhouse_marketable_kg: f64,

    /// Average harvested fruit fresh mass (g/fruit) when fruit_count_m2 > 0.
    pub average_fruit_fresh_mass_g: Option<f64>,

    /// True once fresh_kg_m2 >= PRACTICAL_FIRST_HARVEST_THRESHOLD_KG_M2.
    pub practical_commercial_harvest_reached: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MonthlyHarvestCalendarEntryOutput {
    pub month_label: &'static str,
    pub cumulative_start_fresh_kg_m2: f64,
    pub cumulative_end_fresh_kg_m2: f64,

    /// H_month = H_{cumulative,end} - H_{cumulative,start}, kg/m²_cultivated.
    pub monthly_fresh_kg_m2: f64,

    pub monthly_whole_greenhouse_fresh_kg: f64,
    pub monthly_whole_greenhouse_marketable_kg: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareFruitHarvestOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: FruitHarvestParametersOutput,
    pub published_yield_validation: PublishedYieldValidationSummaryOutput,
    pub records: Vec<PreparedFruitHarvestRecord>,
    pub monthly_harvest_progression: [MonthlyHarvestCalendarEntryOutput; 8],
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    t_24_c: f64,
    h_development: f64,
    stage50_n_fruits_m2: f64,
    stage50_c_mg_m2: f64,
    f_n_49_to_50_fruits_m2_s: f64,
    f_buffer_to_50_mg_m2_s: f64,
    f_c_49_to_50_mg_m2_s: f64,
    r_50_maintenance_mg_m2_s: f64,
    cumulative_dm_harvest_mg_m2: f64,
    cumulative_fruit_count_m2: f64,
}

fn evaluate_record(spec: RecordSpec) -> PreparedFruitHarvestRecord {
    let n_dev = f64::from(NUM_DEVELOPMENT_STAGES);
    let r_dev_s_inv = C1_DEV_RATE_S_INV + C2_DEV_RATE_PER_C_S * spec.t_24_c;
    let fruit_growth_period_fgp_days = 1.0 / (SECONDS_PER_DAY * r_dev_s_inv);
    let gompertz_inflection_m_days = 0.48 * fruit_growth_period_fgp_days;

    let f_set_max_fruits_m2_s = PLANT_DENSITY_PER_M2
        * (FRUIT_SET_A_S_INV + FRUIT_SET_B_PER_C_S * spec.t_24_c).max(0.0);

    // Stage 50 age t_50 = FGP
    let t_j = fruit_growth_period_fgp_days;
    let exp_inner = (-GOMPERTZ_B_PER_DAY * (t_j - gompertz_inflection_m_days)).exp();
    let gompertz_gr_j_mg_fruit_s =
        GOMPERTZ_G_MAX_MG_FRUIT_S * (-exp_inner).exp() * GOMPERTZ_B_PER_DAY * exp_inner;

    let f_n_out_fruits_m2_s =
        n_dev * r_dev_s_inv * spec.h_development * spec.stage50_n_fruits_m2;
    let f_c_out_mg_m2_s = n_dev * r_dev_s_inv * spec.stage50_c_mg_m2;

    let dn_j_dt_fruits_m2_s = spec.f_n_49_to_50_fruits_m2_s - f_n_out_fruits_m2_s;
    let dc_j_dt_mg_m2_s = spec.f_buffer_to_50_mg_m2_s + spec.f_c_49_to_50_mg_m2_s
        - f_c_out_mg_m2_s
        - spec.r_50_maintenance_mg_m2_s;

    let d_dm_harvest_dt_mg_m2_s = ETA_CH2O_TO_DM * f_c_out_mg_m2_s;

    let fresh_kg_m2 = spec.cumulative_dm_harvest_mg_m2
        / (1e6 * REFERENCE_FRUIT_DRY_MATTER_FRACTION);
    let whole_greenhouse_fresh_kg = CULTIVATED_AREA_M2 * fresh_kg_m2;

    let marketable_fresh_kg_m2 = DEFAULT_MARKETABLE_FRACTION * fresh_kg_m2;
    let whole_greenhouse_marketable_kg =
        DEFAULT_MARKETABLE_FRACTION * whole_greenhouse_fresh_kg;

    let average_fruit_fresh_mass_g = if spec.cumulative_fruit_count_m2 > 0.0 {
        Some((fresh_kg_m2 * 1_000.0) / spec.cumulative_fruit_count_m2)
    } else {
        None
    };

    let practical_commercial_harvest_reached =
        fresh_kg_m2 >= PRACTICAL_FIRST_HARVEST_THRESHOLD_KG_M2;

    PreparedFruitHarvestRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        t_24_c: spec.t_24_c,
        h_development: spec.h_development,
        r_dev_s_inv,
        fruit_growth_period_fgp_days,
        gompertz_inflection_m_days,
        f_set_max_fruits_m2_s,
        final_stage_50: StageTransferSnapshotOutput {
            stage_index_j: 50,
            stage_age_days_tj: t_j,
            fruit_count_nj_per_m2: spec.stage50_n_fruits_m2,
            fruit_carbohydrate_cj_mg_m2: spec.stage50_c_mg_m2,
            gompertz_gr_j_mg_fruit_s,
            f_n_in_fruits_m2_s: spec.f_n_49_to_50_fruits_m2_s,
            f_n_out_fruits_m2_s,
            dn_j_dt_fruits_m2_s,
            f_buffer_to_j_mg_m2_s: spec.f_buffer_to_50_mg_m2_s,
            f_c_in_mg_m2_s: spec.f_c_49_to_50_mg_m2_s,
            f_c_out_mg_m2_s,
            r_j_maintenance_mg_m2_s: spec.r_50_maintenance_mg_m2_s,
            dc_j_dt_mg_m2_s,
        },
        d_dm_harvest_dt_mg_m2_s,
        dry_matter_mg_m2: spec.cumulative_dm_harvest_mg_m2,
        fruit_count_m2: spec.cumulative_fruit_count_m2,
        fresh_kg_m2,
        whole_greenhouse_fresh_kg,
        marketable_fresh_kg_m2,
        whole_greenhouse_marketable_kg,
        average_fruit_fresh_mass_g,
        practical_commercial_harvest_reached,
    }
}

fn build_month_entry(
    month_label: &'static str,
    start_fresh_kg_m2: f64,
    end_fresh_kg_m2: f64,
) -> MonthlyHarvestCalendarEntryOutput {
    let monthly_fresh_kg_m2 = end_fresh_kg_m2 - start_fresh_kg_m2;
    let monthly_whole_greenhouse_fresh_kg =
        CULTIVATED_AREA_M2 * monthly_fresh_kg_m2;
    let monthly_whole_greenhouse_marketable_kg =
        DEFAULT_MARKETABLE_FRACTION * monthly_whole_greenhouse_fresh_kg;

    MonthlyHarvestCalendarEntryOutput {
        month_label,
        cumulative_start_fresh_kg_m2: start_fresh_kg_m2,
        cumulative_end_fresh_kg_m2: end_fresh_kg_m2,
        monthly_fresh_kg_m2,
        monthly_whole_greenhouse_fresh_kg,
        monthly_whole_greenhouse_marketable_kg,
    }
}

fn reference_records() -> Vec<PreparedFruitHarvestRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): early trusses setting, zero ripe stage-50 fruit yet
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            t_24_c: 18.2,
            h_development: 0.65,
            stage50_n_fruits_m2: 0.0,
            stage50_c_mg_m2: 0.0,
            f_n_49_to_50_fruits_m2_s: 0.0,
            f_buffer_to_50_mg_m2_s: 0.0,
            f_c_49_to_50_mg_m2_s: 0.0,
            r_50_maintenance_mg_m2_s: 0.0,
            cumulative_dm_harvest_mg_m2: 0.0,
            cumulative_fruit_count_m2: 0.0,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): still pre-harvest phase
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            t_24_c: 18.5,
            h_development: 0.66,
            stage50_n_fruits_m2: 0.0,
            stage50_c_mg_m2: 0.0,
            f_n_49_to_50_fruits_m2_s: 0.0,
            f_buffer_to_50_mg_m2_s: 0.0,
            f_c_49_to_50_mg_m2_s: 0.0,
            r_50_maintenance_mg_m2_s: 0.0,
            cumulative_dm_harvest_mg_m2: 0.0,
            cumulative_fruit_count_m2: 0.0,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): first commercial harvests underway (2.4 kg/m² fresh, 19 fruits/m²)
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            t_24_c: 20.6,
            h_development: 1.00,
            stage50_n_fruits_m2: 0.38,
            stage50_c_mg_m2: 2_650.0,
            f_n_49_to_50_fruits_m2_s: 3.20e-6,
            f_buffer_to_50_mg_m2_s: 0.002_8,
            f_c_49_to_50_mg_m2_s: 0.019_5,
            r_50_maintenance_mg_m2_s: 0.000_2,
            cumulative_dm_harvest_mg_m2: 129_600.0,
            cumulative_fruit_count_m2: 19.0,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): peak mid-season harvest (28.5 kg/m² fresh, 224 fruits/m², Fig. 12)
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            t_24_c: 24.2,
            h_development: 1.00,
            stage50_n_fruits_m2: 0.62,
            stage50_c_mg_m2: 4_350.0,
            f_n_49_to_50_fruits_m2_s: 6.50e-6,
            f_buffer_to_50_mg_m2_s: 0.004_5,
            f_c_49_to_50_mg_m2_s: 0.041_0,
            r_50_maintenance_mg_m2_s: 0.000_4,
            cumulative_dm_harvest_mg_m2: 1_539_000.0,
            cumulative_fruit_count_m2: 224.0,
        }),
    ]
}

pub fn prepare_fruit_harvest_service(
    payload: PrepareFruitHarvestInput,
) -> Result<PrepareFruitHarvestOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_FRUIT_HARVEST_PRESET_ID);

    if preset != DEFAULT_FRUIT_HARVEST_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation fruit harvest preset '{preset}'; \
             supported preset is '{DEFAULT_FRUIT_HARVEST_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedFruitHarvestRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedFruitHarvestRecord> = all_records
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

    Ok(PrepareFruitHarvestOutput {
        preset_id: DEFAULT_FRUIT_HARVEST_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.6, Section 3.7.1, Section 4.3 (Fig. 12) & TiGwerLib 0.7.0 TYM",
        parameters: FruitHarvestParametersOutput {
            cultivar: "Clodano F1 Hybrid (Syngenta)",
            cultivated_area_a_crop_m2: CULTIVATED_AREA_M2,
            num_development_stages: NUM_DEVELOPMENT_STAGES,
            plant_density_per_m2: PLANT_DENSITY_PER_M2,
            c1_dev_rate_s_inv: C1_DEV_RATE_S_INV,
            c2_dev_rate_per_c_s: C2_DEV_RATE_PER_C_S,
            eta_ch2o_to_dm: ETA_CH2O_TO_DM,
            reference_dry_matter_fraction_f_dm: REFERENCE_FRUIT_DRY_MATTER_FRACTION,
            default_marketable_fraction: DEFAULT_MARKETABLE_FRACTION,
            practical_first_harvest_threshold_kg_m2:
            PRACTICAL_FIRST_HARVEST_THRESHOLD_KG_M2,
            marketable_scope_note:
            "Biological fresh weight uses the CTIFL 5.4% dry-matter conversion; marketable harvest \
                 applies an explicit external assumption (92%) until a quality/rejection model is supplied (Section 22).",
        },
        published_yield_validation: PublishedYieldValidationSummaryOutput {
            fresh_yield_rmse_kg_m2: 1.3,
            fresh_yield_mae_kg_m2: 1.1,
            fruit_count_rmse_fruits_m2: 8.7,
            fruit_count_mae_fruits_m2: 6.5,
            season_final_fresh_yield_kg_m2: 49.2,
            season_final_whole_greenhouse_fresh_kg: 49.2 * CULTIVATED_AREA_M2,
            season_final_fruit_count_m2: 385.0,
        },
        records,
        monthly_harvest_progression: [
            build_month_entry("2015-03", 0.0, 3.8),
            build_month_entry("2015-04", 3.8, 10.2),
            build_month_entry("2015-05", 10.2, 17.6),
            build_month_entry("2015-06", 17.6, 25.4),
            build_month_entry("2015-07", 25.4, 33.1),
            build_month_entry("2015-08", 33.1, 40.0),
            build_month_entry("2015-09", 40.0, 45.6),
            build_month_entry("2015-10", 45.6, 49.2),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_transfers_fresh_weight_and_monthly_differences_hold_across_all_records() {
        let output =
            prepare_fruit_harvest_service(PrepareFruitHarvestInput::default())
                .expect("default fruit harvest preset should succeed");

        assert_eq!(output.records.len(), 4);

        let n_dev = f64::from(NUM_DEVELOPMENT_STAGES);

        for record in &output.records {
            // r_dev = c_1 + c_2 * T_24 and FGP = 1 / (86400 * r_dev)
            let expected_r_dev =
                C1_DEV_RATE_S_INV + C2_DEV_RATE_PER_C_S * record.t_24_c;
            assert!((record.r_dev_s_inv - expected_r_dev).abs() < 1e-16);

            let expected_fgp = 1.0 / (SECONDS_PER_DAY * expected_r_dev);
            assert!(
                (record.fruit_growth_period_fgp_days - expected_fgp).abs()
                    < 1e-9
            );

            // Stage 50 transfers
            let st = &record.final_stage_50;
            let expected_fn_out = n_dev
                * expected_r_dev
                * record.h_development
                * st.fruit_count_nj_per_m2;
            let expected_fc_out =
                n_dev * expected_r_dev * st.fruit_carbohydrate_cj_mg_m2;

            assert!((st.f_n_out_fruits_m2_s - expected_fn_out).abs() < 1e-14);
            assert!((st.f_c_out_mg_m2_s - expected_fc_out).abs() < 1e-12);

            // dN_j/dt and dC_j/dt
            assert!(
                (st.dn_j_dt_fruits_m2_s
                    - (st.f_n_in_fruits_m2_s - expected_fn_out))
                    .abs()
                    < 1e-14
            );
            assert!(
                (st.dc_j_dt_mg_m2_s
                    - (st.f_buffer_to_j_mg_m2_s + st.f_c_in_mg_m2_s
                    - expected_fc_out
                    - st.r_j_maintenance_mg_m2_s))
                    .abs()
                    < 1e-12
            );

            // Fresh weight and whole-greenhouse conversion
            let expected_fresh_m2 = record.dry_matter_mg_m2
                / (1e6 * REFERENCE_FRUIT_DRY_MATTER_FRACTION);
            assert!((record.fresh_kg_m2 - expected_fresh_m2).abs() < 1e-9);
            assert!(
                (record.whole_greenhouse_fresh_kg
                    - CULTIVATED_AREA_M2 * expected_fresh_m2)
                    .abs()
                    < 1e-6
            );
            assert!(
                (record.marketable_fresh_kg_m2
                    - DEFAULT_MARKETABLE_FRACTION * expected_fresh_m2)
                    .abs()
                    < 1e-9
            );
        }

        // Monthly harvest progression check: H_month = H_{cumulative,end} - H_{cumulative,start}
        let mut cumulative_sum = 0.0;
        for m in &output.monthly_harvest_progression {
            let expected_diff =
                m.cumulative_end_fresh_kg_m2 - m.cumulative_start_fresh_kg_m2;
            assert!((m.monthly_fresh_kg_m2 - expected_diff).abs() < 1e-12);
            cumulative_sum += m.monthly_fresh_kg_m2;
        }
        assert!((cumulative_sum - 49.2).abs() < 1e-9);
    }
}