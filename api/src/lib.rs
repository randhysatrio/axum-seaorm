use axum::http::Method;
use axum::{Router, Server};
use dotenvy::dotenv;
use sea_orm::*;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use ::migration::{Migrator, MigratorTrait};

mod errors;
mod extractor;
mod handler;
mod middlewares;
mod routes;
mod services;
mod utils;

use routes::{auth_routes, brand_routes, cart_routes, category_routes, product_routes};

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DbConn,
}

#[tokio::main]
pub async fn run() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let conn = if let Ok(connection) = Database::connect(database_url).await {
        connection
    } else {
        eprintln!("Failed to connect to database");
        std::process::exit(1);
    };

    if (Migrator::up(&conn, None).await).is_err() {
        eprintln!("Failed to execute database migration");
        std::process::exit(1);
    }

    let app_state = AppState { conn };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(Any);

    let root_router = Router::new()
        .merge(auth_routes())
        .merge(category_routes())
        .merge(brand_routes())
        .merge(product_routes())
        .merge(cart_routes())
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    Server::bind(&addr)
        .serve(root_router.into_make_service())
        .await
        .expect("Failed to run server");
}
