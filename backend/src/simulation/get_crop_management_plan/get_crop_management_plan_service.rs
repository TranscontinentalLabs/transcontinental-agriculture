use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CROP_MANAGEMENT_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// Specific leaf area SLA = 1 / 37_500 m²_leaf per mg CH2O (~2.6667e-5 m²/mg),
/// matching Model 20's C_leaf values (41,250 mg/m² -> LAI 1.1; 120,000 -> 3.2; 172,500 -> 4.6).
pub const SLA_M2_LEAF_PER_MG_CH2O: f64 = 1.0 / 37_500.0;

/// Finite pruning duration delta_t_prune (1 day = 86,400 s) over which C_remove is extracted.
pub const PRUNING_WINDOW_DURATION_S: f64 = 86_400.0;

pub const CULTIVATED_AREA_M2: f64 = 943.488;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetCropManagementPlanInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–20 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedAgronomicScheduleOutput {
    pub cultivar: &'static str,
    pub row_count: u8,
    pub growing_gutter_elevation_m: f64,
    pub mature_plant_apex_height_m: f64,
    pub sla_m2_leaf_per_mg_ch2o: f64,
    pub pruning_window_duration_s: f64,
    pub recorded_pruning_end_date_iso: &'static str,
    pub recorded_pruning_end_day_after_planting: u16,
    pub post_august_5_assumed_pruning_cadence: &'static str,
    pub post_august_5_target_lai_m2_m2: f64,
    pub topping_date_iso: &'static str,
    pub crop_height_schedule_warning: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCropManagementRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Measured/scheduled crop height (m) for view-factor geometry (Model 13).
    pub scheduled_crop_height_m: f64,

    /// Stored leaf and stem carbohydrate masses, mg CH2O/m²_cultivated.
    pub c_leaf_mg_m2: f64,
    pub c_stem_mg_m2: f64,

    /// LAI = SLA * C_leaf, m²_leaf/m²_cultivated.
    pub leaf_area_index: f64,

    /// Pruning target LAI_target (m²/m²) and whether a pruning event is active at this timestamp.
    pub pruning_event_active: bool,
    pub lai_target_m2_m2: f64,

    /// C_remove = max(0, C_leaf - LAI_target / SLA), mg CH2O/m²_cultivated.
    pub c_remove_at_pruning_mg_m2: f64,

    /// Instantaneous pruning carbohydrate removal rate F_pruning, mg CH2O/(m²·s).
    pub f_pruning_mg_m2_s: f64,

    /// Whole-compartment pruned carbohydrate mass for this event (C_remove * A_crop), kg CH2O.
    pub whole_crop_pruned_carbohydrate_kg: f64,

    /// Allocation and maintenance respiration flows from Model 20, mg CH2O/(m²·s).
    pub f_leaf_mg_m2_s: f64,
    pub r_leaf_mg_m2_s: f64,
    pub f_stem_mg_m2_s: f64,
    pub r_stem_mg_m2_s: f64,

    /// Organ mass derivatives dC_leaf/dt and dC_stem/dt, mg CH2O/(m²·s).
    pub dc_leaf_dt_mg_m2_s: f64,
    pub dc_stem_dt_mg_m2_s: f64,

    /// Leaf area derivative dLAI/dt = SLA * dC_leaf/dt, m²/(m²·s) (and per day).
    pub dlai_dt_per_s: f64,
    pub dlai_dt_per_day: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetCropManagementPlanOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub schedule: PublishedAgronomicScheduleOutput,
    pub records: Vec<PreparedCropManagementRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    scheduled_crop_height_m: f64,
    c_leaf_mg_m2: f64,
    c_stem_mg_m2: f64,
    pruning_event_active: bool,
    lai_target_m2_m2: f64,
    f_leaf_mg_m2_s: f64,
    r_leaf_mg_m2_s: f64,
    f_stem_mg_m2_s: f64,
    r_stem_mg_m2_s: f64,
}

