use axum::{
    extract::{Json, Query, State},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;

use crate::{
    error::AppError,
    jwt::create_access_token,
    models::{AuthorizeRequest, TokenRequest, TokenResponse, OAuthClient, User},
    AppState,
};

pub async fn authorize(
    State(state): State<AppState>,
    Query(params): Query<AuthorizeRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // Validate client
    let client = sqlx::query_as::<_, OAuthClient>("SELECT * FROM oauth_clients WHERE id = ?")
        .bind(&params.client_id)
        .fetch_optional(state.database.pool())
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid client".to_string()))?;

    if !client.is_active {
        return Err(AppError::Authentication("Client is disabled".to_string()));
    }

    // Validate redirect URI
    let redirect_uris: Vec<String> = serde_json::from_str(&client.redirect_uris)
        .map_err(|_| AppError::Internal("Invalid redirect URIs format".to_string()))?;
    
    if !redirect_uris.contains(&params.redirect_uri) {
        return Err(AppError::Authentication("Invalid redirect URI".to_string()));
    }

    // For demo purposes, we'll return an authorization URL
    // In a real implementation, this would redirect to a login/consent page
    Ok(ResponseJson(serde_json::json!({
        "authorize_url": format!(
            "{}?response_type={}&client_id={}&redirect_uri={}&scope={}&state={}",
            "/oauth/login",
            params.response_type,
            params.client_id,
            params.redirect_uri,
            params.scope.unwrap_or_default(),
            params.state.unwrap_or_default()
        )
    })))
}

pub async fn token(
    State(state): State<AppState>,
    Json(payload): Json<TokenRequest>,
) -> Result<ResponseJson<TokenResponse>, AppError> {
    // Validate client
    let client = sqlx::query_as::<_, OAuthClient>("SELECT * FROM oauth_clients WHERE id = ?")
        .bind(&payload.client_id)
        .fetch_optional(state.database.pool())
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid client".to_string()))?;

    if !client.is_active {
        return Err(AppError::Authentication("Client is disabled".to_string()));
    }

    match payload.grant_type.as_str() {
        "authorization_code" => handle_authorization_code_grant(state, client, payload).await,
        "refresh_token" => handle_refresh_token_grant(state, client, payload).await,
        _ => Err(AppError::Authentication("Unsupported grant type".to_string())),
    }
}

async fn handle_authorization_code_grant(
    state: AppState,
    client: OAuthClient,
    payload: TokenRequest,
) -> Result<ResponseJson<TokenResponse>, AppError> {
    let code = payload.code.ok_or_else(|| AppError::Authentication("Authorization code required".to_string()))?;
    
    // Verify authorization code
    let auth_code = sqlx::query!(
        "SELECT user_id, expires_at FROM oauth_authorization_codes WHERE code = ? AND client_id = ?",
        code,
        client.id
    )
    .fetch_optional(state.database.pool())
    .await?
    .ok_or_else(|| AppError::Authentication("Invalid authorization code".to_string()))?;

    // Check if code is expired
    if auth_code.expires_at < Utc::now() {
        return Err(AppError::Authentication("Authorization code expired".to_string()));
    }

    // Delete used authorization code
    sqlx::query!("DELETE FROM oauth_authorization_codes WHERE code = ?", code)
        .execute(state.database.pool())
        .await?;

    // Create tokens
    let access_token = create_access_token(&auth_code.user_id, &state.config.jwt_secret, state.config.token_expiration_minutes)?;
    let refresh_token = generate_refresh_token();

    // Store refresh token
    let expires_at = Utc::now() + Duration::days(30);
    sqlx::query!(
        r#"
        INSERT INTO oauth_refresh_tokens (token, client_id, user_id, expires_at)
        VALUES (?, ?, ?, ?)
        "#,
        refresh_token,
        client.id,
        auth_code.user_id,
        expires_at
    )
    .execute(state.database.pool())
    .await?;

    Ok(ResponseJson(TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.token_expiration_minutes * 60,
        refresh_token: Some(refresh_token),
        scope: None,
    }))
}

async fn handle_refresh_token_grant(
    state: AppState,
    client: OAuthClient,
    payload: TokenRequest,
) -> Result<ResponseJson<TokenResponse>, AppError> {
    let refresh_token = payload.refresh_token.ok_or_else(|| AppError::Authentication("Refresh token required".to_string()))?;
    
    // Verify refresh token
    let token_record = sqlx::query!(
        "SELECT user_id, expires_at FROM oauth_refresh_tokens WHERE token = ? AND client_id = ?",
        refresh_token,
        client.id
    )
    .fetch_optional(state.database.pool())
    .await?
    .ok_or_else(|| AppError::Authentication("Invalid refresh token".to_string()))?;

    // Check if token is expired
    if token_record.expires_at < Utc::now() {
        return Err(AppError::Authentication("Refresh token expired".to_string()));
    }

    // Create new access token
    let access_token = create_access_token(&token_record.user_id, &state.config.jwt_secret, state.config.token_expiration_minutes)?;

    Ok(ResponseJson(TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.token_expiration_minutes * 60,
        refresh_token: None,
        scope: None,
    }))
}

pub async fn userinfo(
    State(state): State<AppState>,
    // Extract token from Authorization header
    // This is a simplified version - in practice you'd use middleware
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // For demo purposes, return a mock user
    Ok(ResponseJson(serde_json::json!({
        "sub": "user123",
        "name": "Demo User",
        "email": "demo@idryos.local",
        "preferred_username": "demo"
    })))
}

pub async fn openid_configuration(
    State(state): State<AppState>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let base_url = "http://localhost:8000"; // In practice, use configured base URL

    Ok(ResponseJson(serde_json::json!({
        "issuer": base_url,
        "authorization_endpoint": format!("{}/oauth/authorize", base_url),
        "token_endpoint": format!("{}/oauth/token", base_url),
        "userinfo_endpoint": format!("{}/oauth/userinfo", base_url),
        "jwks_uri": format!("{}/.well-known/jwks.json", base_url),
        "response_types_supported": ["code"],
        "subject_types_supported": ["public"],
        "id_token_signing_alg_values_supported": ["HS256"]
    })))
}

fn generate_refresh_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    URL_SAFE_NO_PAD.encode(&bytes)
}
