// Fixed demonstration physical data; not summaries of a live simulation run.
use serde::{Deserialize, Serialize};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";
/// User-supplied seasonal estimates, KES/kg, January through December.
/// These are assumptions, not verified market observations.
pub const MONTHLY_TOMATO_PRICES_KES_PER_KG: [f64; 12] = [
    75.0, 85.0, 95.0, 103.0, 108.0, 65.0,
    45.0, 40.0, 55.0, 65.0, 73.0, 83.0,
];

pub fn estimated_monthly_price_kes(month_label: &str) -> Result<f64, String> {
    let bytes = month_label.as_bytes();
    if bytes.len() != 7 || bytes[4] != b'-'
        || !bytes[..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..].iter().all(u8::is_ascii_digit)
    {
        return Err(format!("Invalid month label '{}': expected YYYY-MM.", month_label));
    }
    let month = month_label[5..].parse::<usize>()
        .map_err(|_| "Invalid month number.".to_string())?;
    if !(1..=12).contains(&month) {
        return Err("Month number must be between 01 and 12.".to_string());
    }
    Ok(MONTHLY_TOMATO_PRICES_KES_PER_KG[month - 1])
}

const FLOOR_AREA_M2: f64 = 1_036.8;
const CULTIVATED_AREA_M2: f64 = 943.488;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummarizeSimulationInput {
    pub preset_id: Option<String>,
    pub period_filter: Option<String>,
    /// Optional user-supplied market price per kg of marketable fruit (e.g. EUR/kg or KES/kg)
    /// so the user/frontend multiplies physically simulated harvest by their live market price.
    pub market_price_per_kg: Option<f64>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeSimulationOutput {
    pub preset_id: String,
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub climate_suitability_records: Vec<ClimateSuitabilityPeriodRecord>,
    pub seasonal_climate_suitability: SeasonalClimateSuitabilitySummary,
    pub water_planning_summary: WaterPlanningSummary,
    pub harvest_and_revenue_summary: HarvestAndRevenueSummary,
    pub crop_cycle_guidance: CropCycleGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateSuitabilityPeriodRecord {
    pub period_label: String,
    pub timestamp_utc: String,
    pub duration_hours: f64,
    pub air_temperature_c: f64,
    pub target_min_temperature_c: f64,
    pub target_max_temperature_c: f64,
    pub within_target_indicator: f64,
    pub heat_exceedance_c: f64,
    pub cold_deficit_c: f64,
    pub heat_exposure_degree_hours_degc_h: f64,
    pub cold_exposure_degree_hours_degc_h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalClimateSuitabilitySummary {
    pub total_evaluated_hours: f64,
    pub hours_within_target_range: f64,
    pub fraction_within_target_f_within: f64,
    pub heat_exposure_degree_hours_d_hot_degc_h: f64,
    pub cold_exposure_degree_hours_d_cold_degc_h: f64,
    pub provenance_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterPlanningSummary {
    pub gross_recirculated_irrigation_m3: f64,
    pub crop_transpiration_w_transpiration_m3: f64,
    pub crop_biomass_water_retention_w_biomass_m3: f64,
    pub discarded_drainage_w_discarded_drainage_m3: f64,
    pub other_system_losses_w_other_losses_m3: f64,
    pub storage_change_delta_w_storage_m3: f64,
    pub usable_condensation_and_drain_recovery_w_usable_recovery_m3: f64,
    pub net_fresh_water_intake_w_fresh_m3: f64,
    pub net_fresh_water_per_m2_floor_l_m2: f64,
    pub net_fresh_water_per_m2_cultivated_l_m2: f64,
    pub recirculation_savings_fraction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyHarvestRecord {
    pub applied_price_per_kg: f64,
    pub price_source: String,
    pub month_label: String,
    pub cumulative_harvest_end_kg: f64,
    pub monthly_marketable_harvest_kg: f64,
    pub monthly_harvest_kg_m2_floor: f64,
    pub monthly_harvest_kg_m2_cultivated: f64,
    pub expected_monthly_revenue: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestAndRevenueSummary {
    pub pricing_mode: String,
    pub harvest_weighted_price_per_kg: Option<f64>,
    pub price_assumptions_note: String,
    pub total_expected_marketable_harvest_kg: f64,
    pub total_harvest_kg_m2_floor: f64,
    pub total_harvest_kg_m2_cultivated: f64,
    pub currency_code: String,
    pub user_market_price_per_kg: Option<f64>,
    pub expected_gross_revenue_total: Option<f64>,
    pub revenue_calculation_formula: String,
    pub monthly_harvest_calendar: Vec<MonthlyHarvestRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropCycleGuidance {
    pub crop_species_and_cultivar_type: String,
    pub cycle_duration_days: u32,
    pub vegetative_lead_time_to_first_harvest_days: u32,
    pub active_harvest_window_days: u32,
    pub initial_stem_density_stems_m2: f64,
    pub peak_stem_density_stems_m2: f64,
    pub planning_horizon_recommendation: String,
}

struct PeriodClimateSpec {
    label: &'static str,
    timestamp_utc: &'static str,
    duration_h: f64,
    t_air_c: f64,
    t_min_c: f64,
    t_max_c: f64,
}

const PERIOD_SPECS: [PeriodClimateSpec; 4] = [
    PeriodClimateSpec {
        label: "winter_night",
        timestamp_utc: "2015-01-11T02:00:00Z",
        duration_h: 1.0,
        t_air_c: 17.2,
        t_min_c: 16.5,
        t_max_c: 20.0,
    },
    PeriodClimateSpec {
        label: "winter_noon",
        timestamp_utc: "2015-01-11T12:00:00Z",
        duration_h: 1.0,
        t_air_c: 20.4,
        t_min_c: 18.5,
        t_max_c: 24.0,
    },
    PeriodClimateSpec {
        label: "equinox_noon",
        timestamp_utc: "2015-03-20T12:00:00Z",
        duration_h: 1.0,
        t_air_c: 23.6,
        t_min_c: 19.0,
        t_max_c: 26.0,
    },
    PeriodClimateSpec {
        label: "summer_afternoon",
        timestamp_utc: "2015-07-16T13:00:00Z",
        duration_h: 1.0,
        t_air_c: 31.2,
        t_min_c: 19.0,
        t_max_c: 28.0,
    },
];

const MONTHLY_CUMULATIVE_HARVEST_KG: [(&str, f64); 11] = [
    ("2014-12", 0.0),
    ("2015-01", 0.0),
    ("2015-02", 1_420.0),
    ("2015-03", 5_180.0),
    ("2015-04", 10_640.0),
    ("2015-05", 17_490.0),
    ("2015-06", 24_980.0),
    ("2015-07", 32_150.0),
    ("2015-08", 38_420.0),
    ("2015-09", 43_510.0),
    ("2015-10", 47_085.05),
];

fn evaluate_climate_period(spec: &PeriodClimateSpec) -> ClimateSuitabilityPeriodRecord {
    let within = if spec.t_air_c >= spec.t_min_c && spec.t_air_c <= spec.t_max_c {
        1.0
    } else {
        0.0
    };
    let heat_ex_c = (spec.t_air_c - spec.t_max_c).max(0.0);
    let cold_def_c = (spec.t_min_c - spec.t_air_c).max(0.0);
    ClimateSuitabilityPeriodRecord {
        period_label: spec.label.to_string(),
        timestamp_utc: spec.timestamp_utc.to_string(),
        duration_hours: spec.duration_h,
        air_temperature_c: spec.t_air_c,
        target_min_temperature_c: spec.t_min_c,
        target_max_temperature_c: spec.t_max_c,
        within_target_indicator: within,
        heat_exceedance_c: heat_ex_c,
        cold_deficit_c: cold_def_c,
        heat_exposure_degree_hours_degc_h: heat_ex_c * spec.duration_h,
        cold_exposure_degree_hours_degc_h: cold_def_c * spec.duration_h,
    }
}

pub fn summarize_simulation_service(
    input: SummarizeSimulationInput,
) -> Result<SummarizeSimulationOutput, String> {
    let preset_id = input
        .preset_id
        .unwrap_or_else(|| DEFAULT_PRESET_ID.to_string());
    if preset_id != DEFAULT_PRESET_ID {
        return Err(format!(
            "Unsupported preset_id '{}'. Expected '{}'.",
            preset_id, DEFAULT_PRESET_ID
        ));
    }

    if let Some(price) = input.market_price_per_kg {
        if !price.is_finite() || price < 0.0 {
            return Err("market_price_per_kg must be a non-negative finite number.".to_string());
        }
    }

    let mut climate_records: Vec<ClimateSuitabilityPeriodRecord> =
        PERIOD_SPECS.iter().map(evaluate_climate_period).collect();

    if let Some(ref filter) = input.period_filter {
        climate_records.retain(|r| &r.period_label == filter);
        if climate_records.is_empty() {
            return Err(format!(
                "Unknown period_filter '{}'. Valid values: winter_night, winter_noon, equinox_noon, summer_afternoon.",
                filter
            ));
        }
    }

    let total_evaluated_hours = 7_992.0;
    let hours_within_target_range = 7_144.848;
    let seasonal_climate_suitability = SeasonalClimateSuitabilitySummary {
        total_evaluated_hours,
        hours_within_target_range,
        fraction_within_target_f_within: hours_within_target_range / total_evaluated_hours,
        heat_exposure_degree_hours_d_hot_degc_h: 1_184.6,
        cold_exposure_degree_hours_d_cold_degc_h: 92.4,
        provenance_note: "Fixed demonstration seasonal indicators; not calculated from the four sample records or a live simulation run.".to_string(),
    };

    // Whole-system water balance:
    // W_fresh = W_transpiration + W_biomass + W_discarded_drainage + W_other_losses + delta_W_storage - W_usable_recovery
    let w_transpiration_m3 = 712.4;
    let w_biomass_m3 = 44.7;
    let w_discarded_drainage_m3 = 38.2;
    let w_other_losses_m3 = 14.5;
    let delta_w_storage_m3 = 2.0;
    let w_usable_recovery_m3 = 218.6; // recovered closed-loop drain + roof condensation recovery
    let gross_recirculated_irrigation_m3 = 985.7;
    let w_fresh_m3 = w_transpiration_m3
        + w_biomass_m3
        + w_discarded_drainage_m3
        + w_other_losses_m3
        + delta_w_storage_m3
        - w_usable_recovery_m3;

    let water_planning_summary = WaterPlanningSummary {
        gross_recirculated_irrigation_m3,
        crop_transpiration_w_transpiration_m3: w_transpiration_m3,
        crop_biomass_water_retention_w_biomass_m3: w_biomass_m3,
        discarded_drainage_w_discarded_drainage_m3: w_discarded_drainage_m3,
        other_system_losses_w_other_losses_m3: w_other_losses_m3,
        storage_change_delta_w_storage_m3: delta_w_storage_m3,
        usable_condensation_and_drain_recovery_w_usable_recovery_m3: w_usable_recovery_m3,
        net_fresh_water_intake_w_fresh_m3: w_fresh_m3,
        net_fresh_water_per_m2_floor_l_m2: (w_fresh_m3 * 1_000.0) / FLOOR_AREA_M2,
        net_fresh_water_per_m2_cultivated_l_m2: (w_fresh_m3 * 1_000.0) / CULTIVATED_AREA_M2,
        recirculation_savings_fraction: (gross_recirculated_irrigation_m3 - w_fresh_m3)
            / gross_recirculated_irrigation_m3,
    };

    let currency_code = input.currency_code.unwrap_or_else(|| "KES".to_string());
    if input.market_price_per_kg.is_none() && currency_code != "KES" {
        return Err("Constant monthly prices are in KES. Use KES or supply a flat price in the requested currency.".to_string());
    }
    let mut prev_cum_kg = 0.0;
    let mut total_revenue = 0.0;
    let mut monthly_harvest_calendar = Vec::with_capacity(MONTHLY_CUMULATIVE_HARVEST_KG.len());
    for (month_label, cum_kg) in MONTHLY_CUMULATIVE_HARVEST_KG {
        let delta_kg = cum_kg - prev_cum_kg;
        prev_cum_kg = cum_kg;
        let price = match input.market_price_per_kg {
            Some(flat_price) => flat_price,
            None => estimated_monthly_price_kes(month_label)?,
        };
        let revenue = delta_kg * price;
        if !revenue.is_finite() || !(total_revenue + revenue).is_finite() {
            return Err("Revenue overflow: price is too large.".to_string());
        }
        total_revenue += revenue;
        monthly_harvest_calendar.push(MonthlyHarvestRecord {
            applied_price_per_kg: price,
            price_source: if input.market_price_per_kg.is_some() {
                "user_supplied_flat_override".to_string()
            } else {
                "user_supplied_constant_seasonal_estimate".to_string()
            },
            month_label: month_label.to_string(),
            cumulative_harvest_end_kg: cum_kg,
            monthly_marketable_harvest_kg: delta_kg,
            monthly_harvest_kg_m2_floor: delta_kg / FLOOR_AREA_M2,
            monthly_harvest_kg_m2_cultivated: delta_kg / CULTIVATED_AREA_M2,
            expected_monthly_revenue: Some(revenue),
        });
    }

    let total_harvest_kg = prev_cum_kg;
    let harvest_and_revenue_summary = HarvestAndRevenueSummary {
        pricing_mode: if input.market_price_per_kg.is_some() {
            "flat_override".to_string()
        } else {
            "constant_monthly_estimates".to_string()
        },
        harvest_weighted_price_per_kg: if total_harvest_kg > 0.0 {
            Some(total_revenue / total_harvest_kg)
        } else { None },
        price_assumptions_note: "Monthly KES prices are user-supplied seasonal assumptions repeated each year; market, grade and farmgate/wholesale basis have not been verified. Physical harvest values are fixed demonstration data.".to_string(),
        total_expected_marketable_harvest_kg: total_harvest_kg,
        total_harvest_kg_m2_floor: total_harvest_kg / FLOOR_AREA_M2,
        total_harvest_kg_m2_cultivated: total_harvest_kg / CULTIVATED_AREA_M2,
        currency_code,
        user_market_price_per_kg: input.market_price_per_kg,
        expected_gross_revenue_total: Some(total_revenue),
        revenue_calculation_formula:
        "expected_gross_revenue = sum(monthly_marketable_harvest_kg * applied_price_per_kg)"
            .to_string(),
        monthly_harvest_calendar,
    };

    let crop_cycle_guidance = CropCycleGuidance {
        crop_species_and_cultivar_type: "Indeterminate high-wire truss tomato (Solanum lycopersicum)".to_string(),
        cycle_duration_days: 333,
        vegetative_lead_time_to_first_harvest_days: 68,
        active_harvest_window_days: 265,
        initial_stem_density_stems_m2: 2.5,
        peak_stem_density_stems_m2: 3.75,
        planning_horizon_recommendation: "Demonstration assumption: 68 days before first harvest. Sales start with harvest; positive cash flow requires a separate monthly expense calculation.".to_string(),
    };

    Ok(SummarizeSimulationOutput {
        preset_id,
        floor_area_m2: FLOOR_AREA_M2,
        cultivated_area_m2: CULTIVATED_AREA_M2,
        climate_suitability_records: climate_records,
        seasonal_climate_suitability,
        water_planning_summary,
        harvest_and_revenue_summary,
        crop_cycle_guidance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_balance_and_harvest_calendar_conservation() {
        let out = summarize_simulation_service(SummarizeSimulationInput {
            market_price_per_kg: Some(120.0),
            currency_code: Some("KES".to_string()),
            ..Default::default()
        })
            .expect("summarize_simulation_service should succeed");

        let w = &out.water_planning_summary;
        let expected_w_fresh = w.crop_transpiration_w_transpiration_m3
            + w.crop_biomass_water_retention_w_biomass_m3
            + w.discarded_drainage_w_discarded_drainage_m3
            + w.other_system_losses_w_other_losses_m3
            + w.storage_change_delta_w_storage_m3
            - w.usable_condensation_and_drain_recovery_w_usable_recovery_m3;
        assert!((w.net_fresh_water_intake_w_fresh_m3 - expected_w_fresh).abs() < 1e-12);
        assert!(w.gross_recirculated_irrigation_m3 > w.net_fresh_water_intake_w_fresh_m3);

        let sum_monthly_kg: f64 = out
            .harvest_and_revenue_summary
            .monthly_harvest_calendar
            .iter()
            .map(|m| m.monthly_marketable_harvest_kg)
            .sum();
        assert!(
            (sum_monthly_kg - out.harvest_and_revenue_summary.total_expected_marketable_harvest_kg)
                .abs()
                < 1e-9
        );

        let expected_rev =
            out.harvest_and_revenue_summary.total_expected_marketable_harvest_kg * 120.0;
        assert!(
            (out.harvest_and_revenue_summary
                .expected_gross_revenue_total
                .unwrap()
                - expected_rev)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_degree_hours_summer_exceedance() {
        let out = summarize_simulation_service(SummarizeSimulationInput {
            period_filter: Some("summer_afternoon".to_string()),
            ..Default::default()
        })
            .unwrap();
        let rec = &out.climate_suitability_records[0];
        assert_eq!(rec.within_target_indicator, 0.0);
        assert!((rec.heat_exposure_degree_hours_degc_h - (31.2 - 28.0)).abs() < 1e-12);
    }
    #[test]
    fn monthly_estimates_match_expected_revenue() {
        let out = summarize_simulation_service(SummarizeSimulationInput::default()).unwrap();
        let h = out.harvest_and_revenue_summary;
        assert!((h.expected_gross_revenue_total.unwrap() - 3_352_708.25).abs() < 1e-6);
        let feb = &h.monthly_harvest_calendar[2];
        assert_eq!(feb.applied_price_per_kg, 85.0);
        assert_eq!(feb.expected_monthly_revenue, Some(120_700.0));
        assert_eq!(h.monthly_harvest_calendar[0].expected_monthly_revenue, Some(0.0));
        let sum: f64 = h.monthly_harvest_calendar.iter()
            .map(|m| m.expected_monthly_revenue.unwrap()).sum();
        assert!((sum - h.expected_gross_revenue_total.unwrap()).abs() < 1e-6);
    }

    #[test]
    fn seasonal_prices_repeat_by_calendar_month() {
        assert_eq!(estimated_monthly_price_kes("2026-05").unwrap(), 108.0);
        assert_eq!(estimated_monthly_price_kes("2027-05").unwrap(), 108.0);
        for bad in ["2026-00", "2026-13", "2026-1", "abcd-01"] {
            assert!(estimated_monthly_price_kes(bad).is_err());
        }
    }

    #[test]
    fn currency_and_price_validation() {
        assert!(summarize_simulation_service(SummarizeSimulationInput {
            currency_code: Some("EUR".to_string()),
            ..Default::default()
        }).is_err());
        for price in [-1.0, f64::NAN, f64::INFINITY, f64::MAX] {
            assert!(summarize_simulation_service(SummarizeSimulationInput {
                market_price_per_kg: Some(price),
                ..Default::default()
            }).is_err());
        }
    }

}