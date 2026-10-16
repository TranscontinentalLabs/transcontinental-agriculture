const DEFAULT_PRESET_ID = 'ctifl_nantes_2014_2015';

export interface LookupSolarPackageInput {
  preset_id?: string;
  preference_mode?: string;
  required_backup_hours?: number;
}

export interface ReviewedSolarPackage {
  package_id: string;
  preference_mode: string;
  pv_array_capacity_kwp: number;
  hybrid_inverter_rating_kw: number;
  battery_nominal_capacity_kwh: number;
  battery_usable_capacity_kwh_at_85pct_dod: number;
  estimated_autonomous_night_backup_hours: number;
  indicative_installed_cost_kes: number;
  indicative_installed_cost_eur: number;
  sizing_assumptions: string;
}

const PACKAGE_SPECS = [
  {
    package_id: 'venlo_pv25_inv12_bess0_grid_tied',
    mode: 'grid_tied_daytime_offset',
    pv_kwp: 25.0,
    inverter_kw: 12.0,
    bess_nominal_kwh: 0.0,
    cost_kes: 2850000.0,
    cost_eur: 19800.0,
    assumptions: 'Offsets daytime fertigation pump and fan loads under 5.2 peak sun hours/day; relies on grid for all nighttime loads and cloud transients.',
  },
  {
    package_id: 'venlo_pv35_inv15_bess60_hybrid',
    mode: 'hybrid_critical_load_backup',
    pv_kwp: 35.0,
    inverter_kw: 15.0,
    bess_nominal_kwh: 60.0,
    cost_kes: 6450000.0,
    cost_eur: 44900.0,
    assumptions: 'Covers 100% of daytime load plus full 43.0 kWh average nighttime load at 85% LiFePO4 DoD (51.0 kWh usable), with grid/generator fallback for multi-day overcast weather.',
  },
  {
    package_id: 'venlo_pv55_inv20_bess120_offgrid',
    mode: 'off_grid_full_autonomy',
    pv_kwp: 55.0,
    inverter_kw: 20.0,
    bess_nominal_kwh: 120.0,
    cost_kes: 11900000.0,
    cost_eur: 82800.0,
    assumptions: 'Provides 102.0 kWh usable storage (~24 hours of full load autonomy) and 2.5x surge margin over 7.7 kW coincident peak.',
  },
];

function evaluatePackage(spec: typeof PACKAGE_SPECS[0], avg_night_kw: number): ReviewedSolarPackage {
  const usable_kwh = spec.bess_nominal_kwh * 0.85;
  const backup_hours = avg_night_kw > 0 ? usable_kwh / avg_night_kw : 0.0;
  return {
    package_id: spec.package_id,
    preference_mode: spec.mode,
    pv_array_capacity_kwp: spec.pv_kwp,
    hybrid_inverter_rating_kw: spec.inverter_kw,
    battery_nominal_capacity_kwh: spec.bess_nominal_kwh,
    battery_usable_capacity_kwh_at_85pct_dod: usable_kwh,
    estimated_autonomous_night_backup_hours: backup_hours,
    indicative_installed_cost_kes: spec.cost_kes,
    indicative_installed_cost_eur: spec.cost_eur,
    sizing_assumptions: spec.assumptions,
  };
}

export function lookupSolarPackage(input: LookupSolarPackageInput = {}) {
  const preset_id = input.preset_id || DEFAULT_PRESET_ID;
  const mode = input.preference_mode || 'hybrid_critical_load_backup';
  const backup_hours = input.required_backup_hours ?? 12.0;

  const daytime_kwh_day = 19647.0 / 333.0; // 59.0 kWh/day
  const nighttime_kwh_day = 14319.0 / 333.0; // 43.0 kWh/day
  const daily_kwh_day = daytime_kwh_day + nighttime_kwh_day;
  const coincident_peak_kw = 7.7;
  const avg_night_kw = nighttime_kwh_day / 12.0;

  const all_packages = PACKAGE_SPECS.map((s) => evaluatePackage(s, avg_night_kw));
  const selected = all_packages.find((p) => p.preference_mode === mode);
  if (!selected) {
    throw new Error(`Unknown preference_mode '${mode}'. Valid values: grid_tied_daytime_offset, hybrid_critical_load_backup, off_grid_full_autonomy.`);
  }

  const alternatives = all_packages.filter((p) => p.preference_mode !== mode);

  return {
    preset_id,
    load_sizing_inputs: {
      daily_electricity_use_kwh_day: daily_kwh_day,
      daytime_consumption_kwh_day: daytime_kwh_day,
      nighttime_consumption_kwh_day: nighttime_kwh_day,
      coincident_peak_demand_kw: coincident_peak_kw,
      required_backup_period_hours: backup_hours,
      preference_mode: mode,
    },
    selected_package: selected,
    alternative_packages: alternatives,
    engineering_caveat: 'Matching average daily kWh does not guarantee uninterrupted supply: 42.2% of greenhouse electrical load occurs at night (circulation pumps and boiler/climate fans), requiring battery storage or grid/generator support.',
  };
}
