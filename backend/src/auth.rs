use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    error::AppError,
    jwt::{create_access_token, create_refresh_token},
    models::{CreateUserRequest, LoginRequest, LoginResponse, User, UserResponse},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<ResponseJson<UserResponse>, AppError> {
    // Validate input
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::Validation("All fields are required".to_string()));
    }

    if payload.password.len() < 8 {
        return Err(AppError::Validation("Password must be at least 8 characters".to_string()));
    }

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ? OR username = ?"
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .fetch_optional(state.database.pool())
    .await?;

    if existing_user.is_some() {
        return Err(AppError::Validation("User already exists".to_string()));
    }

    // Hash password
    let password_hash = hash(payload.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

    // Create user
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&now)
    .bind(&now)
    .execute(state.database.pool())
    .await?;

    // Fetch created user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(state.database.pool())
        .await?;

    Ok(ResponseJson(user.into()))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<LoginResponse>, AppError> {
    // Find user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(state.database.pool())
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    // Verify password
    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

    if !is_valid {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    if !user.is_active {
        return Err(AppError::Authentication("Account is disabled".to_string()));
    }

    // Create tokens
    let access_token = create_access_token(&user.id, &state.config.jwt_secret, state.config.token_expiration_minutes)?;
    let refresh_token = create_refresh_token(&user.id, &state.config.jwt_secret)?;

    let response = LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.token_expiration_minutes * 60,
        user: user.into(),
    };

    Ok(ResponseJson(response))
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    use crate::jwt::verify_refresh_token;

    // Verify refresh token
    let user_id = verify_refresh_token(&payload.refresh_token, &state.config.jwt_secret)?;

    // Check if user exists and is active
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(state.database.pool())
        .await?
        .ok_or_else(|| AppError::Authentication("User not found".to_string()))?;

    if !user.is_active {
        return Err(AppError::Authentication("Account is disabled".to_string()));
    }

    // Create new access token
    let access_token = create_access_token(&user.id, &state.config.jwt_secret, state.config.token_expiration_minutes)?;

    Ok(ResponseJson(serde_json::json!({
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": state.config.token_expiration_minutes * 60
    })))
}
