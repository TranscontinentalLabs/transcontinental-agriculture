use axum::Json;

use crate::error::AppError;

use super::prepare_diffuse_solar_geometry_service::{
    prepare_diffuse_solar_geometry_service,
    PrepareDiffuseSolarGeometryInput,
    PrepareDiffuseSolarGeometryOutput,
};

pub async fn prepare_diffuse_solar_geometry_handler(
    payload: Option<Json<PrepareDiffuseSolarGeometryInput>>,
) -> Result<Json<PrepareDiffuseSolarGeometryOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_diffuse_solar_geometry_service(input)?;

    Ok(Json(result))
}