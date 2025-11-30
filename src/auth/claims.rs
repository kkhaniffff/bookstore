use crate::{AppState, error::AppError, models::users::Role};
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: Role,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: Uuid, role: Role, ttl: i64) -> Self {
        Self {
            sub: user_id,
            role,
            exp: (Utc::now() + Duration::seconds(ttl)).timestamp() as usize,
        }
    }
}

impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