fn evaluate_record(spec: RecordSpec) -> PreparedCropManagementRecord {
    let leaf_area_index = SLA_M2_LEAF_PER_MG_CH2O * spec.c_leaf_mg_m2;

    let target_c_leaf_mg_m2 = spec.lai_target_m2_m2 / SLA_M2_LEAF_PER_MG_CH2O;
    let c_remove_at_pruning_mg_m2 =
        (spec.c_leaf_mg_m2 - target_c_leaf_mg_m2).max(0.0);

    let f_pruning_mg_m2_s = if spec.pruning_event_active {
        c_remove_at_pruning_mg_m2 / PRUNING_WINDOW_DURATION_S
    } else {
        0.0
    };

    let whole_crop_pruned_carbohydrate_kg = if spec.pruning_event_active {
        (c_remove_at_pruning_mg_m2 * CULTIVATED_AREA_M2) * 1e-6
    } else {
        0.0
    };

    let dc_leaf_dt_mg_m2_s =
        spec.f_leaf_mg_m2_s - spec.r_leaf_mg_m2_s - f_pruning_mg_m2_s;
    let dc_stem_dt_mg_m2_s = spec.f_stem_mg_m2_s - spec.r_stem_mg_m2_s;

    let dlai_dt_per_s = SLA_M2_LEAF_PER_MG_CH2O * dc_leaf_dt_mg_m2_s;
    let dlai_dt_per_day = dlai_dt_per_s * 86_400.0;

    PreparedCropManagementRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        scheduled_crop_height_m: spec.scheduled_crop_height_m,
        c_leaf_mg_m2: spec.c_leaf_mg_m2,
        c_stem_mg_m2: spec.c_stem_mg_m2,
        leaf_area_index,
        pruning_event_active: spec.pruning_event_active,
        lai_target_m2_m2: spec.lai_target_m2_m2,
        c_remove_at_pruning_mg_m2,
        f_pruning_mg_m2_s,
        whole_crop_pruned_carbohydrate_kg,
        f_leaf_mg_m2_s: spec.f_leaf_mg_m2_s,
        r_leaf_mg_m2_s: spec.r_leaf_mg_m2_s,
        f_stem_mg_m2_s: spec.f_stem_mg_m2_s,
        r_stem_mg_m2_s: spec.r_stem_mg_m2_s,
        dc_leaf_dt_mg_m2_s,
        dc_stem_dt_mg_m2_s,
        dlai_dt_per_s,
        dlai_dt_per_day,
    }
}

fn reference_records() -> Vec<PreparedCropManagementRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): young 1.0 m crop (LAI = 1.1), before first pruning event
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            scheduled_crop_height_m: 1.0,
            c_leaf_mg_m2: 41_250.0,
            c_stem_mg_m2: 26_000.0,
            pruning_event_active: false,
            lai_target_m2_m2: 2.8,
            f_leaf_mg_m2_s: 0.075_449_001_6,
            r_leaf_mg_m2_s: 0.008_895_402_1,
            f_stem_mg_m2_s: 0.059_565_001_3,
            r_stem_mg_m2_s: 0.002_384_811_6,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): young 1.0 m crop (LAI = 1.1), expanding toward canopy closure
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            scheduled_crop_height_m: 1.0,
            c_leaf_mg_m2: 41_250.0,
            c_stem_mg_m2: 26_400.0,
            pruning_event_active: false,
            lai_target_m2_m2: 2.8,
            f_leaf_mg_m2_s: 0.080_177_979_0,
            r_leaf_mg_m2_s: 0.009_092_141_2,
            f_stem_mg_m2_s: 0.063_298_404_5,
            r_stem_mg_m2_s: 0.002_473_109_8,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): 2.8 m crop (LAI = 3.2), between discrete spring prunings
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            scheduled_crop_height_m: 2.8,
            c_leaf_mg_m2: 120_000.0,
            c_stem_mg_m2: 95_000.0,
            pruning_event_active: false,
            lai_target_m2_m2: 3.0,
            f_leaf_mg_m2_s: 0.094_809_354_0,
            r_leaf_mg_m2_s: 0.029_192_855_4,
            f_stem_mg_m2_s: 0.074_849_490_0,
            r_stem_mg_m2_s: 0.009_796_210_5,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): mature 3.9 m apex crop (LAI = 4.6) undergoing discrete pruning to LAI_target = 4.5
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            scheduled_crop_height_m: 3.9,
            c_leaf_mg_m2: 172_500.0,
            c_stem_mg_m2: 168_000.0,
            pruning_event_active: true,
            lai_target_m2_m2: 4.5,
            f_leaf_mg_m2_s: 0.108_338_803_2,
            r_leaf_mg_m2_s: 0.052_891_104_0,
            f_stem_mg_m2_s: 0.085_530_634_1,
            r_stem_mg_m2_s: 0.021_845_021_0,
        }),
    ]
}

