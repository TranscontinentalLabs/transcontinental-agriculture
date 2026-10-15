use axum::Json;

use crate::error::AppError;

use super::prepare_air_volume_service::{
    prepare_air_volume_service,
    PrepareAirVolumeInput,
    PrepareAirVolumeOutput,
};

pub async fn prepare_air_volume_handler(
    payload: Option<Json<PrepareAirVolumeInput>>,
) -> Result<Json<PrepareAirVolumeOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_air_volume_service(input)?;

    Ok(Json(result))
}