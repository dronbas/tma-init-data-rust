use std::time::SystemTimeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseDataError {
    #[error("Invalid signature: {0}")]
    InvalidSignature(#[from] serde_json::Error),

    #[error("Invalid query string: {0}")]
    InvalidQueryString(#[from] url::ParseError),
}

#[derive(Debug, Error)]
pub enum SignError {
    #[error("Could not process signature")]
    CouldNotProcessSignature,

    #[error("Could not process auth time: {0}")]
    CouldNotProcessAuthTime(#[from] SystemTimeError),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid query string: {0}")]
    InvalidQueryString(#[from] url::ParseError),

    #[error("Unexpected format")]
    UnexpectedFormat,

    #[error("Signature is missing")]
    SignMissing,

    #[error("Auth date is missing")]
    AuthDateMissing,

    #[error("Data has expired")]
    Expired,

    #[error("Signature is invalid")]
    SignInvalid,
}
