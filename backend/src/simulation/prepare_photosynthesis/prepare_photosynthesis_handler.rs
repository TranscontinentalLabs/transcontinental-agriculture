use axum::Json;

use crate::error::AppError;

use super::prepare_photosynthesis_service::{
    prepare_photosynthesis_service,
    PreparePhotosynthesisInput,
    PreparePhotosynthesisOutput,
};

pub async fn prepare_photosynthesis_handler(
    payload: Option<Json<PreparePhotosynthesisInput>>,
) -> Result<Json<PreparePhotosynthesisOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_photosynthesis_service(input)?;

    Ok(Json(result))
}