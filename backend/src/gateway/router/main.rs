use axum::{Json, Router};
use axum::routing::get;
use serde_json::json;
use crate::gateway::router::messaging::messaging_routes;
use crate::gateway::router::simulation::simulation_routes;
use crate::gateway::router::tools::tools_routes;
use crate::state::AppState;

pub fn main_router() -> Router<AppState>{
    Router::new()
        .nest("/tools", tools_routes())
        .nest("/messaging", messaging_routes())
        .nest("/simulation", simulation_routes())
        
        .route("/health", get(health_check))
}

pub async fn health_check() -> Json<serde_json::Value>{
    Json(json!({
        "status": "ok"
    }))
}