use memory_stats::memory_stats;
use std::time::Duration;
use tokio::time::sleep;

pub async fn memory_stats_loop() {
    let mut count = 0;
    loop {
        if let Some(usage) = memory_stats() {
            println!(
                "{count}::Current physical memory usage: {}[MB]",
                usage.physical_mem as f64 / (1024.0 * 1024.0)
            );
        } else {
            println!("{count}::Couldn't get the current memory usage :(");
        }
        count += 1;
        sleep(Duration::from_millis(10_000)).await
    }
}
