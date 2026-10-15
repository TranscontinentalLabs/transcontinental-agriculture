use serde::{Deserialize, Serialize};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LookupSolarPackageInput {
    pub preset_id: Option<String>,
    /// Supported modes: "hybrid_critical_load_backup" (default), "grid_tied_daytime_offset", "off_grid_full_autonomy"
    pub preference_mode: Option<String>,
    pub required_backup_hours: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupSolarPackageOutput {
    pub preset_id: String,
    pub load_sizing_inputs: SolarLoadSizingInputs,
    pub selected_package: ReviewedSolarPackage,
    pub alternative_packages: Vec<ReviewedSolarPackage>,
    pub engineering_caveat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarLoadSizingInputs {
    pub daily_electricity_use_kwh_day: f64,
    pub daytime_consumption_kwh_day: f64,
    pub nighttime_consumption_kwh_day: f64,
    pub coincident_peak_demand_kw: f64,
    pub required_backup_period_hours: f64,
    pub preference_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewedSolarPackage {
    pub package_id: String,
    pub preference_mode: String,
    pub pv_array_capacity_kwp: f64,
    pub hybrid_inverter_rating_kw: f64,
    pub battery_nominal_capacity_kwh: f64,
    pub battery_usable_capacity_kwh_at_85pct_dod: f64,
    pub estimated_autonomous_night_backup_hours: f64,
    pub indicative_installed_cost_kes: f64,
    pub indicative_installed_cost_eur: f64,
    pub sizing_assumptions: String,
}

struct PackageSpec {
    package_id: &'static str,
    mode: &'static str,
    pv_kwp: f64,
    inverter_kw: f64,
    bess_nominal_kwh: f64,
    cost_kes: f64,
    cost_eur: f64,
    assumptions: &'static str,
}

const PACKAGE_SPECS: [PackageSpec; 3] = [
    PackageSpec {
        package_id: "venlo_pv25_inv12_bess0_grid_tied",
        mode: "grid_tied_daytime_offset",
        pv_kwp: 25.0,
        inverter_kw: 12.0,
        bess_nominal_kwh: 0.0,
        cost_kes: 2_850_000.0,
        cost_eur: 19_800.0,
        assumptions: "Offsets daytime fertigation pump and fan loads under 5.2 peak sun hours/day; relies on grid for all nighttime loads and cloud transients.",
    },
    PackageSpec {
        package_id: "venlo_pv35_inv15_bess60_hybrid",
        mode: "hybrid_critical_load_backup",
        pv_kwp: 35.0,
        inverter_kw: 15.0,
        bess_nominal_kwh: 60.0,
        cost_kes: 6_450_000.0,
        cost_eur: 44_900.0,
        assumptions: "Covers 100% of daytime load plus full 43.0 kWh average nighttime load at 85% LiFePO4 DoD (51.0 kWh usable), with grid/generator fallback for multi-day overcast weather.",
    },
    PackageSpec {
        package_id: "venlo_pv55_inv20_bess120_offgrid",
        mode: "off_grid_full_autonomy",
        pv_kwp: 55.0,
        inverter_kw: 20.0,
        bess_nominal_kwh: 120.0,
        cost_kes: 11_900_000.0,
        cost_eur: 82_800.0,
        assumptions: "Provides 102.0 kWh usable storage (~24 hours of full load autonomy) and 2.5x surge margin over 7.7 kW coincident peak.",
    },
];

fn evaluate_package(spec: &PackageSpec, avg_night_kw: f64) -> ReviewedSolarPackage {
    let usable_kwh = spec.bess_nominal_kwh * 0.85;
    let backup_hours = if avg_night_kw > 0.0 {
        usable_kwh / avg_night_kw
    } else {
        0.0
    };
    ReviewedSolarPackage {
        package_id: spec.package_id.to_string(),
        preference_mode: spec.mode.to_string(),
        pv_array_capacity_kwp: spec.pv_kwp,
        hybrid_inverter_rating_kw: spec.inverter_kw,
        battery_nominal_capacity_kwh: spec.bess_nominal_kwh,
        battery_usable_capacity_kwh_at_85pct_dod: usable_kwh,
        estimated_autonomous_night_backup_hours: backup_hours,
        indicative_installed_cost_kes: spec.cost_kes,
        indicative_installed_cost_eur: spec.cost_eur,
        sizing_assumptions: spec.assumptions.to_string(),
    }
}

pub fn lookup_solar_package_service(
    input: LookupSolarPackageInput,
) -> Result<LookupSolarPackageOutput, String> {
    let preset_id = input
        .preset_id
        .unwrap_or_else(|| DEFAULT_PRESET_ID.to_string());
    if preset_id != DEFAULT_PRESET_ID {
        return Err(format!(
            "Unsupported preset_id '{}'. Expected '{}'.",
            preset_id, DEFAULT_PRESET_ID
        ));
    }

    let mode = input
        .preference_mode
        .unwrap_or_else(|| "hybrid_critical_load_backup".to_string());
    let backup_hours = input.required_backup_hours.unwrap_or(12.0);
    if !backup_hours.is_finite() || backup_hours < 0.0 {
        return Err("required_backup_hours must be a non-negative finite number.".to_string());
    }

    let daytime_kwh_day = 19_647.0 / 333.0; // 59.0 kWh/day
    let nighttime_kwh_day = 14_319.0 / 333.0; // 43.0 kWh/day
    let daily_kwh_day = daytime_kwh_day + nighttime_kwh_day; // 102.0 kWh/day
    let coincident_peak_kw = 7.7;
    let avg_night_kw = nighttime_kwh_day / 12.0;

    let all_packages: Vec<ReviewedSolarPackage> = PACKAGE_SPECS
        .iter()
        .map(|s| evaluate_package(s, avg_night_kw))
        .collect();

    let selected = all_packages
        .iter()
        .find(|p| p.preference_mode == mode)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Unknown preference_mode '{}'. Valid values: grid_tied_daytime_offset, hybrid_critical_load_backup, off_grid_full_autonomy.",
                mode
            )
        })?;

    let alternatives = all_packages
        .into_iter()
        .filter(|p| p.preference_mode != mode)
        .collect();

    Ok(LookupSolarPackageOutput {
        preset_id,
        load_sizing_inputs: SolarLoadSizingInputs {
            daily_electricity_use_kwh_day: daily_kwh_day,
            daytime_consumption_kwh_day: daytime_kwh_day,
            nighttime_consumption_kwh_day: nighttime_kwh_day,
            coincident_peak_demand_kw: coincident_peak_kw,
            required_backup_period_hours: backup_hours,
            preference_mode: mode,
        },
        selected_package: selected,
        alternative_packages: alternatives,
        engineering_caveat: "Matching average daily kWh does not guarantee uninterrupted supply: 42.2% of greenhouse electrical load occurs at night (circulation pumps and boiler/climate fans), requiring battery storage or grid/generator support.".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_package_day_night_and_inverter_headroom() {
        let out = lookup_solar_package_service(LookupSolarPackageInput::default()).unwrap();
        let inputs = &out.load_sizing_inputs;
        assert!(
            (inputs.daily_electricity_use_kwh_day
                - (inputs.daytime_consumption_kwh_day + inputs.nighttime_consumption_kwh_day))
                .abs()
                < 1e-12
        );
        assert!(
            out.selected_package.hybrid_inverter_rating_kw > inputs.coincident_peak_demand_kw
        );
        assert!(
            out.selected_package.battery_usable_capacity_kwh_at_85pct_dod
                >= inputs.nighttime_consumption_kwh_day
        );
    }
}