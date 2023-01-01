use axum::{Router, Server};
use dotenvy::dotenv;
use sea_orm::*;
use std::env;
use std::net::SocketAddr;

pub mod database;
pub mod handler;
pub mod routes;

use routes::auth_routes;

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

    let conn = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState { conn };

    let root_router = Router::new().merge(auth_routes()).with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    Server::bind(&addr)
        .serve(root_router.into_make_service())
        .await
        .expect("Failed to run server");
}
