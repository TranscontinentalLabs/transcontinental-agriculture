use axum::Json;
use serde_json::{json, Value};

pub async fn start_simulation() -> Json<Value>{
    Json(json!({
        "message": "The simulation finished successfully"
    }))
}