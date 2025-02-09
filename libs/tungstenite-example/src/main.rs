mod accept_connection;

use crate::accept_connection::accept_connection;
use clap::Parser;
use std::io::Error;
use tokio::net::TcpListener;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use mimalloc::MiMalloc;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "libc")]
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Options::parse();
    let url = format!("0.0.0.0:{}", args.port);
    let try_socket = TcpListener::bind(&url).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", url);
    #[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
    let _mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());
    let _memory_stats_loop_task = tokio::spawn(memory_stats_loop());
    #[cfg(feature = "libc")]
    let _malloc_trim_memory_loop_task = tokio::spawn(malloc_trim_memory_loop());
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, args.buffer_size));
    }
    Ok(())
}
