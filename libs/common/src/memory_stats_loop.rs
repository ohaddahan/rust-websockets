use crate::connection_counter::print_connection_counter;
use memory_stats::memory_stats;
use std::time::Duration;
use tokio::time::sleep;

pub fn proc() {
    #[cfg(target_os = "linux")]
    {
        use procfs::process::Process;
        if let Ok(me) = procfs::process::Process::myself() {
            if let Ok(fd_count) = me.fd_count() {
                println!("fd_count = {}", fd_count);
            }
        }
    }
}

pub async fn memory_stats_loop() {
    let mut count = 0;
    loop {
        proc();
        print_connection_counter();
        if let Some(usage) = memory_stats() {
            println!(
                "{count}::Current physical memory usage: {}[MB]",
                usage.physical_mem as f64 / (1024.0 * 1024.0)
            );
        } else {
            println!("{count}::Couldn't get the current memory usage :(");
        }
        count += 1;
        sleep(Duration::from_millis(5_000)).await
    }
}
