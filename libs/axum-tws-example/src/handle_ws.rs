use axum_tws::{Message, WebSocket};

pub async fn handle_ws(mut socket: WebSocket) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(Ok(msg)) = socket.recv().await {
        if msg.is_text() {
            socket.send(msg).await?;
        } else if msg.is_ping() {
            let payload = msg.into_payload();
            if let Err(e) = socket.send(Message::pong(payload)).await {
                eprintln!("{}", e);
                break;
            }
        } else if msg.is_close() {
            break;
        }
    }
    Ok(())
}
