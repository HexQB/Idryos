use axum::{
    extract::{Json, Path, State},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::{error::AppError, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDidRequest {
    pub method: String, // "key", "web", etc.
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: String,
    pub context: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_base58: Option<String>,
    pub public_key_jwk: Option<serde_json::Value>,
}

pub async fn create_did(
    State(state): State<AppState>,
    Json(payload): Json<CreateDidRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // Validate user exists
    let user = sqlx::query!("SELECT id FROM users WHERE id = ?", payload.user_id)
        .fetch_optional(state.database.pool())
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let did = match payload.method.as_str() {
        "key" => create_did_key().await?,
        "web" => create_did_web(&payload.user_id).await?,
        _ => return Err(AppError::Validation("Unsupported DID method".to_string())),
    };

    // Update user with DID
    sqlx::query!(
        "UPDATE users SET did = ?, updated_at = ? WHERE id = ?",
        did.id,
        Utc::now(),
        payload.user_id
    )
    .execute(state.database.pool())
    .await?;

    Ok(ResponseJson(serde_json::json!({
        "did": did.id,
        "document": did
    })))
}

pub async fn resolve_did(
    State(_state): State<AppState>,
    Path(did): Path<String>,
) -> Result<ResponseJson<DidDocument>, AppError> {
    // This is a simplified implementation
    // In practice, you'd resolve the DID from the appropriate registry
    if did.starts_with("did:key:") {
        resolve_did_key(&did).await
    } else if did.starts_with("did:web:") {
        resolve_did_web(&did).await
    } else {
        Err(AppError::NotFound("DID not found or unsupported method".to_string()))
    }
}

async fn create_did_key() -> Result<DidDocument, AppError> {
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

    let rng = SystemRandom::new();
    let key_pair = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| AppError::Internal(format!("Key generation failed: {:?}", e)))?;
    
    let public_key = key_pair.public_key();
    let public_key_b58 = URL_SAFE_NO_PAD.encode(public_key.as_ref());
    
    let did_id = format!("did:key:z{}", public_key_b58);
    let vm_id = format!("{}#key-1", did_id);

    Ok(DidDocument {
        id: did_id.clone(),
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
        ],
        verification_method: vec![VerificationMethod {
            id: vm_id.clone(),
            r#type: "Ed25519VerificationKey2020".to_string(),
            controller: did_id.clone(),
            public_key_base58: Some(public_key_b58),
            public_key_jwk: None,
        }],
        authentication: vec![vm_id],
        created: Utc::now().to_rfc3339(),
        updated: Utc::now().to_rfc3339(),
    })
}

async fn create_did_web(user_id: &str) -> Result<DidDocument, AppError> {
    let domain = "localhost:8000"; // In practice, use configured domain
    let did_id = format!("did:web:{}:users:{}", domain, user_id);
    let vm_id = format!("{}#key-1", did_id);

    // Generate a key pair for the DID:web method
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

    let rng = SystemRandom::new();
    let key_pair = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| AppError::Internal(format!("Key generation failed: {:?}", e)))?;
    
    let public_key = key_pair.public_key();
    let public_key_b58 = URL_SAFE_NO_PAD.encode(public_key.as_ref());

    Ok(DidDocument {
        id: did_id.clone(),
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
        ],
        verification_method: vec![VerificationMethod {
            id: vm_id.clone(),
            r#type: "Ed25519VerificationKey2020".to_string(),
            controller: did_id.clone(),
            public_key_base58: Some(public_key_b58),
            public_key_jwk: None,
        }],
        authentication: vec![vm_id],
        created: Utc::now().to_rfc3339(),
        updated: Utc::now().to_rfc3339(),
    })
}

async fn resolve_did_key(did: &str) -> Result<ResponseJson<DidDocument>, AppError> {
    // Extract public key from DID
    if !did.starts_with("did:key:z") {
        return Err(AppError::NotFound("Invalid DID:key format".to_string()));
    }

    let public_key_b58 = &did[9..]; // Remove "did:key:z" prefix
    let vm_id = format!("{}#key-1", did);

    let doc = DidDocument {
        id: did.to_string(),
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
        ],
        verification_method: vec![VerificationMethod {
            id: vm_id.clone(),
            r#type: "Ed25519VerificationKey2020".to_string(),
            controller: did.to_string(),
            public_key_base58: Some(public_key_b58.to_string()),
            public_key_jwk: None,
        }],
        authentication: vec![vm_id],
        created: Utc::now().to_rfc3339(),
        updated: Utc::now().to_rfc3339(),
    };

    Ok(ResponseJson(doc))
}

async fn resolve_did_web(did: &str) -> Result<ResponseJson<DidDocument>, AppError> {
    // This is a simplified implementation
    // In practice, you'd fetch the DID document from the web location
    Err(AppError::NotFound("DID:web resolution not fully implemented".to_string()))
}
