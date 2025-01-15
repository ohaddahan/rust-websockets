use memory_stats::memory_stats;
use std::time::Duration;
use tokio::time::sleep;

pub async fn memory_stats_loop() {
    loop {
        if let Some(usage) = memory_stats() {
            println!("Current physical memory usage: {}", usage.physical_mem);
            println!("Current virtual memory usage: {}", usage.virtual_mem);
        } else {
            println!("Couldn't get the current memory usage :(");
        }
        sleep(Duration::from_millis(10_000)).await
    }
}
