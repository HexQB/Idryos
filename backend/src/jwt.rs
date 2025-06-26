use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user ID)
    exp: usize,  // Expiration time
    iat: usize,  // Issued at
    token_type: String, // "access" or "refresh"
}

pub fn create_access_token(user_id: &str, secret: &str, expiration_minutes: u64) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::minutes(expiration_minutes as i64);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn create_refresh_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::days(30); // Refresh tokens last 30 days

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<String, AppError> {
    let mut validation = Validation::default();
    validation.algorithms = vec![Algorithm::HS256];

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    if token_data.claims.token_type != "access" {
        return Err(AppError::Authentication("Invalid token type".to_string()));
    }

    Ok(token_data.claims.sub)
}

pub fn verify_refresh_token(token: &str, secret: &str) -> Result<String, AppError> {
    let mut validation = Validation::default();
    validation.algorithms = vec![Algorithm::HS256];

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    if token_data.claims.token_type != "refresh" {
        return Err(AppError::Authentication("Invalid token type".to_string()));
    }

    Ok(token_data.claims.sub)
}
