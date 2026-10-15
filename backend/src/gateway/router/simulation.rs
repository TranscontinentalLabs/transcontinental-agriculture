use axum::routing::post;
use axum::Router;
use crate::simulation::assemble_planning_report::assemble_planning_report_handler::assemble_planning_report_handler;
use crate::simulation::calculate_energy::calculate_energy_handler::calculate_energy_handler;
use crate::simulation::estimate_construction::estimate_construction_handler::estimate_construction_handler;
use crate::simulation::get_crop_management_plan::get_crop_management_plan_handler::get_crop_management_plan_handler;
use crate::simulation::get_greenhouse_configuration::get_greenhouse_configuration_handler::get_greenhouse_configuration_handler;
use crate::simulation::lookup_solar_package::lookup_solar_package_handler::lookup_solar_package_handler;
use crate::simulation::prepare_air_leakage::prepare_air_leakage_handler::prepare_air_leakage_handler;
use crate::simulation::prepare_air_volume::prepare_air_volume_handler::prepare_air_volume_handler;
use crate::simulation::prepare_airflow_transport::prepare_airflow_transport_handler::prepare_airflow_transport_handler;
use crate::simulation::prepare_canopy_heat_balance::prepare_canopy_heat_balance_handler::prepare_canopy_heat_balance_handler;
use crate::simulation::prepare_carbohydrate_allocation::prepare_carbohydrate_allocation_handler::prepare_carbohydrate_allocation_handler;
use crate::simulation::prepare_co2_exchange::prepare_co2_exchange_handler::prepare_co2_exchange_handler;
use crate::simulation::prepare_condensation::prepare_condensation_handler::prepare_condensation_handler;
use crate::simulation::prepare_crop_transpiration::prepare_crop_transpiration_handler::prepare_crop_transpiration_handler;
use crate::simulation::prepare_fruit_harvest::prepare_fruit_harvest_handler::prepare_fruit_harvest_handler;
use crate::simulation::prepare_heating_pipes::prepare_heating_pipes_handler::prepare_heating_pipes_handler;
use crate::simulation::prepare_longwave_radiation::prepare_longwave_radiation_handler::prepare_longwave_radiation_handler;
use crate::simulation::prepare_multilayer_optics::prepare_multilayer_optics_handler::prepare_multilayer_optics_handler;
use crate::simulation::prepare_natural_ventilation::prepare_natural_ventilation_handler::prepare_natural_ventilation_handler;
use crate::simulation::prepare_photosynthesis::prepare_photosynthesis_handler::prepare_photosynthesis_handler;
use crate::simulation::prepare_screen_air_exchange::prepare_screen_air_exchange_handler::prepare_screen_air_exchange_handler;
use crate::simulation::prepare_solar_geometry::prepare_diffuse_solar_geometry_handler::prepare_diffuse_solar_geometry_handler;
use crate::simulation::prepare_solar_geometry::prepare_solar_geometry_handler::prepare_solar_geometry_handler;
use crate::simulation::prepare_surface_convection::prepare_surface_convection_handler::prepare_surface_convection_handler;
use crate::simulation::prepare_thermal_storage::prepare_thermal_storage_handler::prepare_thermal_storage_handler;
use crate::simulation::prepare_weather::prepare_weather_handler::prepare_weather_handler;
use crate::simulation::price_energy::price_energy_handler::price_energy_handler;
use crate::simulation::run_simulation::run_simulation_handler::run_simulation_handler;
use crate::simulation::summarize_simulation::summarize_simulation_handler::summarize_simulation_handler;
use crate::state::AppState;

pub fn simulation_routes() -> Router<AppState> {
    Router::new()
        .route("/get-greenhouse-configuration", post(get_greenhouse_configuration_handler))
        .route("/prepare-weather", post(prepare_weather_handler))
        .route(
            "/prepare-solar-geometry",
            post(prepare_solar_geometry_handler),
        )
        .route(
            "/prepare-solar-geometry/diffuse",
            post(prepare_diffuse_solar_geometry_handler),
        )
        .route(
            "/prepare-multilayer-optics",
            post(prepare_multilayer_optics_handler),
        )
        .route("/prepare-air-volume", post(prepare_air_volume_handler))
        .route(
            "/prepare-natural-ventilation",
            post(prepare_natural_ventilation_handler),
        )
        .route("/prepare-air-leakage", post(prepare_air_leakage_handler))
        .route(
            "/prepare-screen-air-exchange",
            post(prepare_screen_air_exchange_handler),
        )
        .route(
            "/prepare-airflow-transport",
            post(prepare_airflow_transport_handler),
        )
        .route(
            "/prepare-surface-convection",
            post(prepare_surface_convection_handler),
        )
        .route(
            "/prepare-longwave-radiation",
            post(prepare_longwave_radiation_handler),
        )
        .route("/prepare-condensation", post(prepare_condensation_handler))
        .route(
            "/prepare-thermal-storage",
            post(prepare_thermal_storage_handler),
        )
        .route(
            "/prepare-heating-pipes",
            post(prepare_heating_pipes_handler),
        )
        .route(
            "/prepare-canopy-heat-balance",
            post(prepare_canopy_heat_balance_handler),
        )
        .route(
            "/prepare-crop-transpiration",
            post(prepare_crop_transpiration_handler),
        )
        .route(
            "/prepare-photosynthesis",
            post(prepare_photosynthesis_handler),
        )
        .route(
            "/prepare-carbohydrate-allocation",
            post(prepare_carbohydrate_allocation_handler),
        )
        .route(
            "/get-crop-management-plan",
            post(get_crop_management_plan_handler),
        )
        .route(
            "/prepare-fruit-harvest",
            post(prepare_fruit_harvest_handler),
        )
        .route(
            "/prepare-co2-exchange",
            post(prepare_co2_exchange_handler),
        )
        .route("/run-simulation", post(run_simulation_handler))
        .route("/summarize-simulation", post(summarize_simulation_handler))
        .route("/calculate-energy", post(calculate_energy_handler))
        .route("/price-energy", post(price_energy_handler))
        .route("/lookup-solar-package", post(lookup_solar_package_handler))
        .route("/estimate-construction", post(estimate_construction_handler))
        .route("/assemble-planning-report", post(assemble_planning_report_handler))
}
