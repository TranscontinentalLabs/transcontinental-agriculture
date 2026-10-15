use axum::Json;

use crate::error::AppError;

use super::prepare_condensation_service::{
    prepare_condensation_service,
    PrepareCondensationInput,
    PrepareCondensationOutput,
};

pub async fn prepare_condensation_handler(
    payload: Option<Json<PrepareCondensationInput>>,
) -> Result<Json<PrepareCondensationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_condensation_service(input)?;

    Ok(Json(result))
}