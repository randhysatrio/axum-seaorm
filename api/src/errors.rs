use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DBError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Username is alread taken")]
    DuplicateUsername,
    #[error("Email is already registered")]
    DuplicateEmail,
    #[error("Please check your email or password")]
    WrongCredentials,
    #[error("Invalid token!")]
    InvalidToken,
}

// 1. APIResult is an Option enum for handling all internal API process that could potentially fail;
// 2. It will transform itself into an APIError should the caller code had an APIResponse return type
// if the APIResult returns an Err variant (Using the From trait);
pub type APIResult<T> = std::result::Result<T, Error>;
pub type APIError = (StatusCode, Json<Value>);
pub type APIResponse<T> = std::result::Result<T, APIError>;

impl From<Error> for APIError {
    fn from(err: Error) -> Self {
        let status_code = match err {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::DuplicateUsername => StatusCode::CONFLICT,
            Error::DuplicateEmail => StatusCode::CONFLICT,
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // err.to_string() will consumed the data return from thiserror macro;
        let payload = json!({ "success": false, "message": err.to_string() });

        (status_code, Json(payload))
    }
}
