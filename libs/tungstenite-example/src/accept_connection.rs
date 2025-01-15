use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::TcpStream;
use tungstenite::protocol::WebSocketConfig;

pub async fn accept_connection(stream: TcpStream, buffer_size: usize) {
    let config = WebSocketConfig::default()
        .read_buffer_size(buffer_size)
        .write_buffer_size(buffer_size);
    let ws_stream = tokio_tungstenite::accept_async_with_config(stream, Some(config))
        .await
        .expect("Error during the websocket handshake occurred");
    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    let _ = read
        .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await;
}
