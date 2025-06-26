use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod auth;
mod config;
mod database;
mod did;
mod error;
mod jwt;
mod models;
mod oauth;

use config::Config;
use database::Database;
use error::AppError;

type AppState = Arc<AppContext>;

#[derive(Clone)]
pub struct AppContext {
    pub config: Config,
    pub database: Database,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting Idryos Auth Service on port {}", config.port);

    // Initialize database
    let database = Database::new(&config.database_url).await?;
    database.migrate().await?;

    // Create application state
    let state = Arc::new(AppContext { config, database });

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/oauth/authorize", get(oauth::authorize))
        .route("/oauth/token", post(oauth::token))
        .route("/oauth/userinfo", get(oauth::userinfo))
        .route("/.well-known/openid_configuration", get(oauth::openid_configuration))
        .route("/did/create", post(did::create_did))
        .route("/did/resolve/:did", get(did::resolve_did))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", 8000)).await?;
    info!("Auth service listening on {}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "idryos-auth",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
