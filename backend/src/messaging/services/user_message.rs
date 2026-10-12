use axum::Json;
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::state::AppState;
use crate::tools::interpret_user_message::interpret_user_message::interpret_user_message;

#[derive(Deserialize, Serialize)]
pub struct UserMessageInput {
    pub user_message: String,
}

pub async fn user_message_service(
    payload: UserMessageInput,
    app_state: AppState,
) -> Result<Json<serde_json::Value>, AppError> {
    let message = payload.user_message;
    let response = interpret_user_message(app_state, message).await?;

    Ok(Json(json!({
        "message": format!("Your response is {}", response.text)
    })))
}



