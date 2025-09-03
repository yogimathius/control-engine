use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::models::{AuthToken, Practitioner, PractitionerProfile};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // practitioner_id
    pub email: String,
    pub spiritual_name: Option<String>,
    pub exp: usize, // expiration time
    pub iat: usize, // issued at
}

const JWT_SECRET: &[u8] = b"sacred_codex_jwt_secret_key_change_in_production";

pub fn create_jwt_token(
    practitioner: &Practitioner,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: practitioner.id.to_string(),
        email: practitioner.email.clone(),
        spiritual_name: practitioner.spiritual_name.clone(),
        exp: now + 24 * 60 * 60, // 24 hours
        iat: now,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn verify_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET), &validation)
        .map(|data| data.claims)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub async fn auth_middleware(
    State(app_state): State<crate::handlers::AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    let claims = verify_jwt_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Get practitioner from database to ensure they still exist
    let practitioner_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let practitioner =
        sqlx::query_as::<_, Practitioner>("SELECT * FROM practitioners WHERE id = $1")
            .bind(practitioner_id)
            .fetch_one(&app_state.db)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add practitioner to request extensions for handlers to access
    request.extensions_mut().insert(practitioner);

    Ok(next.run(request).await)
}

pub fn create_auth_response(
    practitioner: &Practitioner,
) -> Result<AuthToken, jsonwebtoken::errors::Error> {
    let token = create_jwt_token(practitioner)?;

    let profile = PractitionerProfile {
        id: practitioner.id,
        email: practitioner.email.clone(),
        spiritual_name: practitioner.spiritual_name.clone(),
        archetypal_preferences: practitioner.archetypal_preferences.clone(),
        energy_alignments: practitioner.energy_alignments.clone(),
        privacy_level: practitioner.privacy_level.clone(),
        sacred_path: practitioner.sacred_path.clone(),
        member_since: practitioner.created_at,
    };

    Ok(AuthToken {
        token,
        practitioner: profile,
    })
}
