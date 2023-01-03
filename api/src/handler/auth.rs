use axum::{
    extract::State,
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::APIResponse;
use crate::handler::validate_payload;
use crate::services::AuthService;
use crate::utils::{
    encryption::hash_password,
    jwt::{generate_token, verify_token},
};
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
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> APIResponse<(StatusCode, Json<RegisterResponse>)> {
    validate_payload(&body)?;

    let db = &state.conn;
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
    token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> APIResponse<(StatusCode, Json<LoginResponse>)> {
    let db = &state.conn;
    let LoginRequest { email, password } = body;

    let user = AuthService::login_user(db, email, password).await?;
    let token = generate_token(user.id)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Login Successful!".to_string(),
            token,
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct PersistentLoginData {
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
pub struct PersistentLoginResponse {
    success: bool,
    token: String,
    data: PersistentLoginData,
}

pub async fn persistent_login(
    State(state): State<AppState>,
    TypedHeader(user_token): TypedHeader<Authorization<Bearer>>,
) -> APIResponse<(StatusCode, Json<PersistentLoginResponse>)> {
    let db = &state.conn;

    let verified_token = verify_token(user_token.token())?;
    let user_id = verified_token.user_id;

    let user = AuthService::find_user_by_id(db, user_id).await?;
    let token = generate_token(user.id)?;

    let data = PersistentLoginData {
        username: user.username,
        email: user.email,
    };

    Ok((
        StatusCode::OK,
        Json(PersistentLoginResponse {
            success: true,
            token,
            data,
        }),
    ))
}
