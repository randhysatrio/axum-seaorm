use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::auth, AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/register", post(auth::register_user))
            .route("/login", post(auth::login))
            .route("/persistent", get(auth::persistent_login)),
    )
}
