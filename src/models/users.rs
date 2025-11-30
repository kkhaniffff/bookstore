use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,

    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}
