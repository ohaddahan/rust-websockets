mod handle_socket;
mod process_message;
mod ws_handler;
use axum::{routing::any, Router};

use crate::ws_handler::ws_handler;
use axum::routing::get;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use common::mimalloc_memory_loop::mimalloc_memory_loop;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use mimalloc::MiMalloc;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "libc")]
use common::malloc_trim_memory_loop::malloc_trim_memory_loop;
use common::memory_stats_loop::memory_stats_loop;
use common::options::Options;
use common::stats::stats;
use common::tcp_listener::multi_server;
use common::version::version;
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
    let app = Router::new()
        .route("/ws", any(ws_handler))
        .route("/stats", any(stats))
        .route("/", get(version));

    let args = Options::server_options()?;
    let _ = multi_server(app, args.urls).await;
    #[cfg(feature = "mimalloc")]
    let _mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());
    let memory_stats_loop_task = tokio::spawn(memory_stats_loop());
    #[cfg(feature = "libc")]
    let _malloc_trim_memory_loop_task = tokio::spawn(malloc_trim_memory_loop());
    // let server_task = server(listener, app);
    tokio::select! {
        // _o = server_task => panic!("server_task dead"),
        _o = memory_stats_loop_task => panic!("memory_stats_loop_task dead")
    }
}
