use reqwest::Client;
use sqlx::PgPool;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState{
    pub app_config: AppConfig,
    pub pool: PgPool,
    pub http_client: Client,
}