use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use super::assemble_planning_report_service::{
    assemble_planning_report_service, AssemblePlanningReportInput,
};

pub async fn assemble_planning_report_handler(
    payload: Option<Json<AssemblePlanningReportInput>>,
) -> impl IntoResponse {
    let input = payload.map(|Json(v)| v).unwrap_or_default();
    match assemble_planning_report_service(input) {
        Ok(output) => (StatusCode::OK, Json(json!(output))).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": err })),
        )
            .into_response(),
    }
}