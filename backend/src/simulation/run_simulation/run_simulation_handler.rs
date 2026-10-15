use axum::Json;

use crate::error::AppError;

use super::run_simulation_service::{
    run_simulation_service,
    RunSimulationInput,
    RunSimulationOutput,
};

pub async fn run_simulation_handler(
    payload: Option<Json<RunSimulationInput>>,
) -> Result<Json<RunSimulationOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = run_simulation_service(input)?;

    Ok(Json(result))
}