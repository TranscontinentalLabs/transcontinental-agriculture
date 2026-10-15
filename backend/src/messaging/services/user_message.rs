use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;
use crate::state::AppState;
use crate::tools::interpret_user_message::interpret_user_message::interpret_user_message;

#[derive(Deserialize, Serialize)]
pub struct UserMessageInput {
    pub user_message: String,
    pub session_id: Option<String>,
}

pub async fn user_message_service(
    payload: UserMessageInput,
    app_state: AppState,
) -> Result<Json<serde_json::Value>, AppError> {
    let session_id = payload
        .session_id
        .unwrap_or_else(|| "mark-test-001".to_string());
    let response = interpret_user_message(app_state, payload.user_message, session_id.clone()).await?;

    Ok(Json(json!({
        "session_id": session_id,
        "message": response.text
    })))
}