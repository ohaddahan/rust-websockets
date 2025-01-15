use crate::process_message::process_message;
use axum::body::Bytes;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn spawn_client(server: Arc<String>, who: usize) {
    let url = format!("{}", server);
    let ws_stream = match connect_async(url).await {
        Ok((stream, _response)) => stream,
        Err(e) => {
            println!("WebSocket handshake for client {who} failed with {e}!");
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    //we can ping the server for start
    sender
        .send(Message::Ping(axum::body::Bytes::from_static(
            b"Hello, Server!",
        )))
        .await
        .expect("Can not send!");

    //spawn an async sender to push some more messages into the server
    let mut send_task = tokio::spawn(async move {
        loop {
            if sender
                .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
                .await
                .is_err()
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(15_000)).await;
        }
    });

    //receiver just prints whatever it gets
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
            println!("send_task for {} ended", who);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            println!("rect_task for {} ended", who);
            send_task.abort();
        }
    }
}
