use axum::Json;

use crate::error::AppError;

use super::prepare_air_leakage_service::{
    prepare_air_leakage_service,
    PrepareAirLeakageInput,
    PrepareAirLeakageOutput,
};

pub async fn prepare_air_leakage_handler(
    payload: Option<Json<PrepareAirLeakageInput>>,
) -> Result<Json<PrepareAirLeakageOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_air_leakage_service(input)?;

    Ok(Json(result))
}