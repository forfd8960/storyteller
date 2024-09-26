use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),
}
