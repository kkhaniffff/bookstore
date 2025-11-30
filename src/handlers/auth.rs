use crate::{
    AppState,
    auth::{claims::Claims, jwt, passwords},
    error::AppError,
    payloads::auth::{LoginPayload, RegisterPayload},
    repositories::users as user_repo,
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<String>, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let user = user_repo::fetch_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| AppError::WrongCredentials)?
        .ok_or(AppError::WrongCredentials)?;

    let valid = passwords::verify(&payload.password, &user.password)?;

    if !valid {
        return Err(AppError::WrongCredentials);
    }

    let claims = Claims::new(user.id, user.role, state.jwt_ttl);
    let token = jwt::create(&claims, &state.jwt_secret)?;

    Ok(Json(token))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<uuid::Uuid>), AppError> {
    let hashed = passwords::hash(&payload.password)
        .map_err(|_| AppError::BadRequest("could not hash password".to_string()))?;

    let payload = RegisterPayload {
        password: hashed,
        ..payload
    };

    let id = user_repo::insert(&state.db, &payload)
        .await
        .map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(id)))
}
