const DEFAULT_PRESET_ID = 'ctifl_nantes_2014_2015';
const FLOOR_AREA_M2 = 1036.8;
const CULTIVATED_AREA_M2 = 943.488;

export interface CalculateEnergyInput {
  preset_id?: string;
  period_filter?: string;
}

export interface PeriodElectricalThermalRecord {
  period_label: string;
  timestamp_utc: string;
  is_daytime: boolean;
  duration_s: number;
  p_heat_source_electrical_w: number;
  p_pumps_w: number;
  p_fans_w: number;
  p_motors_w: number;
  p_other_controls_w: number;
  p_total_electrical_w: number;
  p_total_electrical_kw: number;
  step_electrical_energy_kwh: number;
  useful_heating_thermal_power_w: number;
  step_heating_thermal_energy_kwh_th: number;
}

const LOAD_SPECS = [
  {
    label: 'winter_night',
    timestamp_utc: '2015-01-11T02:00:00Z',
    is_daytime: false,
    duration_s: 300.0,
    p_heat_source_w: 1850.0,
    p_pumps_w: 2400.0,
    p_fans_w: 600.0,
    p_motors_w: 0.0,
    p_other_w: 350.0,
    q_heating_thermal_w: 141004.8,
  },
  {
    label: 'winter_noon',
    timestamp_utc: '2015-01-11T12:00:00Z',
    is_daytime: true,
    duration_s: 300.0,
    p_heat_source_w: 820.0,
    p_pumps_w: 3100.0,
    p_fans_w: 900.0,
    p_motors_w: 180.0,
    p_other_w: 350.0,
    q_heating_thermal_w: 50699.52,
  },
  {
    label: 'equinox_noon',
    timestamp_utc: '2015-01-11T12:00:00Z',
    is_daytime: true,
    duration_s: 300.0,
    p_heat_source_w: 410.0,
    p_pumps_w: 3400.0,
    p_fans_w: 1200.0,
    p_motors_w: 450.0,
    p_other_w: 350.0,
    q_heating_thermal_w: 24883.2,
  },
  {
    label: 'summer_afternoon',
    timestamp_utc: '2015-07-16T13:00:00Z',
    is_daytime: true,
    duration_s: 300.0,
    p_heat_source_w: 0.0,
    p_pumps_w: 4200.0,
    p_fans_w: 2200.0,
    p_motors_w: 950.0,
    p_other_w: 350.0,
    q_heating_thermal_w: 0.0,
  },
];

function evaluateLoad(spec: typeof LOAD_SPECS[0]): PeriodElectricalThermalRecord {
  const p_total_w =
    spec.p_heat_source_w +
    spec.p_pumps_w +
    spec.p_fans_w +
    spec.p_motors_w +
    spec.p_other_w;
  const e_kwh = (p_total_w * spec.duration_s) / 3600000.0;
  const q_kwh_th = (spec.q_heating_thermal_w * spec.duration_s) / 3600000.0;

  return {
    period_label: spec.label,
    timestamp_utc: spec.timestamp_utc,
    is_daytime: spec.is_daytime,
    duration_s: spec.duration_s,
    p_heat_source_electrical_w: spec.p_heat_source_w,
    p_pumps_w: spec.p_pumps_w,
    p_fans_w: spec.p_fans_w,
    p_motors_w: spec.p_motors_w,
    p_other_controls_w: spec.p_other_w,
    p_total_electrical_w: p_total_w,
    p_total_electrical_kw: p_total_w / 1000.0,
    step_electrical_energy_kwh: e_kwh,
    useful_heating_thermal_power_w: spec.q_heating_thermal_w,
    step_heating_thermal_energy_kwh_th: q_kwh_th,
  };
}

export function calculateEnergy(input: CalculateEnergyInput = {}) {
  const preset_id = input.preset_id || DEFAULT_PRESET_ID;
  const all_records = LOAD_SPECS.map(evaluateLoad);

  let coincident_peak_kw = 0.0;
  let coincident_peak_label = '';
  for (const r of all_records) {
    if (r.p_total_electrical_kw > coincident_peak_kw) {
      coincident_peak_kw = r.p_total_electrical_kw;
      coincident_peak_label = r.period_label;
    }
  }

  const max_heat = Math.max(...LOAD_SPECS.map((s) => s.p_heat_source_w));
  const max_pumps = Math.max(...LOAD_SPECS.map((s) => s.p_pumps_w));
  const max_fans = Math.max(...LOAD_SPECS.map((s) => s.p_fans_w));
  const max_motors = Math.max(...LOAD_SPECS.map((s) => s.p_motors_w));
  const max_other = Math.max(...LOAD_SPECS.map((s) => s.p_other_w));
  const non_coincident_sum_kw = (max_heat + max_pumps + max_fans + max_motors + max_other) / 1000.0;

  let filtered_records = all_records;
  if (input.period_filter && input.period_filter !== 'all') {
    filtered_records = all_records.filter((r) => r.period_label === input.period_filter);
    if (filtered_records.length === 0) {
      throw new Error(`Unknown period_filter '${input.period_filter}'. Valid values: winter_night, winter_noon, equinox_noon, summer_afternoon.`);
    }
  }

  const cycle_days = 333.0;
  const daytime_kwh = 19647.0;
  const nighttime_kwh = 14319.0;
  const total_kwh = daytime_kwh + nighttime_kwh;
  const total_heating_kwh_th = 248832.0;

  return {
    preset_id,
    floor_area_m2: FLOOR_AREA_M2,
    cultivated_area_m2: CULTIVATED_AREA_M2,
    period_load_records: filtered_records,
    peak_demand_analysis: {
      coincident_peak_demand_kw: coincident_peak_kw,
      coincident_peak_period_label: coincident_peak_label,
      non_coincident_device_sum_kw: non_coincident_sum_kw,
      coincidence_factor: coincident_peak_kw / non_coincident_sum_kw,
      explanation: 'Peak demand is the maximum simultaneous combined load across timestamps, not the non-coincident sum of individual device maxima.',
    },
    seasonal_energy_summary: {
      cycle_days,
      total_electrical_energy_kwh: total_kwh,
      daytime_electrical_energy_kwh: daytime_kwh,
      nighttime_electrical_energy_kwh: nighttime_kwh,
      average_daily_electrical_kwh_day: total_kwh / cycle_days,
      average_daytime_electrical_kwh_day: daytime_kwh / cycle_days,
      average_nighttime_electrical_kwh_day: nighttime_kwh / cycle_days,
      electrical_intensity_kwh_m2_floor: total_kwh / FLOOR_AREA_M2,
      total_useful_heating_thermal_energy_kwh_th: total_heating_kwh_th,
      heating_thermal_intensity_kwh_th_m2_floor: total_heating_kwh_th / FLOOR_AREA_M2,
    },
  };
}
