const DEFAULT_PRESET_ID = 'ctifl_nantes_2014_2015';
const FLOOR_AREA_M2 = 1036.8;
const CULTIVATED_AREA_M2 = 943.488;

export const MONTHLY_TOMATO_PRICES_KES_PER_KG = [
  75.0, 85.0, 95.0, 103.0, 108.0, 65.0,
  45.0, 40.0, 55.0, 65.0, 73.0, 83.0,
];

export function estimatedMonthlyPriceKes(monthLabel: string): number {
  const parts = monthLabel.split('-');
  if (parts.length !== 2) throw new Error(`Invalid month label '${monthLabel}': expected YYYY-MM.`);
  const month = parseInt(parts[1], 10);
  if (isNaN(month) || month < 1 || month > 12) {
    throw new Error('Month number must be between 01 and 12.');
  }
  return MONTHLY_TOMATO_PRICES_KES_PER_KG[month - 1];
}

const PERIOD_CLIMATE_SPECS = [
  { label: 'winter_night', timestamp_utc: '2015-01-11T02:00:00Z', duration_h: 1.0, t_air_c: 17.2, t_min_c: 16.5, t_max_c: 20.0 },
  { label: 'winter_noon', timestamp_utc: '2015-01-11T12:00:00Z', duration_h: 1.0, t_air_c: 20.4, t_min_c: 18.5, t_max_c: 24.0 },
  { label: 'equinox_noon', timestamp_utc: '2015-03-20T12:00:00Z', duration_h: 1.0, t_air_c: 23.6, t_min_c: 19.0, t_max_c: 26.0 },
  { label: 'summer_afternoon', timestamp_utc: '2015-07-16T13:00:00Z', duration_h: 1.0, t_air_c: 31.2, t_min_c: 19.0, t_max_c: 28.0 },
];

const MONTHLY_CUMULATIVE_HARVEST_KG: [string, number][] = [
  ['2014-12', 0.0],
  ['2015-01', 0.0],
  ['2015-02', 1420.0],
  ['2015-03', 5180.0],
  ['2015-04', 10640.0],
  ['2015-05', 17490.0],
  ['2015-06', 24980.0],
  ['2015-07', 32150.0],
  ['2015-08', 38420.0],
  ['2015-09', 43510.0],
  ['2015-10', 47085.05],
];

export interface SummarizeSimulationInput {
  preset_id?: string;
  period_filter?: string;
  market_price_per_kg?: number;
  currency_code?: string;
  month_filter?: string | string[];
}

