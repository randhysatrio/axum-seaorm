use axum::middleware;
use axum::routing::{delete, patch};
use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::category;
use crate::middlewares::user_auth_required;
use crate::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new().nest(
        "/categories",
        Router::new()
            .route("/create", post(category::create_category))
            .route("/delete/:id", delete(category::delete_category))
            .route("/restore/:id", patch(category::restore_category))
            .route_layer(middleware::from_fn(user_auth_required))
            .route("/find", get(category::find_category)),
    )
}
