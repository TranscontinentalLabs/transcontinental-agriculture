use axum::Json;

use crate::error::AppError;

use super::prepare_longwave_radiation_service::{
    prepare_longwave_radiation_service,
    PrepareLongwaveRadiationInput,
    PrepareLongwaveRadiationOutput,
};

pub async fn prepare_longwave_radiation_handler(
    payload: Option<Json<PrepareLongwaveRadiationInput>>,
) -> Result<Json<PrepareLongwaveRadiationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_longwave_radiation_service(input)?;

    Ok(Json(result))
}