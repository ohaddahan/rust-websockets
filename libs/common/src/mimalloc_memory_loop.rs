#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
use libmimalloc_sys::mi_collect;
use std::time::Duration;

pub async fn mimalloc_memory_loop() -> Result<(), anyhow::Error> {
    #[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
    println!("mimalloc activated");
    loop {
        #[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
        unsafe {
            mi_collect(true);
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
