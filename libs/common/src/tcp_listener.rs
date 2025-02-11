use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn get_tcp_listener(url: &str) -> anyhow::Result<TcpListener> {
    let listener = TcpListener::bind(url).await?;
    println!("listening on {}", listener.local_addr()?);
    Ok(listener)
}

pub async fn server(listener: TcpListener, app: Router) {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

pub async fn multi_server(app: Router, urls: Vec<String>) -> anyhow::Result<()> {
    for url in urls {
        let listener = get_tcp_listener(&url).await?;
        let appc = app.clone();
        tokio::spawn(server(listener, appc));
    }
    Ok(())
}
