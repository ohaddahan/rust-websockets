use anyhow::anyhow;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser, Clone, Serialize, Deserialize, Debug)]
pub struct ClientOptions {
    #[clap(long, default_value = "10000")]
    pub num_clients: usize,
    #[clap(long, default_value = "1")]
    pub delay: u64,
    #[clap(long)]
    pub url: String,
}

#[derive(Parser, Clone, Serialize, Deserialize, Debug)]
pub struct ServerOptions {
    #[clap(long, default_value = "1024")]
    pub buffer_size: usize,
    #[clap(long, num_args = 1.., value_delimiter = ' ')]
    pub urls: Vec<String>,
}

#[derive(Parser, Clone, Serialize, Deserialize, Debug)]
pub enum Options {
    ServerOptions(ServerOptions),
    ClientOptions(ClientOptions),
}

impl Options {
    pub fn parse_verbose() -> Self {
        let mut args = Options::parse();
        if let Ok(url) = env::var("URL") {
            match args {
                Options::ServerOptions(ref mut margs) => margs.urls = vec![url],
                Options::ClientOptions(ref mut margs) => margs.url = url,
            }
        }
        println!("args = {:#?}", args);
        args
    }

    pub fn server_options() -> anyhow::Result<ServerOptions> {
        let args = Options::parse_verbose();
        match args {
            Options::ServerOptions(args) => {
                if args.urls.is_empty() {
                    return Err(anyhow!("Must pass in some URLS"));
                }
                Ok(args)
            }
            _ => Err(anyhow!("Provide server options")),
        }
    }

    pub fn client_options() -> anyhow::Result<ClientOptions> {
        let args = Options::parse_verbose();
        match args {
            Options::ClientOptions(args) => Ok(args),
            _ => Err(anyhow!("Provide client options")),
        }
    }
}
