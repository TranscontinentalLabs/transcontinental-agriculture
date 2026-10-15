use axum::Json;

use crate::error::AppError;

use super::get_greenhouse_configuration_service::{
    get_greenhouse_configuration,
    GreenhouseConfigurationInput,
    GreenhouseConfigurationOutput,
};

pub async fn get_greenhouse_configuration_handler(
    Json(payload): Json<GreenhouseConfigurationInput>,
) -> Result<Json<GreenhouseConfigurationOutput>, AppError> {
    let result = get_greenhouse_configuration(payload)?;

    Ok(Json(result))
}