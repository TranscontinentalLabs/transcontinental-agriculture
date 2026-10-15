use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::price_energy_service::{price_energy_service, PriceEnergyInput};

pub async fn price_energy_handler(
    payload: Option<Json<PriceEnergyInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match price_energy_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}