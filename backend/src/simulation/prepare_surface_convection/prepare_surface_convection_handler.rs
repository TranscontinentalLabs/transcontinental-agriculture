use axum::Json;

use crate::error::AppError;

use super::prepare_surface_convection_service::{
    prepare_surface_convection_service,
    PrepareSurfaceConvectionInput,
    PrepareSurfaceConvectionOutput,
};

pub async fn prepare_surface_convection_handler(
    payload: Option<Json<PrepareSurfaceConvectionInput>>,
) -> Result<Json<PrepareSurfaceConvectionOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_surface_convection_service(input)?;

    Ok(Json(result))
}