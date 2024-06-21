use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::features::auth::types::Claims;
use anyhow::Result;

pub fn encode_jwt(user_id: Uuid, user_email: String, secret: String) -> Result<String> {
    let claims = Claims {
        sub: user_email,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        id: user_id,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: String, secret: String) -> Result<Uuid> {
    let Claims {
        id: user_id,
        sub: _,
        exp: _,
    } = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?
    .claims;

    Ok(user_id)
}
