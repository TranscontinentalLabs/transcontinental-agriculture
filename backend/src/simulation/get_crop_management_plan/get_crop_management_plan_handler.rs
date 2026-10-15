use axum::Json;

use crate::error::AppError;

use super::get_crop_management_plan_service::{
    get_crop_management_plan_service,
    GetCropManagementPlanInput,
    GetCropManagementPlanOutput,
};

pub async fn get_crop_management_plan_handler(
    payload: Option<Json<GetCropManagementPlanInput>>,
) -> Result<Json<GetCropManagementPlanOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = get_crop_management_plan_service(input)?;

    Ok(Json(result))
}