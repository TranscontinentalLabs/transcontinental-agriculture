use axum::Json;

use crate::error::AppError;

use super::prepare_diffuse_solar_geometry_service::{
    prepare_diffuse_solar_geometry_service,
    PrepareDiffuseSolarGeometryInput,
    PrepareDiffuseSolarGeometryOutput,
};

pub async fn prepare_diffuse_solar_geometry_handler(
    Json(payload): Json<PrepareDiffuseSolarGeometryInput>,
) -> Result<Json<PrepareDiffuseSolarGeometryOutput>, AppError> {
    let result = tokio::task::spawn_blocking(move || {
        prepare_diffuse_solar_geometry_service(payload)
    })
        .await
        .map_err(|error| {
            AppError::Internal(format!(
                "diffuse solar calculation task failed: {error}"
            ))
        })??;

    Ok(Json(result))
}