use axum::Json;
use crate::error::AppError;
use crate::tools::vdp::vdp_service::{calculate_vdp_service, VDPInput, VDPOutput};

pub async fn calculate_vdp_handler(
    Json(payload): Json<VDPInput>,
) -> Result<Json<VDPOutput>, AppError> {
    let output = calculate_vdp_service(payload)?;
    Ok(Json(output))
}