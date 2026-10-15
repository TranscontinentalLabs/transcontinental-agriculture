use axum::Json;

use crate::error::AppError;

use super::prepare_solar_geometry_service::{
    prepare_solar_geometry_service,
    PrepareSolarGeometryInput,
    PrepareSolarGeometryOutput,
};

pub async fn prepare_solar_geometry_handler(
    Json(payload): Json<PrepareSolarGeometryInput>,
) -> Result<Json<PrepareSolarGeometryOutput>, AppError> {
    let result = prepare_solar_geometry_service(payload)?;

    Ok(Json(result))
}