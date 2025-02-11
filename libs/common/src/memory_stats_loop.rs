use crate::stats::Stats;
use memory_stats::memory_stats;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

pub fn get_fd_count() -> usize {
    #[cfg(target_os = "linux")]
    {
        if let Ok(me) = procfs::process::Process::myself() {
            if let Ok(fd_count) = me.fd_count() {
                return fd_count;
            }
        }
    }
    0
}

pub fn get_memory_stats() -> f64 {
    if let Some(usage) = memory_stats() {
        usage.physical_mem as f64 / (1024.0 * 1024.0)
    } else {
        0.0
    }
}

pub async fn memory_stats_loop() {
    loop {
        let stats = Stats::default();
        if env::var("VERBOSE").is_ok() {
            println!("stats = {:#?}", stats);
        }
        sleep(Duration::from_millis(5_000)).await
    }
}