export function summarizeSimulation(input: SummarizeSimulationInput = {}) {
  const preset_id = input.preset_id || DEFAULT_PRESET_ID;
  const currency_code = input.currency_code || 'KES';

  if (input.market_price_per_kg !== undefined) {
    if (!Number.isFinite(input.market_price_per_kg) || input.market_price_per_kg < 0) {
      throw new Error('market_price_per_kg must be a non-negative finite number.');
    }
  }

  if (input.market_price_per_kg === undefined && currency_code !== 'KES') {
    throw new Error('Constant monthly prices are in KES. Use KES or supply a flat price in the requested currency.');
  }

  let climate_records = PERIOD_CLIMATE_SPECS.map((spec) => {
    const within = spec.t_air_c >= spec.t_min_c && spec.t_air_c <= spec.t_max_c ? 1.0 : 0.0;
    const heat_ex_c = Math.max(0.0, spec.t_air_c - spec.t_max_c);
    const cold_def_c = Math.max(0.0, spec.t_min_c - spec.t_air_c);
    return {
      period_label: spec.label,
      timestamp_utc: spec.timestamp_utc,
      duration_hours: spec.duration_h,
      air_temperature_c: spec.t_air_c,
      target_min_temperature_c: spec.t_min_c,
      target_max_temperature_c: spec.t_max_c,
      within_target_indicator: within,
      heat_exceedance_c: heat_ex_c,
      cold_deficit_c: cold_def_c,
      heat_exposure_degree_hours_degc_h: heat_ex_c * spec.duration_h,
      cold_exposure_degree_hours_degc_h: cold_def_c * spec.duration_h,
    };
  });

  if (input.period_filter && input.period_filter !== 'all') {
    climate_records = climate_records.filter((r) => r.period_label === input.period_filter);
  }

  const total_evaluated_hours = 7992.0;
  const hours_within_target_range = 7144.848;

  // Water planning
  const w_transpiration_m3 = 712.4;
  const w_biomass_m3 = 44.7;
  const w_discarded_drainage_m3 = 38.2;
  const w_other_losses_m3 = 14.5;
  const delta_w_storage_m3 = 2.0;
  const w_usable_recovery_m3 = 218.6;
  const gross_recirculated_irrigation_m3 = 985.7;
  const w_fresh_m3 =
    w_transpiration_m3 +
    w_biomass_m3 +
    w_discarded_drainage_m3 +
    w_other_losses_m3 +
    delta_w_storage_m3 -
    w_usable_recovery_m3;

  let prev_cum_kg = 0.0;
  let total_revenue = 0.0;
  const monthly_harvest_calendar = [];

  for (const [month_label, cum_kg] of MONTHLY_CUMULATIVE_HARVEST_KG) {
    const delta_kg = cum_kg - prev_cum_kg;
    prev_cum_kg = cum_kg;
    const price = input.market_price_per_kg !== undefined
      ? input.market_price_per_kg
      : estimatedMonthlyPriceKes(month_label);
    const revenue = delta_kg * price;
    total_revenue += revenue;

    monthly_harvest_calendar.push({
      applied_price_per_kg: price,
      price_source: input.market_price_per_kg !== undefined
        ? 'user_supplied_flat_override'
        : 'user_supplied_constant_seasonal_estimate',
      month_label,
      cumulative_harvest_end_kg: cum_kg,
      monthly_marketable_harvest_kg: delta_kg,
      monthly_harvest_kg_m2_floor: delta_kg / FLOOR_AREA_M2,
      monthly_harvest_kg_m2_cultivated: delta_kg / CULTIVATED_AREA_M2,
      expected_monthly_revenue: revenue,
    });
  }

  let final_calendar = monthly_harvest_calendar;
  if (input.month_filter) {
    const filters = Array.isArray(input.month_filter)
      ? input.month_filter.map((s) => s.toLowerCase().trim())
      : [input.month_filter.toLowerCase().trim()];

    final_calendar = monthly_harvest_calendar.filter((item) => {
      const label = item.month_label.toLowerCase(); // '2014-12'
      return filters.some((f) => {
        if (label === f || label.endsWith(f) || f.indexOf(label) !== -1) return true;
        if (f.includes('dec') && label.endsWith('-12')) return true;
        if (f.includes('jan') && label.endsWith('-01')) return true;
        if (f.includes('feb') && label.endsWith('-02')) return true;
        if (f.includes('mar') && label.endsWith('-03')) return true;
        if (f.includes('apr') && label.endsWith('-04')) return true;
        if (f.includes('may') && label.endsWith('-05')) return true;
        if (f.includes('jun') && label.endsWith('-06')) return true;
        if (f.includes('jul') && label.endsWith('-07')) return true;
        if (f.includes('aug') && label.endsWith('-08')) return true;
        if (f.includes('sep') && label.endsWith('-09')) return true;
        if (f.includes('oct') && label.endsWith('-10')) return true;
        return false;
      });
    });
  }

  const filtered_harvest_kg = final_calendar.reduce((acc, row) => acc + row.monthly_marketable_harvest_kg, 0.0);
  const filtered_revenue = final_calendar.reduce((acc, row) => acc + row.expected_monthly_revenue, 0.0);
  const total_harvest_kg = input.month_filter ? filtered_harvest_kg : prev_cum_kg;
  const final_total_revenue = input.month_filter ? filtered_revenue : total_revenue;

  return {
    preset_id,
    floor_area_m2: FLOOR_AREA_M2,
    cultivated_area_m2: CULTIVATED_AREA_M2,
    climate_suitability_records: climate_records,
    seasonal_climate_suitability: {
      total_evaluated_hours,
      hours_within_target_range,
      fraction_within_target_f_within: hours_within_target_range / total_evaluated_hours,
      heat_exposure_degree_hours_d_hot_degc_h: 1184.6,
      cold_exposure_degree_hours_d_cold_degc_h: 92.4,
      provenance_note: 'Fixed demonstration seasonal indicators; not calculated from the four sample records or a live simulation run.',
    },
    water_planning_summary: {
      gross_recirculated_irrigation_m3,
      crop_transpiration_w_transpiration_m3: w_transpiration_m3,
      crop_biomass_water_retention_w_biomass_m3: w_biomass_m3,
      discarded_drainage_w_discarded_drainage_m3: w_discarded_drainage_m3,
      other_system_losses_w_other_losses_m3: w_other_losses_m3,
      storage_change_delta_w_storage_m3: delta_w_storage_m3,
      usable_condensation_and_drain_recovery_w_usable_recovery_m3: w_usable_recovery_m3,
      net_fresh_water_intake_w_fresh_m3: w_fresh_m3,
      net_fresh_water_per_m2_floor_l_m2: (w_fresh_m3 * 1000.0) / FLOOR_AREA_M2,
      net_fresh_water_per_m2_cultivated_l_m2: (w_fresh_m3 * 1000.0) / CULTIVATED_AREA_M2,
      recirculation_savings_fraction: (gross_recirculated_irrigation_m3 - w_fresh_m3) / gross_recirculated_irrigation_m3,
    },
    harvest_and_revenue_summary: {
      pricing_mode: input.market_price_per_kg !== undefined ? 'flat_override' : 'constant_monthly_estimates',
      harvest_weighted_price_per_kg: total_harvest_kg > 0 ? total_revenue / total_harvest_kg : null,
      price_assumptions_note:
        'Monthly KES prices are user-supplied seasonal assumptions repeated each year; market, grade and farmgate/wholesale basis have not been verified. Physical harvest values are fixed demonstration data.',
      total_expected_marketable_harvest_kg: total_harvest_kg,
      total_harvest_kg_m2_floor: total_harvest_kg / FLOOR_AREA_M2,
      total_harvest_kg_m2_cultivated: total_harvest_kg / CULTIVATED_AREA_M2,
      currency_code,
      user_market_price_per_kg: input.market_price_per_kg ?? null,
      expected_gross_revenue_total: final_total_revenue,
      revenue_calculation_formula: 'expected_gross_revenue = sum(monthly_marketable_harvest_kg * applied_price_per_kg)',
      monthly_harvest_calendar: final_calendar,
      is_filtered: Boolean(input.month_filter),
      filtered_month_count: final_calendar.length,
    },
    crop_cycle_guidance: {
      crop_species_and_cultivar_type: 'Indeterminate high-wire truss tomato (Solanum lycopersicum)',
      cycle_duration_days: 333,
      vegetative_lead_time_to_first_harvest_days: 68,
      active_harvest_window_days: 265,
      initial_stem_density_stems_m2: 2.5,
      peak_stem_density_stems_m2: 3.75,
      planning_horizon_recommendation:
        'Demonstration assumption: 68 days before first harvest. Sales start with harvest; positive cash flow requires a separate monthly expense calculation.',
    },
  };
}
