use axum::Json;

use crate::error::AppError;

use super::prepare_weather_service::{
    prepare_weather,
    PrepareWeatherInput,
    PrepareWeatherOutput,
};

pub async fn prepare_weather_handler(
    Json(payload): Json<PrepareWeatherInput>,
) -> Result<Json<PrepareWeatherOutput>, AppError> {
    let result = prepare_weather(payload)?;

    Ok(Json(result))
}