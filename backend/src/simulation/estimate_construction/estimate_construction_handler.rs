use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::estimate_construction_service::{
    estimate_construction_service, EstimateConstructionInput,
};

pub async fn estimate_construction_handler(
    payload: Option<Json<EstimateConstructionInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match estimate_construction_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}