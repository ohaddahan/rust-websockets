mod process_message;
mod spawn_client;

use crate::spawn_client::spawn_client;
use clap::Parser;
use futures_util::stream::FuturesUnordered;
use futures_util::StreamExt;
use options::Options;
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse();
    let start_time = Instant::now();
    let server = Arc::new(format!("ws://localhost:{}", args.port));
    //spawn several clients that will concurrently talk to the server
    let mut clients = (0..args.num_clients)
        .map(|cli| tokio::spawn(spawn_client(server.clone(), cli)))
        .collect::<FuturesUnordered<_>>();
    println!("launched {}", clients.len());
    //wait for all our clients to exit
    while clients.next().await.is_some() {}
    let end_time = Instant::now();
    //total time should be the same no matter how many clients we spawn
    println!(
        "Total time taken {:#?} with {} concurrent clients.",
        end_time - start_time,
        args.num_clients
    );
    Ok(())
}
