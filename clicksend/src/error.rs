use std::error::Error;
use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InvalidSender(String),
    InvalidPhoneNumber(String),
    MessageSendFailed(String),
    ClickSendApiError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidPhoneNumber(number) => write!(f, "Invalid Phone number: {}", number),
            AppError::InvalidSender(sender) => write!(f, "Sender ID must be either a registered alpha tag, a verified own number, or a purchased dedicated number: {}", sender),
            AppError::MessageSendFailed(err) => write!(f, "Failed to send message: {}", err),
            AppError::ClickSendApiError(err) => write!(f, "ClickSend API Error: {}", err)
        }
    }
}

impl Error for AppError {}
