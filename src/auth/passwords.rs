use crate::error::AppError;

pub fn hash(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| AppError::WrongCredentials)
}

pub fn verify(password: &str, hashed: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hashed).map_err(|_| AppError::WrongCredentials)
}
