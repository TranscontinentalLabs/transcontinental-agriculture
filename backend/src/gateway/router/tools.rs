use axum::Router;
use axum::routing::post;
use crate::state::AppState;
use crate::tools::simulation::start_simulation::start_simulation;
use crate::tools::vdp::vdp_handler::calculate_vdp_handler;

pub fn tools_routes() -> Router<AppState>{
    Router::new()
        .route("/start-simulation", post(start_simulation))
        .route("/calculate-vdp", post(calculate_vdp_handler))
}