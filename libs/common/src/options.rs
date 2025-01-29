use clap::Parser;
use serde::{Deserialize, Serialize};

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
}

impl Options {
    pub fn parse_verbose() -> Self {
        let args = Options::parse();
        println!("args = {:#?}", args);
        args
    }
}
