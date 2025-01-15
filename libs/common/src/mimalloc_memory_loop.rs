use libmimalloc_sys::mi_collect;
use std::time::Duration;

pub async fn mimalloc_memory_loop() -> Result<(), anyhow::Error> {
    println!("mimalloc activated");
    loop {
        unsafe {
            mi_collect(true);
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
