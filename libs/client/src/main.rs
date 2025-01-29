mod process_message;
mod spawn_client;

use crate::spawn_client::spawn_client;
use common::options::Options;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse_verbose();
    let start_time = Instant::now();
    let server = match args.url {
        Some(url) => Arc::new(url),
        None => Arc::new(format!("ws://{}:{}", args.ip, args.port)),
    };
    let mut clients = (0..args.num_clients)
        .map(|cli| tokio::spawn(spawn_client(server.clone(), cli, args.delay)))
        .collect::<FuturesUnordered<_>>();
    println!("launched {}", clients.len());
    while clients.next().await.is_some() {}
    let end_time = Instant::now();
    println!(
        "Total time taken {:#?} with {} concurrent clients.",
        end_time - start_time,
        args.num_clients
    );
    Ok(())
}
