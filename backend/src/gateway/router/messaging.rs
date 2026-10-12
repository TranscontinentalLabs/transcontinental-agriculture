use axum::Router;
use axum::routing::post;
use crate::messaging::handlers::user_message::user_message_handler;
use crate::state::AppState;

pub fn messaging_routes() -> Router<AppState>{
    Router::new()
        .route("/user-message",post(user_message_handler))
}