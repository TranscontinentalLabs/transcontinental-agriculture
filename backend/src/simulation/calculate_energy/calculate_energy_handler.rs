use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::calculate_energy_service::{calculate_energy_service, CalculateEnergyInput};

pub async fn calculate_energy_handler(
    payload: Option<Json<CalculateEnergyInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match calculate_energy_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}