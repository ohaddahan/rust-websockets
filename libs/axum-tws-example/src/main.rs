mod handle_upgrade;
mod handle_ws;

use crate::handle_upgrade::handle_upgrade;
use axum::Router;

use axum::routing::any;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use mimalloc::MiMalloc;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(all(feature = "libc"))]
use common::malloc_trim_memory_loop::malloc_trim_memory_loop;
use common::memory_stats_loop::memory_stats_loop;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use common::mimalloc_memory_loop::mimalloc_memory_loop;
use common::options::Options;
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
    let args = Options::parse_verbose();
    let app = Router::new().route("/", any(handle_upgrade));
    let url = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&url).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    #[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
    let _mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());
    let memory_stats_loop_task = tokio::spawn(memory_stats_loop());
    let server_task = server(listener, app);
    #[cfg(all(feature = "libc"))]
    let _malloc_trim_memory_loop_task = tokio::spawn(malloc_trim_memory_loop());
    tokio::select! {
        _o = server_task => panic!("server_task dead"),
        _o = memory_stats_loop_task => panic!("memory_stats_loop_task dead")
    }
}
