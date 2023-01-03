use axum::routing::delete;
use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::category;
use crate::AppState;

pub fn category_routes() -> Router<AppState> {
    Router::new().nest(
        "/category",
        Router::new()
            .route("/create", post(category::create_category))
            .route("/delete/:id", delete(category::delete_category))
            .route("/find", get(category::find_category)),
    )
}
