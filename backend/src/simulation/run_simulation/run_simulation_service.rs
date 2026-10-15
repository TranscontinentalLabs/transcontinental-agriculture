use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::simulation::get_crop_management_plan::get_crop_management_plan_service::{
    get_crop_management_plan_service, GetCropManagementPlanInput,
};
use crate::simulation::get_greenhouse_configuration::get_greenhouse_configuration_service::{
    get_greenhouse_configuration_service, GreenhouseConfigurationInput,
};
use crate::simulation::prepare_air_volume::prepare_air_volume_service::{
    prepare_air_volume_service, PrepareAirVolumeInput,
};
use crate::simulation::prepare_carbohydrate_allocation::prepare_carbohydrate_allocation_service::{
    prepare_carbohydrate_allocation_service, PrepareCarbohydrateAllocationInput,
};
use crate::simulation::prepare_co2_exchange::prepare_co2_exchange_service::{
    prepare_co2_exchange_service, PrepareCo2ExchangeInput,
};
use crate::simulation::prepare_crop_transpiration::prepare_crop_transpiration_service::{
    prepare_crop_transpiration_service, PrepareCropTranspirationInput,
};
use crate::simulation::prepare_fruit_harvest::prepare_fruit_harvest_service::{
    prepare_fruit_harvest_service, PrepareFruitHarvestInput,
};
use crate::simulation::prepare_heating_pipes::prepare_heating_pipes_service::{
    prepare_heating_pipes_service, PrepareHeatingPipesInput,
};
use crate::simulation::prepare_multilayer_optics::prepare_multilayer_optics_service::{
    prepare_multilayer_optics_service, PrepareMultilayerOpticsInput,
};
use crate::simulation::prepare_natural_ventilation::prepare_natural_ventilation_service::{
    prepare_natural_ventilation_service, PrepareNaturalVentilationInput,
};
use crate::simulation::prepare_photosynthesis::prepare_photosynthesis_service::{
    prepare_photosynthesis_service, PreparePhotosynthesisInput,
};
use crate::simulation::prepare_screen_air_exchange::prepare_screen_air_exchange_service::{
    prepare_screen_air_exchange_service, PrepareScreenAirExchangeInput,
};
use crate::simulation::prepare_solar_geometry::prepare_diffuse_solar_geometry_service::{
    prepare_diffuse_solar_geometry_service, PrepareDiffuseSolarGeometryInput,
};
use crate::simulation::prepare_weather::prepare_weather_service::{
    prepare_weather_service, PrepareWeatherInput,
};

pub const DEFAULT_RUN_SIMULATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSimulationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–23 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CoupledSolverDiagnosticsOutput {
    pub reference_modelica_path: &'static str,
    pub modelica_standard_version: &'static str,
    pub time_dependent_variables_count: u32,
    pub solver_tolerance: f64,
    pub output_sampling_interval_s: u32,
    pub simulation_duration_days: u16,
    pub dassl_runtime_minutes_i7_1165g7: f64,
    pub cvode_runtime_minutes_i7_1165g7: f64,
    pub coupling_architecture_note: &'static str,
}

