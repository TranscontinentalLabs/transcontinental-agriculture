use axum::Json;

use crate::error::AppError;

use super::prepare_carbohydrate_allocation_service::{
    prepare_carbohydrate_allocation_service,
    PrepareCarbohydrateAllocationInput,
    PrepareCarbohydrateAllocationOutput,
};

pub async fn prepare_carbohydrate_allocation_handler(
    payload: Option<Json<PrepareCarbohydrateAllocationInput>>,
) -> Result<Json<PrepareCarbohydrateAllocationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_carbohydrate_allocation_service(input)?;

    Ok(Json(result))
}