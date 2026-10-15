use serde::{Deserialize, Serialize};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";
const FLOOR_AREA_M2: f64 = 1_036.8;
const CULTIVATED_AREA_M2: f64 = 943.488;
const EXPECTED_MARKETABLE_HARVEST_KG: f64 = 47_085.05;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceEnergyInput {
    pub preset_id: Option<String>,
    /// Supported tariff profiles: "kplc_kenya_ci1_tou" (default for Witeithie pitching) or "edf_france_pro_tou"
    pub tariff_id: Option<String>,
    /// Optional user-supplied market price per kg of tomato so the response also shows Net Operating Margin (Revenue - Cost of Running).
    pub market_price_per_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceEnergyOutput {
    pub preset_id: String,
    pub tariff_id: String,
    pub currency_code: String,
    pub tariff_effective_date: String,
    pub electricity_bill_breakdown: ElectricityBillBreakdown,
    pub cost_of_running_opex_breakdown: CostOfRunningOpexBreakdown,
    pub unit_economics_and_margin_summary: UnitEconomicsAndMarginSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricityBillBreakdown {
    pub daytime_energy_kwh: f64,
    pub nighttime_energy_kwh: f64,
    pub daytime_rate_per_kwh: f64,
    pub nighttime_offpeak_rate_per_kwh: f64,
    pub c_energy: f64,
    pub coincident_peak_demand_kw: f64,
    pub billing_months: f64,
    pub demand_rate_per_kw_month: f64,
    pub c_demand: f64,
    pub fixed_charge_per_month: f64,
    pub c_fixed: f64,
    pub adjustment_and_levy_fraction: f64,
    pub c_applicable_adjustments: f64,
    pub c_electricity_total: f64,
    pub effective_blended_cost_per_kwh: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostOfRunningOpexBreakdown {
    // 1. Physics-driven utility running costs (directly from simulation models)
    pub c_electricity_utilities: f64,
    pub c_heating_fuel_thermal: f64,
    pub c_fresh_water_supply: f64,
    pub c_co2_enrichment_supply: f64,
    pub subtotal_physics_driven_utility_opex: f64,
    // 2. Agronomic & operational running costs (schedule/assumed inputs)
    pub c_seeds_substrate_fertilizer_ipm: f64,
    pub c_operating_and_harvest_labour: f64,
    pub c_routine_maintenance_and_consumables: f64,
    pub subtotal_agronomic_and_labour_opex: f64,
    // Total Cost of Running (OPEX)
    pub c_running_total_opex: f64,
    pub c_running_per_month_average: f64,
    pub c_running_per_m2_floor: f64,
    pub c_running_per_m2_cultivated: f64,
    pub c_running_per_kg_harvested: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitEconomicsAndMarginSummary {
    pub expected_marketable_harvest_kg: f64,
    pub c_running_total_opex: f64,
    pub break_even_farmgate_price_per_kg: f64,
    pub user_market_price_per_kg: Option<f64>,
    pub expected_gross_revenue: Option<f64>,
    pub expected_net_operating_income_ebitda: Option<f64>,
    pub revenue_and_margin_formula: String,
}

struct TariffSpec {
    tariff_id: &'static str,
    currency_code: &'static str,
    effective_date: &'static str,
    day_rate_kwh: f64,
    night_rate_kwh: f64,
    demand_rate_kw_month: f64,
    fixed_month: f64,
    adjustment_fraction: f64,
    heating_fuel_cost: f64,
    water_rate_per_m3: f64,
    co2_rate_per_kg: f64,
    crop_inputs_cost: f64,
    labour_cost: f64,
    maintenance_cost: f64,
}

const KPLC_KENYA_SPEC: TariffSpec = TariffSpec {
    tariff_id: "kplc_kenya_ci1_tou",
    currency_code: "KES",
    effective_date: "2026-01-01",
    day_rate_kwh: 18.50,
    night_rate_kwh: 9.25, // 50% TOU off-peak discount
    demand_rate_kw_month: 1_100.0,
    fixed_month: 5_500.0,
    adjustment_fraction: 0.24, // fuel energy cost, FX, EPRA, WARMA & VAT adjustments
    heating_fuel_cost: 420_000.0, // supplemental biomass/LPG night heating equivalent
    water_rate_per_m3: 135.0,
    co2_rate_per_kg: 38.0,
    crop_inputs_cost: 685_000.0, // seedlings, cocopeat slabs, hydroponic A+B salts, IPM
    labour_cost: 792_000.0,      // 6 greenhouse workers across 11 months
    maintenance_cost: 165_000.0,
};

const EDF_FRANCE_SPEC: TariffSpec = TariffSpec {
    tariff_id: "edf_france_pro_tou",
    currency_code: "EUR",
    effective_date: "2015-01-01",
    day_rate_kwh: 0.145,
    night_rate_kwh: 0.092,
    demand_rate_kw_month: 9.80,
    fixed_month: 42.0,
    adjustment_fraction: 0.18,
    heating_fuel_cost: 14_930.0,
    water_rate_per_m3: 2.10,
    co2_rate_per_kg: 0.28,
    crop_inputs_cost: 9_400.0,
    labour_cost: 28_500.0,
    maintenance_cost: 3_200.0,
};

pub fn price_energy_service(input: PriceEnergyInput) -> Result<PriceEnergyOutput, String> {
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

    let tariff_id = input
        .tariff_id
        .unwrap_or_else(|| "kplc_kenya_ci1_tou".to_string());
    let spec = match tariff_id.as_str() {
        "kplc_kenya_ci1_tou" => &KPLC_KENYA_SPEC,
        "edf_france_pro_tou" => &EDF_FRANCE_SPEC,
        other => {
            return Err(format!(
                "Unsupported tariff_id '{}'. Valid values: kplc_kenya_ci1_tou, edf_france_pro_tou.",
                other
            ))
        }
    };

    // Simulated quantities from calculate_energy, summarize_simulation, and prepare_co2_exchange
    let daytime_kwh = 19_647.0;
    let nighttime_kwh = 14_319.0;
    let total_kwh = daytime_kwh + nighttime_kwh;
    let coincident_peak_kw = 7.7;
    let billing_months = 11.0;
    let net_fresh_water_m3 = 593.2;
    let injected_co2_kg = 11_420.0;

    // C_electricity = C_energy + C_demand + C_fixed + C_applicable_adjustments
    let c_energy =
        daytime_kwh * spec.day_rate_kwh + nighttime_kwh * spec.night_rate_kwh;
    let c_demand = coincident_peak_kw * spec.demand_rate_kw_month * billing_months;
    let c_fixed = spec.fixed_month * billing_months;
    let base_bill = c_energy + c_demand + c_fixed;
    let c_adjustments = base_bill * spec.adjustment_fraction;
    let c_electricity = base_bill + c_adjustments;

    let electricity_bill_breakdown = ElectricityBillBreakdown {
        daytime_energy_kwh: daytime_kwh,
        nighttime_energy_kwh: nighttime_kwh,
        daytime_rate_per_kwh: spec.day_rate_kwh,
        nighttime_offpeak_rate_per_kwh: spec.night_rate_kwh,
        c_energy,
        coincident_peak_demand_kw: coincident_peak_kw,
        billing_months,
        demand_rate_per_kw_month: spec.demand_rate_kw_month,
        c_demand,
        fixed_charge_per_month: spec.fixed_month,
        c_fixed,
        adjustment_and_levy_fraction: spec.adjustment_fraction,
        c_applicable_adjustments: c_adjustments,
        c_electricity_total: c_electricity,
        effective_blended_cost_per_kwh: c_electricity / total_kwh,
    };

    // Full Cost of Running (OPEX)
    let c_water = net_fresh_water_m3 * spec.water_rate_per_m3;
    let c_co2 = injected_co2_kg * spec.co2_rate_per_kg;
    let subtotal_utilities =
        c_electricity + spec.heating_fuel_cost + c_water + c_co2;
    let subtotal_agronomic_labour =
        spec.crop_inputs_cost + spec.labour_cost + spec.maintenance_cost;
    let c_running_total = subtotal_utilities + subtotal_agronomic_labour;

    let cost_of_running_opex_breakdown = CostOfRunningOpexBreakdown {
        c_electricity_utilities: c_electricity,
        c_heating_fuel_thermal: spec.heating_fuel_cost,
        c_fresh_water_supply: c_water,
        c_co2_enrichment_supply: c_co2,
        subtotal_physics_driven_utility_opex: subtotal_utilities,
        c_seeds_substrate_fertilizer_ipm: spec.crop_inputs_cost,
        c_operating_and_harvest_labour: spec.labour_cost,
        c_routine_maintenance_and_consumables: spec.maintenance_cost,
        subtotal_agronomic_and_labour_opex: subtotal_agronomic_labour,
        c_running_total_opex: c_running_total,
        c_running_per_month_average: c_running_total / billing_months,
        c_running_per_m2_floor: c_running_total / FLOOR_AREA_M2,
        c_running_per_m2_cultivated: c_running_total / CULTIVATED_AREA_M2,
        c_running_per_kg_harvested: c_running_total / EXPECTED_MARKETABLE_HARVEST_KG,
    };

    let expected_gross_revenue = input
        .market_price_per_kg
        .map(|p| EXPECTED_MARKETABLE_HARVEST_KG * p);
    let expected_net_operating_income =
        expected_gross_revenue.map(|rev| rev - c_running_total);

    let unit_economics_and_margin_summary = UnitEconomicsAndMarginSummary {
        expected_marketable_harvest_kg: EXPECTED_MARKETABLE_HARVEST_KG,
        c_running_total_opex: c_running_total,
        break_even_farmgate_price_per_kg: c_running_total / EXPECTED_MARKETABLE_HARVEST_KG,
        user_market_price_per_kg: input.market_price_per_kg,
        expected_gross_revenue,
        expected_net_operating_income_ebitda: expected_net_operating_income,
        revenue_and_margin_formula:
        "net_operating_income = (expected_marketable_harvest_kg * market_price_per_kg) - c_running_total_opex"
            .to_string(),
    };

    Ok(PriceEnergyOutput {
        preset_id,
        tariff_id: spec.tariff_id.to_string(),
        currency_code: spec.currency_code.to_string(),
        tariff_effective_date: spec.effective_date.to_string(),
        electricity_bill_breakdown,
        cost_of_running_opex_breakdown,
        unit_economics_and_margin_summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tariff_and_running_cost_conservation() {
        let out = price_energy_service(PriceEnergyInput {
            market_price_per_kg: Some(130.0),
            ..Default::default()
        })
            .unwrap();

        let b = &out.electricity_bill_breakdown;
        let expected_elec =
            b.c_energy + b.c_demand + b.c_fixed + b.c_applicable_adjustments;
        assert!((b.c_electricity_total - expected_elec).abs() < 1e-9);

        let opex = &out.cost_of_running_opex_breakdown;
        let expected_u = opex.c_electricity_utilities
            + opex.c_heating_fuel_thermal
            + opex.c_fresh_water_supply
            + opex.c_co2_enrichment_supply;
        assert!((opex.subtotal_physics_driven_utility_opex - expected_u).abs() < 1e-9);

        let expected_total_opex =
            opex.subtotal_physics_driven_utility_opex + opex.subtotal_agronomic_and_labour_opex;
        assert!((opex.c_running_total_opex - expected_total_opex).abs() < 1e-9);

        let m = &out.unit_economics_and_margin_summary;
        let rev = m.expected_gross_revenue.unwrap();
        let net = m.expected_net_operating_income_ebitda.unwrap();
        assert!((net - (rev - opex.c_running_total_opex)).abs() < 1e-9);
    }
}