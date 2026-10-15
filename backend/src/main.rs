use crate::config::AppConfig;
use crate::connection::create_pool;
use crate::error::AppError;
use crate::gateway::router::main::main_router;
use crate::state::AppState;
use reqwest::Client;

pub mod gateway;
pub mod config;
pub mod error;
pub mod connection;
pub mod state;
pub mod tools;
pub mod messaging;
pub mod simulation;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();

    let app_config = AppConfig::load_config()?;

    let http_client = Client::new();

    let pool = create_pool(&app_config.database_url).await?;
    println!("Connected to database");

    let app_state = AppState{
        app_config: app_config.clone(),
        pool,
        http_client,
    };

    let app = main_router()
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(app_config.server_address).await?;

    println!("Server running on port: http://{}", app_config.server_address);

    axum::serve(listener, app).await?;

    Ok(())
}
