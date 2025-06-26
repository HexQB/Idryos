use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub token_expiration_minutes: u64,
    pub database_url: String,
    pub cors_origins: Vec<String>,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        Ok(Config {
            port: env::var("AUTH_SERVICE_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "ChangeMeSuperSecretKey".to_string()),
            token_expiration_minutes: env::var("TOKEN_EXPIRATION_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://./data/idryos.db".to_string()),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }
}
