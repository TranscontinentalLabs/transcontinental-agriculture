use axum::Json;

use crate::error::AppError;

use super::prepare_fruit_harvest_service::{
    prepare_fruit_harvest_service,
    PrepareFruitHarvestInput,
    PrepareFruitHarvestOutput,
};

pub async fn prepare_fruit_harvest_handler(
    payload: Option<Json<PrepareFruitHarvestInput>>,
) -> Result<Json<PrepareFruitHarvestOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_fruit_harvest_service(input)?;

    Ok(Json(result))
}