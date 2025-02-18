use crate::handle_ws::handle_ws;
use axum::response::Response;
use axum_tws::{Config, WebSocketUpgrade};
use common::connection_counter::mut_to_connection_counter;

pub async fn handle_upgrade(ws: WebSocketUpgrade) -> Response {
    let config = Config::default().frame_size(4 * 1024 * 1024);
    let ws = ws.config(config);
    ws.on_upgrade({
        move |socket| async {
            mut_to_connection_counter(1);
            if let Err(e) = handle_ws(socket).await {
                println!("websocket error: {:?}", e);
            }
            mut_to_connection_counter(-1);
        }
    })
}
