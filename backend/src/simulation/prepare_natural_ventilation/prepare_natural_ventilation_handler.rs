use axum::Json;

use crate::error::AppError;

use super::prepare_natural_ventilation_service::{
    prepare_natural_ventilation_service,
    PrepareNaturalVentilationInput,
    PrepareNaturalVentilationOutput,
};

pub async fn prepare_natural_ventilation_handler(
    payload: Option<Json<PrepareNaturalVentilationInput>>,
) -> Result<Json<PrepareNaturalVentilationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_natural_ventilation_service(input)?;

    Ok(Json(result))
}