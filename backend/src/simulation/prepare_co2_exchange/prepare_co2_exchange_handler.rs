use axum::Json;

use crate::error::AppError;

use super::prepare_co2_exchange_service::{
    prepare_co2_exchange_service,
    PrepareCo2ExchangeInput,
    PrepareCo2ExchangeOutput,
};

pub async fn prepare_co2_exchange_handler(
    payload: Option<Json<PrepareCo2ExchangeInput>>,
) -> Result<Json<PrepareCo2ExchangeOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_co2_exchange_service(input)?;

    Ok(Json(result))
}