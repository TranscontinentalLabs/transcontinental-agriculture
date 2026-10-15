use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct InterpretedUserMessage {
    pub text: String,
}

pub async fn interpret_user_message(
    app_state: AppState,
    user_message: String,
    session_id: String,
) -> Result<InterpretedUserMessage, AppError> {
    let url = "http://localhost:5678/webhook/interpret-user-message";

    let response = app_state
        .http_client
        .post(url)
        .json(&json!({
            "message": user_message,
            "session_id": session_id
        }))
        .send()
        .await
        .map_err(AppError::from)?;

    let interpreted_result = response
        .json::<InterpretedUserMessage>()
        .await
        .map_err(AppError::from)?;

    Ok(interpreted_result)
}