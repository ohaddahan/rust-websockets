mod handle_upgrade;
mod handle_ws;

use crate::handle_upgrade::handle_upgrade;
use axum::Router;

use axum::routing::{any, get};

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
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use common::mimalloc_memory_loop::mimalloc_memory_loop;
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
    let args = Options::server_options()?;
    let app: Router<_> = Router::new()
        .route("/ws", any(handle_upgrade))
        .route("/stats", any(stats))
        .route("/", get(version));

    let _ = multi_server(app, args.urls).await;
    #[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
    let _mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());
    let memory_stats_loop_task = tokio::spawn(memory_stats_loop());
    #[cfg(feature = "libc")]
    let _malloc_trim_memory_loop_task = tokio::spawn(malloc_trim_memory_loop());
    tokio::select! {
        _o = memory_stats_loop_task => panic!("memory_stats_loop_task dead")
    }
}
