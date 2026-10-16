import { calculateEnergy } from './energy.js';
import { priceEnergy } from './tariffs.js';
import { lookupSolarPackage } from './solarPackage.js';
import { estimateConstruction } from './construction.js';
import { summarizeSimulation } from './simulationSummary.js';

export interface AssemblePlanningReportInput {
  preset_id?: string;
  period_filter?: string;
  tariff_id?: string;
  cost_book_id?: string;
  solar_preference_mode?: string;
  required_backup_hours?: number;
  market_price_per_kg?: number;
  month_filter?: string | string[];
}

export function assemblePlanningReport(input: AssemblePlanningReportInput = {}) {
  const preset_id = input.preset_id || 'ctifl_nantes_2014_2015';
  const tariff_id = input.tariff_id || 'kplc_kenya_ci1_tou';
  const cost_book_id =
    input.cost_book_id || (tariff_id === 'edf_france_pro_tou' ? 'europe_venlo_reference_eur' : 'kenya_witeithie_2026_kes');
  const currency_code = tariff_id === 'edf_france_pro_tou' ? 'EUR' : 'KES';

  const simulation_summary = summarizeSimulation({
    preset_id,
    period_filter: input.period_filter,
    market_price_per_kg: input.market_price_per_kg,
    currency_code,
    month_filter: input.month_filter,
  });

  const energy_calculation = calculateEnergy({
    preset_id,
    period_filter: input.period_filter,
  });

  const tariff_and_running_cost = priceEnergy({
    preset_id,
    tariff_id,
    market_price_per_kg: input.market_price_per_kg,
  });

  const solar_package_recommendation = lookupSolarPackage({
    preset_id,
    preference_mode: input.solar_preference_mode,
    required_backup_hours: input.required_backup_hours,
  });

  const construction_boq_estimate = estimateConstruction({
    preset_id,
    cost_book_id,
  });

  const solar_capex =
    currency_code === 'EUR'
      ? solar_package_recommendation.selected_package.indicative_installed_cost_eur
      : solar_package_recommendation.selected_package.indicative_installed_cost_kes;

  const construction_capex = construction_boq_estimate.c_construction_total;
  const combined_capex = construction_capex + solar_capex;
  const opex = tariff_and_running_cost.cost_of_running_opex_breakdown.c_running_total_opex;
  const gross_revenue = simulation_summary.harvest_and_revenue_summary.expected_gross_revenue_total;
  const net_income = gross_revenue !== null ? gross_revenue - opex : null;

  tariff_and_running_cost.unit_economics_and_margin_summary.expected_gross_revenue = gross_revenue;
  tariff_and_running_cost.unit_economics_and_margin_summary.expected_net_operating_income_ebitda = net_income;

  const payback_construction = net_income !== null && net_income > 0 ? construction_capex / net_income : null;
  const payback_with_solar = net_income !== null && net_income > 0 ? combined_capex / net_income : null;

  const w = simulation_summary.water_planning_summary;
  const water_saved_m3 = w.gross_recirculated_irrigation_m3 - w.net_fresh_water_intake_w_fresh_m3;

  const executive_feasibility_highlights = {
    floor_area_m2: simulation_summary.floor_area_m2,
    cultivated_area_m2: simulation_summary.cultivated_area_m2,
    cycle_duration_days: simulation_summary.crop_cycle_guidance.cycle_duration_days,
    expected_marketable_harvest_kg: simulation_summary.harvest_and_revenue_summary.total_expected_marketable_harvest_kg,
    yield_kg_m2_cultivated: simulation_summary.harvest_and_revenue_summary.total_harvest_kg_m2_cultivated,
    climate_time_within_target_fraction: simulation_summary.seasonal_climate_suitability.fraction_within_target_f_within,
    net_fresh_water_intake_m3: w.net_fresh_water_intake_w_fresh_m3,
    water_saved_by_recirculation_and_condensation_m3: water_saved_m3,
    coincident_peak_electrical_demand_kw: energy_calculation.peak_demand_analysis.coincident_peak_demand_kw,
    average_daily_electrical_use_kwh_day: energy_calculation.seasonal_energy_summary.average_daily_electrical_kwh_day,
    construction_capex_total: construction_capex,
    selected_solar_package_capex: solar_capex,
    combined_capex_construction_plus_solar: combined_capex,
    cost_of_running_opex_per_cycle: opex,
    break_even_farmgate_price_per_kg: tariff_and_running_cost.unit_economics_and_margin_summary.break_even_farmgate_price_per_kg,
    user_market_price_per_kg: input.market_price_per_kg ?? null,
    expected_gross_revenue_per_cycle: gross_revenue,
    expected_net_operating_income_per_cycle: net_income,
    simple_payback_cycles_construction_only: payback_construction,
    simple_payback_cycles_with_solar: payback_with_solar,
  };

  return {
    preset_id,
    currency_code,
    executive_feasibility_highlights,
    simulation_summary,
    energy_calculation,
    tariff_and_running_cost,
    solar_package_recommendation,
    construction_boq_estimate,
    provenance_and_scope_statement: [
      'The current summary service uses fixed demonstration harvest, climate and water values. This report does not establish that a completed coupled simulation produced them.',
      'All area-normalized metrics explicitly distinguish total greenhouse floor area (1,036.8 m2) from cultivated crop area (943.488 m2, r_cul = 0.91).',
      'Revenue sums monthly demonstration harvest multiplied by constant monthly estimated prices, unless a user supplies a flat-price override. Operating income subtracts listed OPEX. Payback divides the stated capital cost by positive operating income; solar-specific changes to OPEX are not modelled here.',
      'Root-zone EC/pH nutrient chemistry and site-specific structural engineering require separate specialized models/stamps.',
    ],
  };
}
