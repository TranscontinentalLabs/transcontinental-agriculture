use axum::Json;

use crate::error::AppError;

use super::prepare_heating_pipes_service::{
    prepare_heating_pipes_service,
    PrepareHeatingPipesInput,
    PrepareHeatingPipesOutput,
};

pub async fn prepare_heating_pipes_handler(
    payload: Option<Json<PrepareHeatingPipesInput>>,
) -> Result<Json<PrepareHeatingPipesOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_heating_pipes_service(input)?;

    Ok(Json(result))
}