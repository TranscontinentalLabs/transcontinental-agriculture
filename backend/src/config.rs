use std::env;
use std::net::SocketAddr;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppConfig{
    pub server_address:SocketAddr,
    pub database_url:String
}

impl AppConfig{
    pub fn load_config() -> Result<Self, AppError> {
        let server_address = env::var("SERVER_ADDRESS")?
            .parse::<SocketAddr>()?;

        let database_url = env::var("DATABASE_URL")?;

        Ok(Self{
            server_address,
            database_url,
        })
    }
}