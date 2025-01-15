mod handle_socket;
mod process_message;
mod ws_handler;

use axum::{routing::any, Router};

use crate::ws_handler::ws_handler;
use clap::Parser;
use options::Options;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let args = Options::parse();
    // build our application with some routes
    let app = Router::new().route("/", any(ws_handler));
    // run it with hyper
    let url = format!("localhost:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
