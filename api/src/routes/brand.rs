use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::handler::brand;
use crate::middlewares::user_auth_required;
use crate::AppState;

pub fn brand_routes() -> Router<AppState> {
    Router::new().nest(
        "/brands",
        Router::new()
            .route("/create", post(brand::create_brand))
            .route("/delete/:id", delete(brand::delete_brand))
            .route_layer(middleware::from_fn(user_auth_required))
            .route("/find", get(brand::find_brands)),
    )
}
