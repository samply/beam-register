use std::sync::LazyLock;

use clap::Parser;
use config::Config;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

mod routes;
mod config;
mod beam_app;
mod security;
mod app_register;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

#[tokio::main]
async fn main() {
    tracing_subscriber::FmtSubscriber::new().init();
    // Build the application using routes from the `routes` module
    let app = routes::create_router();
    let listener = TcpListener::bind(CONFIG.bind_addr).await.unwrap();
    info!("Server running at http://{}", CONFIG.bind_addr);
    
    // Use `axum::serve` to run the server
    axum::serve(listener, app).await.unwrap();

}