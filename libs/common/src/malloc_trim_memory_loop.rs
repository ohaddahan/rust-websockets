#[cfg(all(feature = "libc", not(feature = "mimalloc"), not(feature = "jemalloc")))]
use libc;
use std::time::Duration;

pub async fn malloc_trim_memory_loop() -> Result<(), anyhow::Error> {
    #[cfg(all(feature = "libc", not(feature = "mimalloc"), not(feature = "jemalloc")))]
    println!("malloc_trim");
    loop {
        #[cfg(all(feature = "libc", not(feature = "mimalloc"), not(feature = "jemalloc")))]
        unsafe {
            let _ = libc::malloc_trim(0usize);
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
