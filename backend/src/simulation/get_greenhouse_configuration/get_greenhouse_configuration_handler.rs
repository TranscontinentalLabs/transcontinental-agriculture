use axum::Json;

use crate::error::AppError;

use super::get_greenhouse_configuration_service::{
    get_greenhouse_configuration_service,
    GreenhouseConfigurationInput,
    GreenhouseConfigurationOutput,
};

pub async fn get_greenhouse_configuration_handler(
    payload: Option<Json<GreenhouseConfigurationInput>>,
) -> Result<Json<GreenhouseConfigurationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = get_greenhouse_configuration_service(input)?;

    Ok(Json(result))
}