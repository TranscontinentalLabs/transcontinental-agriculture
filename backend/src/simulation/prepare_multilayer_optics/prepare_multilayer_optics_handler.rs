use axum::Json;

use crate::error::AppError;

use super::prepare_multilayer_optics_service::{
    prepare_multilayer_optics_service,
    PrepareMultilayerOpticsInput,
    PrepareMultilayerOpticsOutput,
};

pub async fn prepare_multilayer_optics_handler(
    payload: Option<Json<PrepareMultilayerOpticsInput>>,
) -> Result<Json<PrepareMultilayerOpticsOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_multilayer_optics_service(input)?;

    Ok(Json(result))
}