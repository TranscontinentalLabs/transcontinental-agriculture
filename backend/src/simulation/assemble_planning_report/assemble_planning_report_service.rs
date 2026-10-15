use serde::{Deserialize, Serialize};

use crate::simulation::{
    calculate_energy::calculate_energy_service::{
        calculate_energy_service, CalculateEnergyInput, CalculateEnergyOutput,
    },
    estimate_construction::estimate_construction_service::{
        estimate_construction_service, EstimateConstructionInput, EstimateConstructionOutput,
    },
    lookup_solar_package::lookup_solar_package_service::{
        lookup_solar_package_service, LookupSolarPackageInput, LookupSolarPackageOutput,
    },
    price_energy::price_energy_service::{
        price_energy_service, PriceEnergyInput, PriceEnergyOutput,
    },
    summarize_simulation::summarize_simulation_service::{
        summarize_simulation_service, SummarizeSimulationInput, SummarizeSimulationOutput,
    },
};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblePlanningReportInput {
    pub preset_id: Option<String>,
    pub period_filter: Option<String>,
    /// "kplc_kenya_ci1_tou" (default) or "edf_france_pro_tou"
    pub tariff_id: Option<String>,
    /// "kenya_witeithie_2026_kes" (default) or "europe_venlo_reference_eur"
    pub cost_book_id: Option<String>,
    /// "hybrid_critical_load_backup" (default), "grid_tied_daytime_offset", or "off_grid_full_autonomy"
    pub solar_preference_mode: Option<String>,
    pub required_backup_hours: Option<f64>,
    /// User-supplied market price per kg of marketable tomato (e.g. 130.0 KES/kg)
    pub market_price_per_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblePlanningReportOutput {
    pub preset_id: String,
    pub currency_code: String,
    pub executive_feasibility_highlights: ExecutiveFeasibilityHighlights,
    pub simulation_summary: SummarizeSimulationOutput,
    pub energy_calculation: CalculateEnergyOutput,
    pub tariff_and_running_cost: PriceEnergyOutput,
    pub solar_package_recommendation: LookupSolarPackageOutput,
    pub construction_boq_estimate: EstimateConstructionOutput,
    pub provenance_and_scope_statement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveFeasibilityHighlights {
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub cycle_duration_days: u32,
    pub expected_marketable_harvest_kg: f64,
    pub yield_kg_m2_cultivated: f64,
    pub climate_time_within_target_fraction: f64,
    pub net_fresh_water_intake_m3: f64,
    pub water_saved_by_recirculation_and_condensation_m3: f64,
    pub coincident_peak_electrical_demand_kw: f64,
    pub average_daily_electrical_use_kwh_day: f64,
    pub construction_capex_total: f64,
    pub selected_solar_package_capex: f64,
    pub combined_capex_construction_plus_solar: f64,
    pub cost_of_running_opex_per_cycle: f64,
    pub break_even_farmgate_price_per_kg: f64,
    pub user_market_price_per_kg: Option<f64>,
    pub expected_gross_revenue_per_cycle: Option<f64>,
    pub expected_net_operating_income_per_cycle: Option<f64>,
    pub simple_payback_cycles_construction_only: Option<f64>,
    pub simple_payback_cycles_with_solar: Option<f64>,
}

pub fn assemble_planning_report_service(
    input: AssemblePlanningReportInput,
) -> Result<AssemblePlanningReportOutput, String> {
    let preset_id = input
        .preset_id
        .unwrap_or_else(|| DEFAULT_PRESET_ID.to_string());

    let tariff_id = input
        .tariff_id
        .unwrap_or_else(|| "kplc_kenya_ci1_tou".to_string());

    let cost_book_id = input.cost_book_id.unwrap_or_else(|| {
        if tariff_id == "edf_france_pro_tou" {
            "europe_venlo_reference_eur".to_string()
        } else {
            "kenya_witeithie_2026_kes".to_string()
        }
    });

    let currency_code = if tariff_id == "edf_france_pro_tou" {
        "EUR".to_string()
    } else {
        "KES".to_string()
    };

    let simulation_summary = summarize_simulation_service(SummarizeSimulationInput {
        preset_id: Some(preset_id.clone()),
        period_filter: input.period_filter.clone(),
        market_price_per_kg: input.market_price_per_kg,
        currency_code: Some(currency_code.clone()),
    })?;

    let energy_calculation = calculate_energy_service(CalculateEnergyInput {
        preset_id: Some(preset_id.clone()),
        period_filter: input.period_filter,
    })?;

    let mut tariff_and_running_cost = price_energy_service(PriceEnergyInput {
        preset_id: Some(preset_id.clone()),
        tariff_id: Some(tariff_id),
        market_price_per_kg: input.market_price_per_kg,
    })?;

    let solar_package_recommendation = lookup_solar_package_service(LookupSolarPackageInput {
        preset_id: Some(preset_id.clone()),
        preference_mode: input.solar_preference_mode,
        required_backup_hours: input.required_backup_hours,
    })?;

    let construction_boq_estimate = estimate_construction_service(EstimateConstructionInput {
        preset_id: Some(preset_id.clone()),
        cost_book_id: Some(cost_book_id),
    })?;

    let solar_capex = if currency_code == "EUR" {
        solar_package_recommendation
            .selected_package
            .indicative_installed_cost_eur
    } else {
        solar_package_recommendation
            .selected_package
            .indicative_installed_cost_kes
    };

    let construction_capex = construction_boq_estimate.c_construction_total;
    let combined_capex = construction_capex + solar_capex;
    let opex = tariff_and_running_cost
        .cost_of_running_opex_breakdown
        .c_running_total_opex;
    // The harvest summary owns revenue calculation: constant monthly estimates
    // by default, or the optional user-supplied flat-price override.
    let gross_revenue = simulation_summary
        .harvest_and_revenue_summary
        .expected_gross_revenue_total;
    let net_income = gross_revenue.map(|revenue| revenue - opex);
    {
        let economics =
            &mut tariff_and_running_cost.unit_economics_and_margin_summary;
        economics.expected_gross_revenue = gross_revenue;
        economics.expected_net_operating_income_ebitda = net_income;
        economics.revenue_and_margin_formula =
            "net_operating_income = sum(monthly_harvest_kg * applied_monthly_price) - c_running_total_opex"
                .to_string();
    }

    let payback_construction = net_income.and_then(|net| {
        if net > 0.0 {
            Some(construction_capex / net)
        } else {
            None
        }
    });
    let payback_with_solar = net_income.and_then(|net| {
        if net > 0.0 {
            Some(combined_capex / net)
        } else {
            None
        }
    });

    let w = &simulation_summary.water_planning_summary;
    let water_saved_m3 =
        w.gross_recirculated_irrigation_m3 - w.net_fresh_water_intake_w_fresh_m3;

    let executive_feasibility_highlights = ExecutiveFeasibilityHighlights {
        floor_area_m2: simulation_summary.floor_area_m2,
        cultivated_area_m2: simulation_summary.cultivated_area_m2,
        cycle_duration_days: simulation_summary.crop_cycle_guidance.cycle_duration_days,
        expected_marketable_harvest_kg: simulation_summary
            .harvest_and_revenue_summary
            .total_expected_marketable_harvest_kg,
        yield_kg_m2_cultivated: simulation_summary
            .harvest_and_revenue_summary
            .total_harvest_kg_m2_cultivated,
        climate_time_within_target_fraction: simulation_summary
            .seasonal_climate_suitability
            .fraction_within_target_f_within,
        net_fresh_water_intake_m3: w.net_fresh_water_intake_w_fresh_m3,
        water_saved_by_recirculation_and_condensation_m3: water_saved_m3,
        coincident_peak_electrical_demand_kw: energy_calculation
            .peak_demand_analysis
            .coincident_peak_demand_kw,
        average_daily_electrical_use_kwh_day: energy_calculation
            .seasonal_energy_summary
            .average_daily_electrical_kwh_day,
        construction_capex_total: construction_capex,
        selected_solar_package_capex: solar_capex,
        combined_capex_construction_plus_solar: combined_capex,
        cost_of_running_opex_per_cycle: opex,
        break_even_farmgate_price_per_kg: tariff_and_running_cost
            .unit_economics_and_margin_summary
            .break_even_farmgate_price_per_kg,
        user_market_price_per_kg: input.market_price_per_kg,
        expected_gross_revenue_per_cycle: gross_revenue,
        expected_net_operating_income_per_cycle: net_income,
        simple_payback_cycles_construction_only: payback_construction,
        simple_payback_cycles_with_solar: payback_with_solar,
    };

    Ok(AssemblePlanningReportOutput {
        preset_id,
        currency_code,
        executive_feasibility_highlights,
        simulation_summary,
        energy_calculation,
        tariff_and_running_cost,
        solar_package_recommendation,
        construction_boq_estimate,
        provenance_and_scope_statement: vec![
            "The current summary service uses fixed demonstration harvest, climate and water values. This report does not establish that a completed coupled simulation produced them.".to_string(),
            "All area-normalized metrics explicitly distinguish total greenhouse floor area (1,036.8 m2) from cultivated crop area (943.488 m2, r_cul = 0.91).".to_string(),
            "Revenue sums monthly demonstration harvest multiplied by constant monthly estimated prices, unless a user supplies a flat-price override. Operating income subtracts listed OPEX. Payback divides the stated capital cost by positive operating income; solar-specific changes to OPEX are not modelled here.".to_string(),
            "Root-zone EC/pH nutrient chemistry and site-specific structural engineering require separate specialized models/stamps.".to_string(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_planning_report_end_to_end_consistency() {
        let out = assemble_planning_report_service(AssemblePlanningReportInput {
            market_price_per_kg: Some(130.0),
            ..Default::default()
        })
            .expect("assemble_planning_report_service should succeed");

        let h = &out.executive_feasibility_highlights;
        assert_eq!(out.currency_code, "KES");
        assert!((h.expected_marketable_harvest_kg - 47_085.05).abs() < 1e-9);
        assert!(
            (h.combined_capex_construction_plus_solar
                - (h.construction_capex_total + h.selected_solar_package_capex))
                .abs()
                < 1e-9
        );
        let expected_rev = 47_085.05 * 130.0;
        assert!((h.expected_gross_revenue_per_cycle.unwrap() - expected_rev).abs() < 1e-6);
        assert!(h.simple_payback_cycles_construction_only.unwrap() > 0.0);
    }
    #[test]
    fn test_monthly_pricing_report_consistency() {
        let out = assemble_planning_report_service(
            AssemblePlanningReportInput::default(),
        ).expect("monthly report should succeed");
        let h = &out.executive_feasibility_highlights;
        let harvest = &out.simulation_summary.harvest_and_revenue_summary;
        let economics = &out.tariff_and_running_cost.unit_economics_and_margin_summary;
        let monthly_total: f64 = harvest.monthly_harvest_calendar.iter()
            .map(|month| month.expected_monthly_revenue.unwrap())
            .sum();
        assert!((monthly_total - 3_352_708.25).abs() < 1e-6);
        for revenue in [h.expected_gross_revenue_per_cycle,
            harvest.expected_gross_revenue_total, economics.expected_gross_revenue]
        {
            assert!((revenue.unwrap() - monthly_total).abs() < 1e-6);
        }
        let expected_net = monthly_total - h.cost_of_running_opex_per_cycle;
        assert!((h.expected_net_operating_income_per_cycle.unwrap() - expected_net).abs() < 1e-6);
        assert!((economics.expected_net_operating_income_ebitda.unwrap() - expected_net).abs() < 1e-6);
        assert_eq!(h.user_market_price_per_kg, None);
        if expected_net <= 0.0 {
            assert_eq!(h.simple_payback_cycles_construction_only, None);
            assert_eq!(h.simple_payback_cycles_with_solar, None);
        }
    }

}