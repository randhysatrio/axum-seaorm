use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::APIResponse;
use crate::handler::validate_payload;
use crate::services::AuthService;
use crate::utils::encryption::hash_password;
use crate::AppState;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    success: bool,
    message: String,
}

pub async fn register_user(
    State(app_state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> APIResponse<(StatusCode, Json<RegisterResponse>)> {
    validate_payload(&body)?;

    let db = &app_state.conn;

    let RegisterRequest {
        username,
        email,
        password,
    } = body;
    let password = hash_password(password);

    AuthService::register_user(db, username, email, password).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            success: true,
            message: "Register success!".to_string(),
        }),
    ))
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    success: bool,
    message: String,
    id: i32,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> APIResponse<(StatusCode, Json<LoginResponse>)> {
    let db = &state.conn;

    let LoginRequest { email, password } = body;

    let user = AuthService::login_user(db, email, password).await?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Login Successful!".to_string(),
            id: user.id,
        }),
    ))
}