pub fn get_crop_management_plan_service(
    payload: GetCropManagementPlanInput,
) -> Result<GetCropManagementPlanOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CROP_MANAGEMENT_PRESET_ID);

    if preset != DEFAULT_CROP_MANAGEMENT_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation crop management preset '{preset}'; \
             supported preset is '{DEFAULT_CROP_MANAGEMENT_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCropManagementRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCropManagementRecord> = all_records
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

    Ok(GetCropManagementPlanOutput {
        preset_id: DEFAULT_CROP_MANAGEMENT_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.2, Section 2.3.3, Section 3.6, Section 4.3 (Fig. 11) & TiGwerLib 0.7.0 TYM",
        schedule: PublishedAgronomicScheduleOutput {
            cultivar: "Clodano F1 Hybrid (Syngenta)",
            row_count: 15,
            growing_gutter_elevation_m: 1.1,
            mature_plant_apex_height_m: 3.9,
            sla_m2_leaf_per_mg_ch2o: SLA_M2_LEAF_PER_MG_CH2O,
            pruning_window_duration_s: PRUNING_WINDOW_DURATION_S,
            recorded_pruning_end_date_iso: "2015-08-05",
            recorded_pruning_end_day_after_planting: 246,
            post_august_5_assumed_pruning_cadence: "weekly (starting calendar week 33)",
            post_august_5_target_lai_m2_m2: 4.5,
            topping_date_iso: "2015-09-02",
            crop_height_schedule_warning:
            "Predicted LAI does not automatically determine crop height for longwave view factors; \
                 an explicit trellising/height schedule up to the 3.9 m apex is required (Section 21).",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lai_pruning_removal_and_organ_balances_hold_across_all_records() {
        let output = get_crop_management_plan_service(
            GetCropManagementPlanInput::default(),
        )
            .expect("default crop management preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // LAI = SLA * C_leaf
            let expected_lai = SLA_M2_LEAF_PER_MG_CH2O * record.c_leaf_mg_m2;
            assert!((record.leaf_area_index - expected_lai).abs() < 1e-12);

            // C_remove = max(0, C_leaf - LAI_target / SLA)
            let expected_c_remove = (record.c_leaf_mg_m2
                - record.lai_target_m2_m2 / SLA_M2_LEAF_PER_MG_CH2O)
                .max(0.0);
            assert!(
                (record.c_remove_at_pruning_mg_m2 - expected_c_remove).abs()
                    < 1e-9
            );

            // dC_leaf/dt = F_leaf - R_leaf - F_pruning
            let expected_dc_leaf = record.f_leaf_mg_m2_s
                - record.r_leaf_mg_m2_s
                - record.f_pruning_mg_m2_s;
            assert!(
                (record.dc_leaf_dt_mg_m2_s - expected_dc_leaf).abs() < 1e-12
            );

            // dC_stem/dt = F_stem - R_stem
            let expected_dc_stem =
                record.f_stem_mg_m2_s - record.r_stem_mg_m2_s;
            assert!(
                (record.dc_stem_dt_mg_m2_s - expected_dc_stem).abs() < 1e-12
            );
        }

        // Summer afternoon has active pruning from LAI 4.6 toward target 4.5 (C_remove = 3,750 mg/m²).
        let summer = &output.records[3];
        assert!(summer.pruning_event_active);
        assert!((summer.c_remove_at_pruning_mg_m2 - 3_750.0).abs() < 1e-9);
        assert!(summer.dc_leaf_dt_mg_m2_s > 0.0);
    }
}