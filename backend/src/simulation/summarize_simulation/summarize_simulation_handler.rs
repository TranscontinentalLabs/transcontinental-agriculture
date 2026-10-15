use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::summarize_simulation_service::{
    summarize_simulation_service, SummarizeSimulationInput,
};

pub async fn summarize_simulation_handler(
    payload: Option<Json<SummarizeSimulationInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match summarize_simulation_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}