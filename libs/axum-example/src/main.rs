mod handle_socket;
mod mimalloc_memory_loop;
mod process_message;
mod ws_handler;
use axum::{routing::any, Router};

use crate::mimalloc_memory_loop::mimalloc_memory_loop;
use crate::ws_handler::ws_handler;
use clap::Parser;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use mimalloc::MiMalloc;
use options::Options;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
use tikv_jemallocator::Jemalloc;
#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub async fn server(listener: TcpListener, app: Router) {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse();
    // build our application with some routes
    let app = Router::new().route("/", any(ws_handler));
    // run it with hyper
    let url = format!("localhost:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());
    let server_task = server(listener, app);
    tokio::select! {
        _o = server_task => panic!("server_task dead"),
        _o = mimalloc_memory_loop_task => panic!("mimalloc_memory_loop_task dead")
    }
}
