use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to send event: {0}")]
    SendError(String),
    #[error("Event stream closed unexpectedly")]
    EventStreamClosed,

    #[error("Failed to draw: {0}")]
    DrawError(String),
}

pub type AppResult<T> = Result<T, AppError>;
