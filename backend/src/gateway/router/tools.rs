use axum::Router;
use axum::routing::post;
use crate::state::AppState;
use crate::tools::get_greenhouse_configuration::get_greenhouse_configuration_handler::get_greenhouse_configuration_handler;
use crate::tools::prepare_solar_geometry::prepare_diffuse_solar_geometry_handler::prepare_diffuse_solar_geometry_handler;
use crate::tools::prepare_solar_geometry::prepare_solar_geometry_handler::prepare_solar_geometry_handler;
use crate::tools::prepare_weather::prepare_weather_handler::prepare_weather_handler;
use crate::tools::simulation::start_simulation::start_simulation;
use crate::tools::vdp::vdp_handler::calculate_vdp_handler;

pub fn tools_routes() -> Router<AppState>{
    Router::new()
        .route("/start-simulation", post(start_simulation))
        .route("/calculate-vdp", post(calculate_vdp_handler))
        .route("/tools/get-greenhouse-configuration",post(get_greenhouse_configuration_handler))
        .route("/prepare-weather", post(prepare_weather_handler))
        .route("/prepare-solar-geometry", post(prepare_solar_geometry_handler))
        .route("/prepare-solar-geometry/diffuse", post(prepare_diffuse_solar_geometry_handler))

}