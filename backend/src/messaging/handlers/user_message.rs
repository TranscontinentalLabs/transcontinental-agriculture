use axum::extract::State;
use axum::Json;
use crate::error::AppError;
use crate::messaging::services::user_message::{user_message_service, UserMessageInput};
use crate::state::AppState;

pub async fn user_message_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<UserMessageInput>,
) -> Result<Json<serde_json::Value>, AppError> {

    let result = user_message_service(
        payload,
        app_state.clone(),
    ).await?;
    Ok(result)
}
