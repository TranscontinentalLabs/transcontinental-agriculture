use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::lookup_solar_package_service::{
    lookup_solar_package_service, LookupSolarPackageInput,
};

pub async fn lookup_solar_package_handler(
    payload: Option<Json<LookupSolarPackageInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match lookup_solar_package_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}