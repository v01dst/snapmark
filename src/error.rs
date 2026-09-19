use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("storage error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid data: {0}")]
    Json(#[from] serde_json::Error),
    #[error("entry not found: {0}")]
    NotFound(String),
    #[error("entry already exists: {0}")]
    AlreadyExists(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("server error: {0}")]
    Server(String),
}
