use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{database::mutation::AuthMutation, AppState};

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
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
) -> impl IntoResponse {
    let db = &app_state.conn;

    let RegisterRequest {
        username,
        email,
        password,
    } = body;
    let password = hash_password(password).unwrap();

    AuthMutation::register_user(db, username, email, password)
        .await
        .map(|_| {
            (
                StatusCode::CREATED,
                Json(RegisterResponse {
                    success: true,
                    message: "Register success!".to_string(),
                }),
            )
        })
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse {
                    success: false,
                    message: "Failed to register user".to_string(),
                }),
            )
        })
}
