use axum::Json;

use crate::error::AppError;

use super::prepare_screen_air_exchange_service::{
    prepare_screen_air_exchange_service,
    PrepareScreenAirExchangeInput,
    PrepareScreenAirExchangeOutput,
};

pub async fn prepare_screen_air_exchange_handler(
    payload: Option<Json<PrepareScreenAirExchangeInput>>,
) -> Result<Json<PrepareScreenAirExchangeOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_screen_air_exchange_service(input)?;

    Ok(Json(result))
}