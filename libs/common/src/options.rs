use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser, Clone, Serialize, Deserialize, Debug)]
pub struct Options {
    #[clap(long, default_value = "8000")]
    pub port: u16,
    #[clap(long, default_value = "10000")]
    pub num_clients: usize,
    #[clap(long, default_value = "1024")]
    pub buffer_size: usize,
    #[clap(long, default_value = "localhost")]
    pub ip: String,
    #[clap(long, default_value = "1")]
    pub delay: u64,
    #[clap(long)]
    pub url: Option<String>,
}

impl Options {
    pub fn parse_verbose() -> Self {
        let mut args = Options::parse();
        println!("pre env args = {:#?}", args);
        if let Ok(port) = env::var("PORT") {
            args.port = port.parse().unwrap_or(8000);
        }
        println!("post env args = {:#?}", args);
        args
    }
}
