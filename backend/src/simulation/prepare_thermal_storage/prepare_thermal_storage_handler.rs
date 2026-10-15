use axum::Json;

use crate::error::AppError;

use super::prepare_thermal_storage_service::{
    prepare_thermal_storage_service,
    PrepareThermalStorageInput,
    PrepareThermalStorageOutput,
};

pub async fn prepare_thermal_storage_handler(
    payload: Option<Json<PrepareThermalStorageInput>>,
) -> Result<Json<PrepareThermalStorageOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_thermal_storage_service(input)?;

    Ok(Json(result))
}