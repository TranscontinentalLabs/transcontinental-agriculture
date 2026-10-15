use axum::Json;

use crate::error::AppError;

use super::prepare_solar_geometry_service::{
    prepare_solar_geometry_service,
    PrepareSolarGeometryInput,
    PrepareSolarGeometryOutput,
};

pub async fn prepare_solar_geometry_handler(
    payload: Option<Json<PrepareSolarGeometryInput>>,
) -> Result<Json<PrepareSolarGeometryOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_solar_geometry_service(input)?;

    Ok(Json(result))
}