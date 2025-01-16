use crate::handle_socket::handle_socket;
use axum::extract::ConnectInfo;
use axum::response::IntoResponse;
use axum_ws_extractor::WebSocketUpgrade;
use std::net::SocketAddr;

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.read_buffer_size(1024)
        .on_upgrade(move |socket| handle_socket(socket, addr))
}
