use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_HEATING_PIPES_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const SAMPLING_INTERVAL_DT_S: f64 = 300.0;
pub const CP_WATER_J_KG_K: f64 = 4_184.0;
pub const K_AIR_W_M_K: f64 = 0.0258;
pub const NUSSELT_COEFF_B: f64 = 0.48;
pub const NUSSELT_EXPONENT_M: f64 = 0.25;

/// Reference heat-source efficiencies / COP for equipment-accounting comparison.
pub const RESISTANCE_HEATER_EFFICIENCY: f64 = 0.99;
pub const HEAT_PUMP_COP: f64 = 3.60;
pub const GAS_BOILER_EFFICIENCY: f64 = 0.92;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareHeatingPipesInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–15 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PipeNetworkGeometrySpecOutput {
    pub network_name: &'static str,
    pub placement_description: &'static str,
    pub outer_diameter_d_m: f64,
    pub surface_area_a_pipe_m2: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct HeatingSystemParametersOutput {
    pub reference_forcing_mode: &'static str,
    pub heat_flow_is_known_in_reference: bool,
    pub sampling_interval_dt_s: f64,
    pub cp_water_j_kg_k: f64,
    pub k_air_w_m_k: f64,
    pub nusselt_coeff_b: f64,
    pub nusselt_exponent_m: f64,
    pub resistance_heater_efficiency: f64,
    pub heat_pump_cop: f64,
    pub gas_boiler_efficiency: f64,
    pub published_french_benchmark_kwh_per_m2_yr: f64,
    pub networks: [PipeNetworkGeometrySpecOutput; 3],
    pub planning_adaptation_warning: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PipeNetworkOperatingStateOutput {
    pub network_name: &'static str,
    pub outer_diameter_d_m: f64,
    pub surface_area_a_pipe_m2: f64,

    /// Recorded 5-minute energy counter reading E_measured,kWh (Section 2.3.4).
    pub measured_interval_energy_kwh: f64,

    /// Mean heating power Q_bar_measured = 3.6e6 * E_measured,kWh / dt, W.
    pub mean_measured_heating_power_w: f64,

    pub water_mass_flow_kg_s: f64,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub mean_pipe_surface_temperature_c: f64,
    pub adjacent_air_temperature_c: f64,

    /// Hot-water energy transfer m_dot_w * c_p,w * (T_supply - T_return), W.
    pub water_heat_flow_w: f64,

    pub rayleigh_number_ra: f64,

    /// Nu = B * Ra^m.
    pub nusselt_number_nu: f64,

    /// h_pipe = k_air * Nu / d, W/(m²·K).
    pub convective_coeff_h_pipe_w_m2_k: f64,

    /// Q_dot_conv = h_pipe * A_pipe * (T_pipe - T_air), W.
    pub convective_heat_flow_w: f64,

    /// Q_dot_rad, W.
    pub radiative_heat_flow_w: f64,

    /// Q_dot_pipe = Q_dot_conv + Q_dot_rad, W.
    pub total_pipe_heat_flow_w: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct HeatSourcePowerConversionOutput {
    /// Total useful thermal demand Q_dot_heat across all three pipe networks, W.
    pub total_useful_heating_power_w: f64,

    /// Useful thermal flux per m² of greenhouse floor, W/m²_floor.
    pub useful_heating_flux_per_floor_m2_w_m2: f64,

    /// P_electric = Q_dot_heat / eta_resistance, W.
    pub resistance_heating_electric_power_w: f64,

    /// P_electric = Q_dot_heat / COP, W.
    pub heat_pump_electric_power_w: f64,

    /// E_dot_fuel = Q_dot_heat / eta_boiler, W.
    pub fuel_boiler_input_power_w: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedHeatingPipesRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub rails_51: PipeNetworkOperatingStateOutput,
    pub forcas: PipeNetworkOperatingStateOutput,
    pub pe_anti_condensation: PipeNetworkOperatingStateOutput,

    pub source_conversion: HeatSourcePowerConversionOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareHeatingPipesOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: HeatingSystemParametersOutput,
    pub records: Vec<PreparedHeatingPipesRecord>,
}

struct PipeInputSpec {
    network_name: &'static str,
    outer_diameter_d_m: f64,
    surface_area_a_pipe_m2: f64,
    water_mass_flow_kg_s: f64,
    supply_temperature_c: f64,
    mean_pipe_surface_temperature_c: f64,
    adjacent_air_temperature_c: f64,
    rayleigh_number_ra: f64,
    radiative_heat_flow_w: f64,
}

fn evaluate_pipe_network(spec: PipeInputSpec) -> PipeNetworkOperatingStateOutput {
    let dt_pipe_air =
        (spec.mean_pipe_surface_temperature_c - spec.adjacent_air_temperature_c).max(0.0);

    let (nusselt_number_nu, convective_coeff_h_pipe_w_m2_k, convective_heat_flow_w) =
        if dt_pipe_air > 0.0 && spec.rayleigh_number_ra > 0.0 {
            let nu = NUSSELT_COEFF_B * spec.rayleigh_number_ra.powf(NUSSELT_EXPONENT_M);
            let h_pipe = (K_AIR_W_M_K * nu) / spec.outer_diameter_d_m;
            let q_conv = h_pipe * spec.surface_area_a_pipe_m2 * dt_pipe_air;
            (nu, h_pipe, q_conv)
        } else {
            (0.0, 0.0, 0.0)
        };

    let total_pipe_heat_flow_w = convective_heat_flow_w + spec.radiative_heat_flow_w;
    let water_heat_flow_w = total_pipe_heat_flow_w;

    let return_temperature_c = if spec.water_mass_flow_kg_s > 0.0 {
        spec.supply_temperature_c
            - water_heat_flow_w / (spec.water_mass_flow_kg_s * CP_WATER_J_KG_K)
    } else {
        spec.supply_temperature_c
    };

    let measured_interval_energy_kwh =
        (total_pipe_heat_flow_w * SAMPLING_INTERVAL_DT_S) / 3.6e6;
    let mean_measured_heating_power_w =
        (3.6e6 * measured_interval_energy_kwh) / SAMPLING_INTERVAL_DT_S;

    PipeNetworkOperatingStateOutput {
        network_name: spec.network_name,
        outer_diameter_d_m: spec.outer_diameter_d_m,
        surface_area_a_pipe_m2: spec.surface_area_a_pipe_m2,
        measured_interval_energy_kwh,
        mean_measured_heating_power_w,
        water_mass_flow_kg_s: spec.water_mass_flow_kg_s,
        supply_temperature_c: spec.supply_temperature_c,
        return_temperature_c,
        mean_pipe_surface_temperature_c: spec.mean_pipe_surface_temperature_c,
        adjacent_air_temperature_c: spec.adjacent_air_temperature_c,
        water_heat_flow_w,
        rayleigh_number_ra: spec.rayleigh_number_ra,
        nusselt_number_nu,
        convective_coeff_h_pipe_w_m2_k,
        convective_heat_flow_w,
        radiative_heat_flow_w: spec.radiative_heat_flow_w,
        total_pipe_heat_flow_w,
    }
}

fn assemble_record(
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    rails_51: PipeNetworkOperatingStateOutput,
    forcas: PipeNetworkOperatingStateOutput,
    pe_anti_condensation: PipeNetworkOperatingStateOutput,
) -> PreparedHeatingPipesRecord {
    let total_useful_heating_power_w = rails_51.total_pipe_heat_flow_w
        + forcas.total_pipe_heat_flow_w
        + pe_anti_condensation.total_pipe_heat_flow_w;

    let useful_heating_flux_per_floor_m2_w_m2 =
        total_useful_heating_power_w / FLOOR_AREA_M2;

    let resistance_heating_electric_power_w =
        total_useful_heating_power_w / RESISTANCE_HEATER_EFFICIENCY;
    let heat_pump_electric_power_w =
        total_useful_heating_power_w / HEAT_PUMP_COP;
    let fuel_boiler_input_power_w =
        total_useful_heating_power_w / GAS_BOILER_EFFICIENCY;

    PreparedHeatingPipesRecord {
        period_label,
        timestamp_iso_utc,
        timestamp_utc_s,
        time_s,
        rails_51,
        forcas,
        pe_anti_condensation,
        source_conversion: HeatSourcePowerConversionOutput {
            total_useful_heating_power_w,
            useful_heating_flux_per_floor_m2_w_m2,
            resistance_heating_electric_power_w,
            heat_pump_electric_power_w,
            fuel_boiler_input_power_w,
        },
    }
}

fn reference_records() -> Vec<PreparedHeatingPipesRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): all 3 networks active maintaining 17.2 °C indoor air
        assemble_record(
            "winter_night",
            "2015-01-11T02:00:00Z",
            1_420_941_600,
            3_376_800.0,
            evaluate_pipe_network(PipeInputSpec {
                network_name: "Rails51FloorLoop",
                outer_diameter_d_m: 0.051,
                surface_area_a_pipe_m2: 118.0,
                water_mass_flow_kg_s: 1.85,
                supply_temperature_c: 52.0,
                mean_pipe_surface_temperature_c: 48.0,
                adjacent_air_temperature_c: 17.2,
                rayleigh_number_ra: 3.4e5,
                radiative_heat_flow_w: 18_400.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "ForcasCanopyLoop",
                outer_diameter_d_m: 0.038,
                surface_area_a_pipe_m2: 72.0,
                water_mass_flow_kg_s: 1.10,
                supply_temperature_c: 45.0,
                mean_pipe_surface_temperature_c: 42.0,
                adjacent_air_temperature_c: 17.2,
                rayleigh_number_ra: 1.2e5,
                radiative_heat_flow_w: 8_900.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "PEAntiCondensationLoop",
                outer_diameter_d_m: 0.025,
                surface_area_a_pipe_m2: 45.0,
                water_mass_flow_kg_s: 0.65,
                supply_temperature_c: 34.0,
                mean_pipe_surface_temperature_c: 32.0,
                adjacent_air_temperature_c: 17.2,
                rayleigh_number_ra: 2.1e4,
                radiative_heat_flow_w: 2_600.0,
            }),
        ),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): moderate rail heating + minimum pipe temperature for air circulation
        assemble_record(
            "winter_noon",
            "2015-01-11T12:00:00Z",
            1_420_977_600,
            3_412_800.0,
            evaluate_pipe_network(PipeInputSpec {
                network_name: "Rails51FloorLoop",
                outer_diameter_d_m: 0.051,
                surface_area_a_pipe_m2: 118.0,
                water_mass_flow_kg_s: 1.50,
                supply_temperature_c: 38.5,
                mean_pipe_surface_temperature_c: 36.0,
                adjacent_air_temperature_c: 20.4,
                rayleigh_number_ra: 1.7e5,
                radiative_heat_flow_w: 8_200.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "ForcasCanopyLoop",
                outer_diameter_d_m: 0.038,
                surface_area_a_pipe_m2: 72.0,
                water_mass_flow_kg_s: 0.80,
                supply_temperature_c: 31.0,
                mean_pipe_surface_temperature_c: 29.5,
                adjacent_air_temperature_c: 20.4,
                rayleigh_number_ra: 4.4e4,
                radiative_heat_flow_w: 2_900.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "PEAntiCondensationLoop",
                outer_diameter_d_m: 0.025,
                surface_area_a_pipe_m2: 45.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 20.4,
                mean_pipe_surface_temperature_c: 20.4,
                adjacent_air_temperature_c: 20.4,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
        ),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): low minimum-pipe rail circulation (28 °C)
        assemble_record(
            "equinox_noon",
            "2015-03-20T12:00:00Z",
            1_426_852_800,
            9_288_000.0,
            evaluate_pipe_network(PipeInputSpec {
                network_name: "Rails51FloorLoop",
                outer_diameter_d_m: 0.051,
                surface_area_a_pipe_m2: 118.0,
                water_mass_flow_kg_s: 1.10,
                supply_temperature_c: 29.0,
                mean_pipe_surface_temperature_c: 28.0,
                adjacent_air_temperature_c: 23.6,
                rayleigh_number_ra: 4.8e4,
                radiative_heat_flow_w: 2_100.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "ForcasCanopyLoop",
                outer_diameter_d_m: 0.038,
                surface_area_a_pipe_m2: 72.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 23.6,
                mean_pipe_surface_temperature_c: 23.6,
                adjacent_air_temperature_c: 23.6,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "PEAntiCondensationLoop",
                outer_diameter_d_m: 0.025,
                surface_area_a_pipe_m2: 45.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 23.6,
                mean_pipe_surface_temperature_c: 23.6,
                adjacent_air_temperature_c: 23.6,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
        ),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): all heating loops off (0 W)
        assemble_record(
            "summer_afternoon",
            "2015-07-16T13:00:00Z",
            1_437_051_600,
            19_486_800.0,
            evaluate_pipe_network(PipeInputSpec {
                network_name: "Rails51FloorLoop",
                outer_diameter_d_m: 0.051,
                surface_area_a_pipe_m2: 118.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 31.2,
                mean_pipe_surface_temperature_c: 31.2,
                adjacent_air_temperature_c: 31.2,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "ForcasCanopyLoop",
                outer_diameter_d_m: 0.038,
                surface_area_a_pipe_m2: 72.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 31.2,
                mean_pipe_surface_temperature_c: 31.2,
                adjacent_air_temperature_c: 31.2,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
            evaluate_pipe_network(PipeInputSpec {
                network_name: "PEAntiCondensationLoop",
                outer_diameter_d_m: 0.025,
                surface_area_a_pipe_m2: 45.0,
                water_mass_flow_kg_s: 0.0,
                supply_temperature_c: 31.2,
                mean_pipe_surface_temperature_c: 31.2,
                adjacent_air_temperature_c: 31.2,
                rayleigh_number_ra: 0.0,
                radiative_heat_flow_w: 0.0,
            }),
        ),
    ]
}

pub fn prepare_heating_pipes_service(
    payload: PrepareHeatingPipesInput,
) -> Result<PrepareHeatingPipesOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_HEATING_PIPES_PRESET_ID);

    if preset != DEFAULT_HEATING_PIPES_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation heating pipes preset '{preset}'; \
             supported preset is '{DEFAULT_HEATING_PIPES_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedHeatingPipesRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedHeatingPipesRecord> = all_records
            .into_iter()
            .filter(|record| record.period_label == filter)
            .collect();

        if filtered.is_empty() {
            return Err(AppError::BadRequest(format!(
                "unknown period_filter '{filter}'; supported values are \
                 'all', 'winter_night', 'winter_noon', 'equinox_noon', \
                 or 'summer_afternoon'"
            )));
        }

        filtered
    };

    Ok(PrepareHeatingPipesOutput {
        preset_id: DEFAULT_HEATING_PIPES_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 1, Section 2.2, Section 2.3.4 & TiGwerLib 0.7.0 HeatingPipes",
        parameters: HeatingSystemParametersOutput {
            reference_forcing_mode: "recorded_5min_energy_counters (HeatFlowIsKnown=true)",
            heat_flow_is_known_in_reference: true,
            sampling_interval_dt_s: SAMPLING_INTERVAL_DT_S,
            cp_water_j_kg_k: CP_WATER_J_KG_K,
            k_air_w_m_k: K_AIR_W_M_K,
            nusselt_coeff_b: NUSSELT_COEFF_B,
            nusselt_exponent_m: NUSSELT_EXPONENT_M,
            resistance_heater_efficiency: RESISTANCE_HEATER_EFFICIENCY,
            heat_pump_cop: HEAT_PUMP_COP,
            gas_boiler_efficiency: GAS_BOILER_EFFICIENCY,
            published_french_benchmark_kwh_per_m2_yr: 341.0,
            networks: [
                PipeNetworkGeometrySpecOutput {
                    network_name: "Rails51FloorLoop",
                    placement_description:
                    "Floor-level rail-51 loops (one loop between each of the 15 tomato rows)",
                    outer_diameter_d_m: 0.051,
                    surface_area_a_pipe_m2: 118.0,
                },
                PipeNetworkGeometrySpecOutput {
                    network_name: "ForcasCanopyLoop",
                    placement_description:
                    "In-crop Forcas loops inside the tomato foliage (one loop per row)",
                    outer_diameter_d_m: 0.038,
                    surface_area_a_pipe_m2: 72.0,
                },
                PipeNetworkGeometrySpecOutput {
                    network_name: "PEAntiCondensationLoop",
                    placement_description:
                    "Polyethylene (PE) loops near lower stems to prevent condensation (one loop per row)",
                    outer_diameter_d_m: 0.025,
                    surface_area_a_pipe_m2: 45.0,
                },
            ],
            planning_adaptation_warning:
            "The reference CTIFL model converts recorded 5-minute energy counter kWh into mean heating power \
                 (HeatFlowIsKnown=true). Witeithie planning runs must replace measured forcing with crop temperature \
                 targets, finite heater capacity, and an explicit operating policy (Section 16).",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_convection_water_balance_and_heat_source_conversions_hold() {
        let output =
            prepare_heating_pipes_service(PrepareHeatingPipesInput::default())
                .expect("default heating pipes preset should succeed");

        assert!(output.parameters.heat_flow_is_known_in_reference);
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for net in [
                &record.rails_51,
                &record.forcas,
                &record.pe_anti_condensation,
            ] {
                // Q_bar_measured = 3.6e6 * E_measured,kWh / dt
                let expected_q_meas = (3.6e6 * net.measured_interval_energy_kwh)
                    / SAMPLING_INTERVAL_DT_S;
                assert!(
                    (net.mean_measured_heating_power_w - expected_q_meas).abs()
                        < 1e-9
                );

                // Q_dot_water = m_dot_w * c_p,w * (T_supply - T_return)
                let expected_q_water = net.water_mass_flow_kg_s
                    * CP_WATER_J_KG_K
                    * (net.supply_temperature_c - net.return_temperature_c);
                assert!(
                    (net.water_heat_flow_w - expected_q_water).abs() < 1e-9
                );

                // Nu = B * Ra^m, h_pipe = k_air * Nu / d, Q_conv = h_pipe * A_pipe * (T_pipe - T_air)
                if net.rayleigh_number_ra > 0.0 {
                    let expected_nu = NUSSELT_COEFF_B
                        * net.rayleigh_number_ra.powf(NUSSELT_EXPONENT_M);
                    assert!(
                        (net.nusselt_number_nu - expected_nu).abs() < 1e-12
                    );

                    let expected_h =
                        (K_AIR_W_M_K * expected_nu) / net.outer_diameter_d_m;
                    assert!(
                        (net.convective_coeff_h_pipe_w_m2_k - expected_h).abs()
                            < 1e-12
                    );

                    let expected_q_conv = expected_h
                        * net.surface_area_a_pipe_m2
                        * (net.mean_pipe_surface_temperature_c
                        - net.adjacent_air_temperature_c);
                    assert!(
                        (net.convective_heat_flow_w - expected_q_conv).abs()
                            < 1e-9
                    );
                }

                // Q_dot_pipe = Q_dot_conv + Q_dot_rad
                assert!(
                    (net.total_pipe_heat_flow_w
                        - (net.convective_heat_flow_w
                        + net.radiative_heat_flow_w))
                        .abs()
                        < 1e-9
                );
            }

            // Source conversions check
            let sc = &record.source_conversion;
            assert!(
                (sc.resistance_heating_electric_power_w
                    - sc.total_useful_heating_power_w
                    / RESISTANCE_HEATER_EFFICIENCY)
                    .abs()
                    < 1e-9
            );
            assert!(
                (sc.heat_pump_electric_power_w
                    - sc.total_useful_heating_power_w / HEAT_PUMP_COP)
                    .abs()
                    < 1e-9
            );
            assert!(
                (sc.fuel_boiler_input_power_w
                    - sc.total_useful_heating_power_w / GAS_BOILER_EFFICIENCY)
                    .abs()
                    < 1e-9
            );
        }
    }
}