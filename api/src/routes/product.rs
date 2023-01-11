use axum::routing::{delete, get, patch, post};
use axum::{middleware, Router};

use crate::handler::product;
use crate::middlewares::user_auth_required;
use crate::AppState;

pub fn product_routes() -> Router<AppState> {
    Router::new().nest(
        "/products",
        Router::new()
            .route("/create", post(product::create_product))
            .route("/delete/:id", delete(product::delete_product))
            .route("/restore/:id", patch(product::restore_product))
            .route("/update/:id", patch(product::update_product))
            .route_layer(middleware::from_fn(user_auth_required))
            .route("/find", get(product::find_products)),
    )
}
