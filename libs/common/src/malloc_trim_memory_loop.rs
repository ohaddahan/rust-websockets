#[cfg(target_os = "linux")]
#[cfg(target_env = "gnu")]
use libc;
use std::time::Duration;

pub async fn malloc_trim_memory_loop() -> Result<(), anyhow::Error> {
    #[cfg(target_os = "linux")]
    #[cfg(target_env = "gnu")]
    println!("malloc_trim activated");
    loop {
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(target_os = "linux")]
            #[cfg(target_env = "gnu")]
            let _ = libc::malloc_trim(0usize);
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
