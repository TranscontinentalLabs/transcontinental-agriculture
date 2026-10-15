use axum::Json;

use crate::error::AppError;

use super::prepare_weather_service::{
    prepare_weather_service,
    PrepareWeatherInput,
    PrepareWeatherOutput,
};

pub async fn prepare_weather_handler(
    payload: Option<Json<PrepareWeatherInput>>,
) -> Result<Json<PrepareWeatherOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_weather_service(input)?;

    Ok(Json(result))
}