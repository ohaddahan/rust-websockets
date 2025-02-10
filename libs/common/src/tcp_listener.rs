use crate::options::ServerOptions;
use tokio::net::TcpListener;

pub async fn get_tcp_listener(args: &ServerOptions) -> anyhow::Result<TcpListener> {
    let listener = TcpListener::bind(&args.url).await?;
    println!("listening on {}", listener.local_addr()?);
    Ok(listener)
}
