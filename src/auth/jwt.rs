use crate::{auth::claims::Claims, error::AppError};
use jsonwebtoken::{EncodingKey, Header, encode};

pub fn create(claims: &Claims, secret: &str) -> Result<String, AppError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::TokenCreation)
}
