use clap::Parser;
use memory_stats::memory_stats;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Clone, Serialize, Deserialize)]
pub struct Options {
    #[clap(long, default_value = "8000")]
    pub port: u16,
    #[clap(long, default_value = "10000")]
    pub num_clients: usize,
    #[clap(long, default_value = "1024")]
    pub buffer_size: usize,
}

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
