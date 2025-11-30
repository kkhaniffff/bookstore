use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
    pub name: String,
}
