mod accept_connection;

use crate::accept_connection::accept_connection;

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
use common::tcp_listener::get_tcp_listener;
#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(feature = "jemalloc", not(feature = "mimalloc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::server_options()?;
    let listener = get_tcp_listener(&args).await?;
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
