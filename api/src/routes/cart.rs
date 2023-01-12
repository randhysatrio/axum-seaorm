use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handler::cart, middlewares::user_auth_required, AppState};

pub fn cart_routes() -> Router<AppState> {
    Router::new().nest(
        "/carts",
        Router::new()
            .route("/create-or-update", post(cart::create_or_update_cart))
            .route("/find", get(cart::find_carts))
            .route_layer(middleware::from_fn(user_auth_required)),
    )
}
