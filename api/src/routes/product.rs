use axum::routing::{delete, get, post};
use axum::Router;

use crate::handler::product;
use crate::AppState;

pub fn product_routes() -> Router<AppState> {
    Router::new().nest(
        "/products",
        Router::new()
            .route("/create", post(product::create_product))
            .route("/delete/:id", delete(product::delete_product))
            .route("/find", get(product::find_products)),
    )
}
