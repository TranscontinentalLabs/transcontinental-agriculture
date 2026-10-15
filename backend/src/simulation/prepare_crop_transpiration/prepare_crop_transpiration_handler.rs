use axum::Json;

use crate::error::AppError;

use super::prepare_crop_transpiration_service::{
    prepare_crop_transpiration_service,
    PrepareCropTranspirationInput,
    PrepareCropTranspirationOutput,
};

pub async fn prepare_crop_transpiration_handler(
    payload: Option<Json<PrepareCropTranspirationInput>>,
) -> Result<Json<PrepareCropTranspirationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_crop_transpiration_service(input)?;

    Ok(Json(result))
}