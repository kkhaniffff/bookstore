use crate::{
    AppState,
    auth::{claims::Claims, passwords},
    error::AppError,
    models::users::User,
    payloads::users::ChangePasswordPayload,
    repositories::users as user_repo,
};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn current_user(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>, AppError> {
    let user = user_repo::fetch_by_id(&state.db, &claims.sub)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::NotFound(claims.sub.to_string()))?;

    Ok(Json(user))
}

pub async fn change_password(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChangePasswordPayload>,
) -> Result<(), AppError> {
    let user = user_repo::fetch_by_id(&state.db, &claims.sub)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::NotFound(claims.sub.to_string()))?;

    let valid = passwords::verify(&payload.old_password, &user.password)?;

    if !valid {
        return Err(AppError::WrongCredentials);
    }

    let hashed = passwords::hash(&payload.new_password)
        .map_err(|_| AppError::BadRequest("could not hash password".to_string()))?;

    let rows_affected = user_repo::change_password(&state.db, &user.id, &hashed)
        .await
        .map_err(AppError::from)?;

    match rows_affected {
        0 => Err(AppError::NotFound(user.id.to_string())),
        _ => Ok(()),
    }
}
