mod accept_connection;
mod mimalloc_memory_loop;

use crate::accept_connection::accept_connection;
use clap::Parser;
use options::Options;
use std::io::Error;
use tokio::net::TcpListener;

#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use mimalloc::MiMalloc;
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use crate::mimalloc_memory_loop::mimalloc_memory_loop;
#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Options::parse();
    let url = format!("localhost:{}", args.port);
    let try_socket = TcpListener::bind(&url).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", url);
    let _mimalloc_memory_loop_task = tokio::spawn(mimalloc_memory_loop());

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, args.buffer_size));
    }
    Ok(())
}
