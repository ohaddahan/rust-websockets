use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Clone, Serialize, Deserialize)]
pub struct Options {
    #[clap(long, default_value = "8000")]
    pub port: u16,
    #[clap(long, default_value = "10000")]
    pub num_clients: usize,
}