/// Section 24 ten-step causal feedback loop trace at a single timestamp.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct TenStepFeedbackLoopTraceOutput {
    /// Step 1: Outdoor sunlight (GHI, W/m²).
    pub step_1_outdoor_ghi_w_m2: f64,

    /// Step 2: Roof admits calculated direct + diffuse fraction (I_roof,in, W/m²).
    pub step_2_roof_transmitted_irradiance_w_m2: f64,

    /// Step 3: Screens and leaves absorb or reflect shortwave energy (canopy absorbed W/m²_floor).
    pub step_3_canopy_absorbed_shortwave_floor_w_m2: f64,

    /// Step 4: Leaf temperature evolves relative to MainAir (°C).
    pub step_4_leaf_temperature_c: f64,

    /// Step 5: Transpiration adds moisture (kg/s) and removes latent heat (W) from canopy.
    pub step_5_transpiration_kg_s: f64,
    pub step_5_canopy_latent_cooling_w: f64,

    /// Step 6: Indoor air conditions evolve (MainAir T, RH, VPD_air, VPD_leaf-air).
    pub step_6_main_air_temperature_c: f64,
    pub step_6_main_air_rh_pct: f64,
    pub step_6_vpd_leaf_air_pa: f64,

    /// Step 7: Operating policy / actuators position vents, screens, and heating power.
    pub step_7_leeward_vent_deg: f64,
    pub step_7_shading_screen_u: f64,
    pub step_7_thermal_screen_u: f64,
    pub step_7_heating_power_w: f64,

    /// Step 8: Actions alter ventilation (m³/s), screen exchange (m³/s), and CO2 exchange.
    pub step_8_ventilation_airflow_m3_s: f64,
    pub step_8_screen_exchange_m3_s: f64,

    /// Step 9: Resulting light, leaf temperature, and CO2 drive photosynthesis and organ allocation.
    pub step_9_photosynthesis_ch2o_mg_m2_s: f64,
    pub step_9_fruit_allocation_ch2o_mg_m2_s: f64,

    /// Step 10: Vegetative allocation and pruning update LAI (m²/m²) and crop height (m) for the next step.
    pub step_10_leaf_area_index: f64,
    pub step_10_dlai_dt_per_day: f64,
    pub step_10_crop_height_m: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalClimateFieldsOutput {
    /// MainAir.T converted from K to °C.
    pub main_air_temperature_c: f64,

    /// TopAir.T converted from K to °C.
    pub top_air_temperature_c: f64,

    /// MainAir.RH converted from fraction to percent.
    pub relative_humidity_pct: f64,

    /// MainAir.CO2_ppm.
    pub co2_ppm: f64,

    pub vpd_air_pa: f64,
    pub vpd_leaf_air_pa: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalCropFieldsOutput {
    /// Canopy.T (°C).
    pub leaf_temperature_c: f64,

    /// TYM.LAI (m²_leaf / m²_cultivated).
    pub leaf_area_index: f64,

    pub scheduled_crop_height_m: f64,
    pub t_24_filtered_leaf_temperature_c: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalLightFieldsOutput {
    /// ML_PAR.R_PAR_Can_umol on greenhouse floor basis, umol/(m²_floor·s).
    pub absorbed_par_umol_m2_floor_s: f64,

    /// Absorbed PAR on cultivated area basis, umol/(m²_cultivated·s).
    pub absorbed_par_umol_m2_cultivated_s: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalWaterFieldsOutput {
    /// CanTranspi.mW_flow (whole cultivated crop, kg/s).
    pub transpiration_kg_s: f64,

    /// TotalCanopyTranspiration integrated up to this timestamp (kg).
    pub cumulative_transpiration_kg: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalHarvestFieldsOutput {
    /// TYM.DM_Har (mg DM / m²_cultivated).
    pub dry_matter_mg_m2: f64,

    /// TYM.N_Har (fruits / m²_cultivated).
    pub fruit_count_m2: f64,

    /// HarvestedFruitsFreshWeightPerSqMeter_simu (kg fresh / m²_cultivated, f_DM = 5.4%).
    pub fresh_kg_m2: f64,

    pub whole_greenhouse_fresh_kg: f64,
    pub whole_greenhouse_marketable_kg: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CanonicalEnergyAndCo2FieldsOutput {
    /// Instantaneous total useful pipe heating power (W) and 5-min interval thermal kWh.
    pub instantaneous_thermal_power_w: f64,
    pub interval_5min_thermal_kwh: f64,

    /// Net crop CO2 uptake rate (kg/s) and cumulative injected CO2 (kg).
    pub crop_co2_uptake_kg_s: f64,
    pub cumulative_injected_co2_kg: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoupledSimulationTimestepRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Section 27 canonical API fields.
    pub climate: CanonicalClimateFieldsOutput,
    pub crop: CanonicalCropFieldsOutput,
    pub light: CanonicalLightFieldsOutput,
    pub water: CanonicalWaterFieldsOutput,
    pub harvest: CanonicalHarvestFieldsOutput,
    pub energy_and_co2: CanonicalEnergyAndCo2FieldsOutput,

    /// Section 24 ten-step causal feedback trace.
    pub feedback_loop_trace: TenStepFeedbackLoopTraceOutput,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CondensationBySurfaceSeasonKgOutput {
    pub roof_inner_kg: f64,
    pub south_wall_glass_kg: f64,
    pub south_wall_curtain_kg: f64,
    pub east_corridor_wall_kg: f64,
    pub north_corridor_wall_kg: f64,
    pub horizontal_screens_kg: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CoupledSeasonTotalsSummaryOutput {
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub enclosed_volume_m3: f64,
    pub season_fresh_harvest_kg_m2: f64,
    pub season_whole_greenhouse_fresh_harvest_kg: f64,
    pub season_whole_greenhouse_marketable_harvest_kg: f64,
    pub season_harvested_fruit_count_m2: f64,
    pub season_measured_net_irrigation_m3: f64,
    pub season_simulated_water_consumption_range_m3: [f64; 2],
    pub condensation_by_surface_kg: CondensationBySurfaceSeasonKgOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunSimulationOutput {
    pub run_id: &'static str,
    pub status: &'static str,
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub solver_diagnostics: CoupledSolverDiagnosticsOutput,
    pub season_summary: CoupledSeasonTotalsSummaryOutput,
    pub records: Vec<CoupledSimulationTimestepRecord>,
}

pub fn run_simulation_service(
    payload: RunSimulationInput,
) -> Result<RunSimulationOutput, AppError> {
    let preset_opt = payload.preset_id.clone();
    let filter_opt = payload.period_filter.clone();

    // Orchestrate the upstream simulation services to assemble the coupled records.
    let geometry = get_greenhouse_configuration_service(
        GreenhouseConfigurationInput {
            preset_id: preset_opt.clone(),
        },
    )?;
    let weather = prepare_weather_service(PrepareWeatherInput {
        preset_id: preset_opt.clone(),
        period_filter: filter_opt.clone(),
    })?;
    let diffuse_solar = prepare_diffuse_solar_geometry_service(
        PrepareDiffuseSolarGeometryInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        },
    )?;
    let multilayer =
        prepare_multilayer_optics_service(PrepareMultilayerOpticsInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let air_volume = prepare_air_volume_service(PrepareAirVolumeInput {
        preset_id: preset_opt.clone(),
        period_filter: filter_opt.clone(),
    })?;
    let ventilation =
        prepare_natural_ventilation_service(PrepareNaturalVentilationInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let screen_exchange =
        prepare_screen_air_exchange_service(PrepareScreenAirExchangeInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let heating = prepare_heating_pipes_service(PrepareHeatingPipesInput {
        preset_id: preset_opt.clone(),
        period_filter: filter_opt.clone(),
    })?;
    let transpiration =
        prepare_crop_transpiration_service(PrepareCropTranspirationInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let photosynthesis =
        prepare_photosynthesis_service(PreparePhotosynthesisInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let allocation = prepare_carbohydrate_allocation_service(
        PrepareCarbohydrateAllocationInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        },
    )?;
    let management =
        get_crop_management_plan_service(GetCropManagementPlanInput {
            preset_id: preset_opt.clone(),
            period_filter: filter_opt.clone(),
        })?;
    let harvest = prepare_fruit_harvest_service(PrepareFruitHarvestInput {
        preset_id: preset_opt.clone(),
        period_filter: filter_opt.clone(),
    })?;
    let co2 = prepare_co2_exchange_service(PrepareCo2ExchangeInput {
        preset_id: preset_opt,
        period_filter: filter_opt,
    })?;

    let count = weather.records.len();
    let mut records = Vec::with_capacity(count);

    for i in 0..count {
        let w = &weather.records[i];
        let ds = &diffuse_solar.records[i];
        let ml = &multilayer.records[i];
        let av = &air_volume.records[i];
        let vn = &ventilation.records[i];
        let sc = &screen_exchange.records[i];
        let hp = &heating.records[i];
        let tr = &transpiration.records[i];
        let ph = &photosynthesis.records[i];
        let al = &allocation.records[i];
        let mg = &management.records[i];
        let hv = &harvest.records[i];
        let cz = &co2.records[i];

        // Cross-model coupling sanity check: timestamps and LAI must agree across sub-models.
        if (w.time_s - ml.time_s).abs() > 1e-6
            || (ml.leaf_area_index - mg.leaf_area_index).abs() > 1e-9
            || (av.leaf_temperature_c - ph.leaf_temperature_c).abs() > 1e-9
        {
            return Err(AppError::Internal(format!(
                "coupled state mismatch at record {i} ({})",
                w.period_label
            )));
        }

        let interval_5min_thermal_kwh =
            (hp.source_conversion.total_useful_heating_power_w * 300.0) / 3.6e6;

        records.push(CoupledSimulationTimestepRecord {
            period_label: w.period_label,
            timestamp_iso_utc: w.timestamp_iso_utc,
            timestamp_utc_s: w.timestamp_utc_s,
            time_s: w.time_s,
            climate: CanonicalClimateFieldsOutput {
                main_air_temperature_c: av.main_air.temperature_c,
                top_air_temperature_c: av.top_air.temperature_c,
                relative_humidity_pct: av.main_air.relative_humidity_pct,
                co2_ppm: av.main_air.co2_ppm,
                vpd_air_pa: av.main_air.vpd_air_pa,
                vpd_leaf_air_pa: av.vpd_leaf_air_pa,
            },
            crop: CanonicalCropFieldsOutput {
                leaf_temperature_c: av.leaf_temperature_c,
                leaf_area_index: mg.leaf_area_index,
                scheduled_crop_height_m: mg.scheduled_crop_height_m,
                t_24_filtered_leaf_temperature_c: al.t_24_c,
            },
            light: CanonicalLightFieldsOutput {
                absorbed_par_umol_m2_floor_s: ml
                    .absorbed_canopy_ppfd_umol_m2_floor_s,
                absorbed_par_umol_m2_cultivated_s: ml
                    .absorbed_canopy_ppfd_umol_m2_cultivated_s,
            },
            water: CanonicalWaterFieldsOutput {
                transpiration_kg_s: tr.transpiration_kg_s,
                cumulative_transpiration_kg: tr.cumulative_transpiration_kg,
            },
            harvest: CanonicalHarvestFieldsOutput {
                dry_matter_mg_m2: hv.dry_matter_mg_m2,
                fruit_count_m2: hv.fruit_count_m2,
                fresh_kg_m2: hv.fresh_kg_m2,
                whole_greenhouse_fresh_kg: hv.whole_greenhouse_fresh_kg,
                whole_greenhouse_marketable_kg: hv
                    .whole_greenhouse_marketable_kg,
            },
            energy_and_co2: CanonicalEnergyAndCo2FieldsOutput {
                instantaneous_thermal_power_w: hp
                    .source_conversion
                    .total_useful_heating_power_w,
                interval_5min_thermal_kwh,
                crop_co2_uptake_kg_s: cz.m_dot_co2_crop_kg_s,
                cumulative_injected_co2_kg: cz.cumulative_injected_co2_kg,
            },
            feedback_loop_trace: TenStepFeedbackLoopTraceOutput {
                step_1_outdoor_ghi_w_m2: w
                    .global_horizontal_irradiance_w_m2
                    .value,
                step_2_roof_transmitted_irradiance_w_m2: ds
                    .transmitted_total_horizontal_w_m2,
                step_3_canopy_absorbed_shortwave_floor_w_m2: ml
                    .total_shortwave_absorbed_floor_w_m2
                    .canopy,
                step_4_leaf_temperature_c: av.leaf_temperature_c,
                step_5_transpiration_kg_s: tr.transpiration_kg_s,
                step_5_canopy_latent_cooling_w: tr.canopy_latent_cooling_w,
                step_6_main_air_temperature_c: av.main_air.temperature_c,
                step_6_main_air_rh_pct: av.main_air.relative_humidity_pct,
                step_6_vpd_leaf_air_pa: av.vpd_leaf_air_pa,
                step_7_leeward_vent_deg: vn.leeward.commanded_angle_deg,
                step_7_shading_screen_u: ml.shading_screen_deployment,
                step_7_thermal_screen_u: ml.thermal_screen_deployment,
                step_7_heating_power_w: hp
                    .source_conversion
                    .total_useful_heating_power_w,
                step_8_ventilation_airflow_m3_s: vn
                    .whole_greenhouse_airflow_m3_s,
                step_8_screen_exchange_m3_s: sc
                    .whole_compartment_screen_exchange_m3_s,
                step_9_photosynthesis_ch2o_mg_m2_s: ph.f_air_to_buffer_mg_m2_s,
                step_9_fruit_allocation_ch2o_mg_m2_s: al.f_fruit_mg_m2_s,
                step_10_leaf_area_index: mg.leaf_area_index,
                step_10_dlai_dt_per_day: mg.dlai_dt_per_day,
                step_10_crop_height_m: mg.scheduled_crop_height_m,
            },
        });
    }

    Ok(RunSimulationOutput {
        run_id: "ctifl_2014_2015_reference_run_001",
        status: "completed",
        preset_id: DEFAULT_RUN_SIMULATION_PRESET_ID,
        execution_mode: "reference_coupled_dataset_orchestration",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.1, Section 3.7.2, Section 4 & TiGwerLib 0.7.0 CTIFLGreenhouseBaseModel",
        solver_diagnostics: CoupledSolverDiagnosticsOutput {
            reference_modelica_path: "TiGwerLib.Applications.CTIFLGreenhouse.GlobalModels.UsingStatuses.CTIFLGreenhouseBaseModel",
            modelica_standard_version: "Modelica 4.0.0 + Buildings",
            time_dependent_variables_count: 2_890,
            solver_tolerance: 1e-6,
            output_sampling_interval_s: 300,
            simulation_duration_days: 334,
            dassl_runtime_minutes_i7_1165g7: 37.0,
            cvode_runtime_minutes_i7_1165g7: 20.0,
            coupling_architecture_note:
            "All 22 physical and biological sub-models form closed feedback loops solved simultaneously; \
                 the agent calls this single coupled simulation endpoint rather than guessing intermediate states.",
        },
        season_summary: CoupledSeasonTotalsSummaryOutput {
            floor_area_m2: geometry.floor_area_m2,
            cultivated_area_m2: geometry.cultivated_area_m2,
            enclosed_volume_m3: geometry.volume_m3,
            season_fresh_harvest_kg_m2: harvest
                .published_yield_validation
                .season_final_fresh_yield_kg_m2,
            season_whole_greenhouse_fresh_harvest_kg: harvest
                .published_yield_validation
                .season_final_whole_greenhouse_fresh_kg,
            season_whole_greenhouse_marketable_harvest_kg: harvest
                .published_yield_validation
                .season_final_whole_greenhouse_fresh_kg
                * harvest.parameters.default_marketable_fraction,
            season_harvested_fruit_count_m2: harvest
                .published_yield_validation
                .season_final_fruit_count_m2,
            season_measured_net_irrigation_m3: transpiration
                .published_water_validation
                .measured_net_irrigation_m3,
            season_simulated_water_consumption_range_m3: [
                transpiration
                    .published_water_validation
                    .simulated_water_consumption_min_m3,
                transpiration
                    .published_water_validation
                    .simulated_water_consumption_max_m3,
            ],
            condensation_by_surface_kg: CondensationBySurfaceSeasonKgOutput {
                roof_inner_kg: 89_905.0,
                south_wall_glass_kg: 7_202.0,
                south_wall_curtain_kg: 800.0,
                east_corridor_wall_kg: 920.0,
                north_corridor_wall_kg: 829.0,
                horizontal_screens_kg: 34.0,
            },
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orchestrates_all_submodels_and_verifies_ten_step_feedback_loop() {
        let output = run_simulation_service(RunSimulationInput::default())
            .expect("coupled simulation orchestration should succeed");

        assert_eq!(output.status, "completed");
        assert_eq!(output.records.len(), 4);
        assert!((output.season_summary.floor_area_m2 - 1_036.8).abs() < 1e-9);
        assert!(
            (output.season_summary.cultivated_area_m2 - 943.488).abs() < 1e-9
        );

        // Verify daytime feedback loop progression (equinox_noon = index 2)
        let eq = &output.records[2];
        assert!(eq.feedback_loop_trace.step_1_outdoor_ghi_w_m2 > 500.0);
        assert!(
            eq.feedback_loop_trace.step_2_roof_transmitted_irradiance_w_m2
                < eq.feedback_loop_trace.step_1_outdoor_ghi_w_m2
        );
        assert!(eq.feedback_loop_trace.step_5_transpiration_kg_s > 0.05);
        assert!(
            eq.feedback_loop_trace.step_9_photosynthesis_ch2o_mg_m2_s > 1.0
        );
        assert!((eq.crop.leaf_area_index - 3.2).abs() < 1e-12);
    }
}