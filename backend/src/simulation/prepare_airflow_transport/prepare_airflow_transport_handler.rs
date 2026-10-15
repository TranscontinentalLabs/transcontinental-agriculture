use axum::Json;

use crate::error::AppError;

use super::prepare_airflow_transport_service::{
    prepare_airflow_transport_service,
    PrepareAirflowTransportInput,
    PrepareAirflowTransportOutput,
};

pub async fn prepare_airflow_transport_handler(
    payload: Option<Json<PrepareAirflowTransportInput>>,
) -> Result<Json<PrepareAirflowTransportOutput>, AppError> {
    let input = payload.map(|Json(body)| body).unwrap_or_default();
    let result = prepare_airflow_transport_service(input)?;

    Ok(Json(result))
}