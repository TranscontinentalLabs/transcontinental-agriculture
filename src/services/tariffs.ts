const DEFAULT_PRESET_ID = 'ctifl_nantes_2014_2015';
const FLOOR_AREA_M2 = 1036.8;
const CULTIVATED_AREA_M2 = 943.488;
const EXPECTED_MARKETABLE_HARVEST_KG = 47085.05;

export interface PriceEnergyInput {
  preset_id?: string;
  tariff_id?: string;
  market_price_per_kg?: number;
}

interface TariffSpec {
  tariff_id: string;
  currency_code: string;
  effective_date: string;
  day_rate_kwh: number;
  night_rate_kwh: number;
  demand_rate_kw_month: number;
  fixed_month: number;
  adjustment_fraction: number;
  heating_fuel_cost: number;
  water_rate_per_m3: number;
  co2_rate_per_kg: number;
  crop_inputs_cost: number;
  labour_cost: number;
  maintenance_cost: number;
}

const KPLC_KENYA_SPEC: TariffSpec = {
  tariff_id: 'kplc_kenya_ci1_tou',
  currency_code: 'KES',
  effective_date: '2026-01-01',
  day_rate_kwh: 18.5,
  night_rate_kwh: 9.25,
  demand_rate_kw_month: 1100.0,
  fixed_month: 5500.0,
  adjustment_fraction: 0.24,
  heating_fuel_cost: 420000.0,
  water_rate_per_m3: 135.0,
  co2_rate_per_kg: 38.0,
  crop_inputs_cost: 685000.0,
  labour_cost: 792000.0,
  maintenance_cost: 165000.0,
};

const EDF_FRANCE_SPEC: TariffSpec = {
  tariff_id: 'edf_france_pro_tou',
  currency_code: 'EUR',
  effective_date: '2015-01-01',
  day_rate_kwh: 0.145,
  night_rate_kwh: 0.092,
  demand_rate_kw_month: 9.8,
  fixed_month: 42.0,
  adjustment_fraction: 0.18,
  heating_fuel_cost: 14930.0,
  water_rate_per_m3: 2.1,
  co2_rate_per_kg: 0.28,
  crop_inputs_cost: 9400.0,
  labour_cost: 28500.0,
  maintenance_cost: 3200.0,
};

export function priceEnergy(input: PriceEnergyInput = {}) {
  const preset_id = input.preset_id || DEFAULT_PRESET_ID;
  const tariff_id = input.tariff_id || 'kplc_kenya_ci1_tou';
  const spec = tariff_id === 'edf_france_pro_tou' ? EDF_FRANCE_SPEC : KPLC_KENYA_SPEC;

  if (input.market_price_per_kg !== undefined) {
    if (!Number.isFinite(input.market_price_per_kg) || input.market_price_per_kg < 0) {
      throw new Error('market_price_per_kg must be a non-negative finite number.');
    }
  }

  const daytime_kwh = 19647.0;
  const nighttime_kwh = 14319.0;
  const total_kwh = daytime_kwh + nighttime_kwh;
  const coincident_peak_kw = 7.7;
  const billing_months = 11.0;
  const net_fresh_water_m3 = 593.2;
  const injected_co2_kg = 11420.0;

  const c_energy = daytime_kwh * spec.day_rate_kwh + nighttime_kwh * spec.night_rate_kwh;
  const c_demand = coincident_peak_kw * spec.demand_rate_kw_month * billing_months;
  const c_fixed = spec.fixed_month * billing_months;
  const base_bill = c_energy + c_demand + c_fixed;
  const c_adjustments = base_bill * spec.adjustment_fraction;
  const c_electricity = base_bill + c_adjustments;

  const electricity_bill_breakdown = {
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

  const c_water = net_fresh_water_m3 * spec.water_rate_per_m3;
  const c_co2 = injected_co2_kg * spec.co2_rate_per_kg;
  const subtotal_utilities = c_electricity + spec.heating_fuel_cost + c_water + c_co2;
  const subtotal_agronomic_labour = spec.crop_inputs_cost + spec.labour_cost + spec.maintenance_cost;
  const c_running_total = subtotal_utilities + subtotal_agronomic_labour;

  const cost_of_running_opex_breakdown = {
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

  const expected_gross_revenue = input.market_price_per_kg !== undefined
    ? EXPECTED_MARKETABLE_HARVEST_KG * input.market_price_per_kg
    : null;
  const expected_net_operating_income = expected_gross_revenue !== null
    ? expected_gross_revenue - c_running_total
    : null;

  const unit_economics_and_margin_summary = {
    expected_marketable_harvest_kg: EXPECTED_MARKETABLE_HARVEST_KG,
    c_running_total_opex: c_running_total,
    break_even_farmgate_price_per_kg: c_running_total / EXPECTED_MARKETABLE_HARVEST_KG,
    user_market_price_per_kg: input.market_price_per_kg ?? null,
    expected_gross_revenue,
    expected_net_operating_income_ebitda: expected_net_operating_income,
    revenue_and_margin_formula:
      'net_operating_income = (expected_marketable_harvest_kg * market_price_per_kg) - c_running_total_opex',
  };

  return {
    preset_id,
    tariff_id: spec.tariff_id,
    currency_code: spec.currency_code,
    tariff_effective_date: spec.effective_date,
    electricity_bill_breakdown,
    cost_of_running_opex_breakdown,
    unit_economics_and_margin_summary,
  };
}
