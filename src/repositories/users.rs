use crate::{
    models::users::{Role, User},
    payloads::auth::RegisterPayload,
};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn insert(pool: &PgPool, payload: &RegisterPayload) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO users (id, email, name, password, role)
            VALUES ($1, $2, $3, $4, 'user')
        "#,
        id,
        payload.email,
        payload.name,
        payload.password,
    )
    .execute(pool)
    .await
    .map(|_| id)
}

pub async fn fetch_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT 
                    id,
                    email,
                    name,
                    password,
                    role as "role: Role"
            FROM users
            WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>, Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT 
                    id,
                    email,
                    name,
                    password,
                    role as "role: Role"
            FROM users
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn change_password(pool: &PgPool, id: &Uuid, password: &str) -> Result<u64, Error> {
    sqlx::query!("UPDATE users SET password = $1 WHERE id = $2", password, id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}
