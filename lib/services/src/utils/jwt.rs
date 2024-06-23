use crate::features::auth::types::Claims;
use anyhow::Result;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use uuid::Uuid;

static SECRET: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    std::env::var("JWT_SECRET").unwrap_or("my-secret".into())
});

pub fn encode_jwt(user_id: Uuid, user_email: String) -> Result<String> {
    let claims = Claims {
        sub: user_email,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        id: user_id,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: String) -> Result<Claims> {
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?
    .claims;

    Ok(claims)
}
