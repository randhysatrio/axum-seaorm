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
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Username is alread taken")]
    DuplicateUsername,
    #[error("Email is already registered")]
    DuplicateEmail,
    #[error("Please check your email or password")]
    WrongCredentials,
}

pub type APIResult<T> = std::result::Result<T, Error>;
pub type APIError = (StatusCode, Json<Value>);
pub type APIResponse<T> = std::result::Result<T, APIError>;

impl From<Error> for APIError {
    fn from(err_msg: Error) -> Self {
        let status_code = match err_msg {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::DuplicateUsername => StatusCode::CONFLICT,
            Error::DuplicateEmail => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let payload = json!({ "success": false, "message": err_msg.to_string() });

        (status_code, Json(payload))
    }
}
