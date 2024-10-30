use lapin::Error as LapinError;
use serde_json::Error as SerdeError;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] LapinError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),
}
