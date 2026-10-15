use serde::{Deserialize, Serialize};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";
const FLOOR_AREA_M2: f64 = 1_036.8;
const CULTIVATED_AREA_M2: f64 = 943.488;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalculateEnergyInput {
    pub preset_id: Option<String>,
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateEnergyOutput {
    pub preset_id: String,
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub period_load_records: Vec<PeriodElectricalThermalRecord>,
    pub peak_demand_analysis: PeakDemandAnalysis,
    pub seasonal_energy_summary: SeasonalEnergySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodElectricalThermalRecord {
    pub period_label: String,
    pub timestamp_utc: String,
    pub is_daytime: bool,
    pub duration_s: f64,
    pub p_heat_source_electrical_w: f64,
    pub p_pumps_w: f64,
    pub p_fans_w: f64,
    pub p_motors_w: f64,
    pub p_other_controls_w: f64,
    pub p_total_electrical_w: f64,
    pub p_total_electrical_kw: f64,
    pub step_electrical_energy_kwh: f64,
    pub useful_heating_thermal_power_w: f64,
    pub step_heating_thermal_energy_kwh_th: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakDemandAnalysis {
    pub coincident_peak_demand_kw: f64,
    pub coincident_peak_period_label: String,
    pub non_coincident_device_sum_kw: f64,
    pub coincidence_factor: f64,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalEnergySummary {
    pub cycle_days: f64,
    pub total_electrical_energy_kwh: f64,
    pub daytime_electrical_energy_kwh: f64,
    pub nighttime_electrical_energy_kwh: f64,
    pub average_daily_electrical_kwh_day: f64,
    pub average_daytime_electrical_kwh_day: f64,
    pub average_nighttime_electrical_kwh_day: f64,
    pub electrical_intensity_kwh_m2_floor: f64,
    pub total_useful_heating_thermal_energy_kwh_th: f64,
    pub heating_thermal_intensity_kwh_th_m2_floor: f64,
}

struct LoadSpec {
    label: &'static str,
    timestamp_utc: &'static str,
    is_daytime: bool,
    duration_s: f64,
    p_heat_source_w: f64,
    p_pumps_w: f64,
    p_fans_w: f64,
    p_motors_w: f64,
    p_other_w: f64,
    q_heating_thermal_w: f64,
}

const LOAD_SPECS: [LoadSpec; 4] = [
    LoadSpec {
        label: "winter_night",
        timestamp_utc: "2015-01-11T02:00:00Z",
        is_daytime: false,
        duration_s: 300.0,
        p_heat_source_w: 1_850.0, // boiler draft fan + burner controls during peak night heating
        p_pumps_w: 2_400.0,       // hot-water pipe network circulation pumps
        p_fans_w: 600.0,          // horizontal air circulation fans
        p_motors_w: 0.0,          // thermal screen already closed
        p_other_w: 350.0,         // climate computer + sensors
        q_heating_thermal_w: 141_004.8,
    },
    LoadSpec {
        label: "winter_noon",
        timestamp_utc: "2015-01-11T12:00:00Z",
        is_daytime: true,
        duration_s: 300.0,
        p_heat_source_w: 820.0,
        p_pumps_w: 3_100.0, // heating circulation + drip fertigation pump
        p_fans_w: 900.0,
        p_motors_w: 180.0, // minor vent adjustment
        p_other_w: 350.0,
        q_heating_thermal_w: 50_699.52,
    },
    LoadSpec {
        label: "equinox_noon",
        timestamp_utc: "2015-01-11T12:00:00Z",
        is_daytime: true,
        duration_s: 300.0,
        p_heat_source_w: 410.0,
        p_pumps_w: 3_400.0, // active fertigation + UV disinfection skid
        p_fans_w: 1_200.0,
        p_motors_w: 450.0, // roof vent rack-and-pinion actuation
        p_other_w: 350.0,
        q_heating_thermal_w: 24_883.2,
    },
    LoadSpec {
        label: "summer_afternoon",
        timestamp_utc: "2015-07-16T13:00:00Z",
        is_daytime: true,
        duration_s: 300.0,
        p_heat_source_w: 0.0,
        p_pumps_w: 4_200.0, // peak daytime irrigation + drain recirculation + UV sterilization
        p_fans_w: 2_200.0,  // full air circulationdestratification fans
        p_motors_w: 950.0,  // shading screen + full roof vent actuation
        p_other_w: 350.0,
        q_heating_thermal_w: 0.0,
    },
];

fn evaluate_load(spec: &LoadSpec) -> PeriodElectricalThermalRecord {
    let p_total_w = spec.p_heat_source_w
        + spec.p_pumps_w
        + spec.p_fans_w
        + spec.p_motors_w
        + spec.p_other_w;
    let e_kwh = (p_total_w * spec.duration_s) / 3_600_000.0;
    let q_kwh_th = (spec.q_heating_thermal_w * spec.duration_s) / 3_600_000.0;

    PeriodElectricalThermalRecord {
        period_label: spec.label.to_string(),
        timestamp_utc: spec.timestamp_utc.to_string(),
        is_daytime: spec.is_daytime,
        duration_s: spec.duration_s,
        p_heat_source_electrical_w: spec.p_heat_source_w,
        p_pumps_w: spec.p_pumps_w,
        p_fans_w: spec.p_fans_w,
        p_motors_w: spec.p_motors_w,
        p_other_controls_w: spec.p_other_w,
        p_total_electrical_w: p_total_w,
        p_total_electrical_kw: p_total_w / 1_000.0,
        step_electrical_energy_kwh: e_kwh,
        useful_heating_thermal_power_w: spec.q_heating_thermal_w,
        step_heating_thermal_energy_kwh_th: q_kwh_th,
    }
}

pub fn calculate_energy_service(
    input: CalculateEnergyInput,
) -> Result<CalculateEnergyOutput, String> {
    let preset_id = input
        .preset_id
        .unwrap_or_else(|| DEFAULT_PRESET_ID.to_string());
    if preset_id != DEFAULT_PRESET_ID {
        return Err(format!(
            "Unsupported preset_id '{}'. Expected '{}'.",
            preset_id, DEFAULT_PRESET_ID
        ));
    }

    let all_records: Vec<PeriodElectricalThermalRecord> =
        LOAD_SPECS.iter().map(evaluate_load).collect();

    // P_peak = max_t sum_i P_i(t) vs non-coincident sum_i max_t P_i(t)
    let mut coincident_peak_kw = 0.0;
    let mut coincident_peak_label = String::new();
    for r in &all_records {
        if r.p_total_electrical_kw > coincident_peak_kw {
            coincident_peak_kw = r.p_total_electrical_kw;
            coincident_peak_label = r.period_label.clone();
        }
    }

    let max_heat_source_w = LOAD_SPECS.iter().map(|s| s.p_heat_source_w).fold(0.0, f64::max);
    let max_pumps_w = LOAD_SPECS.iter().map(|s| s.p_pumps_w).fold(0.0, f64::max);
    let max_fans_w = LOAD_SPECS.iter().map(|s| s.p_fans_w).fold(0.0, f64::max);
    let max_motors_w = LOAD_SPECS.iter().map(|s| s.p_motors_w).fold(0.0, f64::max);
    let max_other_w = LOAD_SPECS.iter().map(|s| s.p_other_w).fold(0.0, f64::max);
    let non_coincident_sum_kw =
        (max_heat_source_w + max_pumps_w + max_fans_w + max_motors_w + max_other_w) / 1_000.0;

    let mut filtered_records = all_records;
    if let Some(ref filter) = input.period_filter {
        filtered_records.retain(|r| &r.period_label == filter);
        if filtered_records.is_empty() {
            return Err(format!(
                "Unknown period_filter '{}'. Valid values: winter_night, winter_noon, equinox_noon, summer_afternoon.",
                filter
            ));
        }
    }

    let cycle_days = 333.0;
    let daytime_kwh = 19_647.0;
    let nighttime_kwh = 14_319.0;
    let total_kwh = daytime_kwh + nighttime_kwh;
    let total_heating_kwh_th = 248_832.0;

    Ok(CalculateEnergyOutput {
        preset_id,
        floor_area_m2: FLOOR_AREA_M2,
        cultivated_area_m2: CULTIVATED_AREA_M2,
        period_load_records: filtered_records,
        peak_demand_analysis: PeakDemandAnalysis {
            coincident_peak_demand_kw: coincident_peak_kw,
            coincident_peak_period_label: coincident_peak_label,
            non_coincident_device_sum_kw: non_coincident_sum_kw,
            coincidence_factor: coincident_peak_kw / non_coincident_sum_kw,
            explanation: "Peak demand is the maximum simultaneous combined load across timestamps, not the non-coincident sum of individual device maxima.".to_string(),
        },
        seasonal_energy_summary: SeasonalEnergySummary {
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coincident_peak_is_strictly_less_than_non_coincident_sum() {
        let out = calculate_energy_service(CalculateEnergyInput::default()).unwrap();
        assert!(
            out.peak_demand_analysis.coincident_peak_demand_kw
                < out.peak_demand_analysis.non_coincident_device_sum_kw
        );
        assert!((out.peak_demand_analysis.coincident_peak_demand_kw - 7.7).abs() < 1e-12);
        assert!((out.peak_demand_analysis.non_coincident_device_sum_kw - 9.55).abs() < 1e-12);
    }

    #[test]
    fn test_power_sum_and_kwh_integration() {
        let out = calculate_energy_service(CalculateEnergyInput::default()).unwrap();
        for rec in &out.period_load_records {
            let sum_w = rec.p_heat_source_electrical_w
                + rec.p_pumps_w
                + rec.p_fans_w
                + rec.p_motors_w
                + rec.p_other_controls_w;
            assert!((rec.p_total_electrical_w - sum_w).abs() < 1e-12);
            let expected_kwh = (sum_w * rec.duration_s) / 3_600_000.0;
            assert!((rec.step_electrical_energy_kwh - expected_kwh).abs() < 1e-12);
        }
    }
}