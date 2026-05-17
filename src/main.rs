mod config;
mod handlers;
mod models;
mod routes;
mod services;

use axum::Router;
use config::Config;
use routes::webhook::webhook_routes;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Arc::new(Config::from_env());
    let app = Router::new()
        .merge(webhook_routes())
        .with_state(config.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
