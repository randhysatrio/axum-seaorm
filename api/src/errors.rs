use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Crate Error
    #[error(transparent)]
    DBError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
    // Auth Error
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
    // Category Error
    #[error("Category already created")]
    DuplicateCategory,
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Category already deleted")]
    CategoryAlreadyDeleted,
}

// 1. APIResult is an Option enum for handling all internal API process that could potentially fail;
// 2. Anything that returns an APIResult can be propagated by using ? at the end of expression in the caller code;
// 3. We must transform APIResult Error variant into an APIError type should the caller code had an APIResponse return type (Using the From trait defined below);
// 4. To transform it, if its an external crate error we need to use the .map_err fn then mapped that crate error from the closure argument into Error::ErrorKind(e: ExternalCrateErrorType) or just return Err(Error::SomeError) if it was this crate error;
pub type APIResult<T> = std::result::Result<T, AppError>;
pub type APIError = (StatusCode, Json<Value>);
pub type APIResponse<T> = std::result::Result<T, APIError>;

impl From<AppError> for APIError {
    fn from(err: AppError) -> Self {
        let status_code = match err {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DuplicateUsername => StatusCode::CONFLICT,
            AppError::DuplicateEmail => StatusCode::CONFLICT,
            AppError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            // Category Error
            AppError::DuplicateCategory => StatusCode::CONFLICT,
            AppError::CategoryNotFound => StatusCode::BAD_REQUEST,
            AppError::CategoryAlreadyDeleted => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // err.to_string() will consumed the message defined in #[error(err_message_here)] macro;
        let payload = json!({ "success": false, "message": err.to_string() });

        (status_code, Json(payload))
    }
}
