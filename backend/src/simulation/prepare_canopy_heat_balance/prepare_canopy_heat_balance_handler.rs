use axum::Json;

use crate::error::AppError;

use super::prepare_canopy_heat_balance_service::{
    prepare_canopy_heat_balance_service,
    PrepareCanopyHeatBalanceInput,
    PrepareCanopyHeatBalanceOutput,
};

pub async fn prepare_canopy_heat_balance_handler(
    payload: Option<Json<PrepareCanopyHeatBalanceInput>>,
) -> Result<Json<PrepareCanopyHeatBalanceOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_canopy_heat_balance_service(input)?;

    Ok(Json(result))
}