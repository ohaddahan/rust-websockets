use crate::process_message::process_message;
use axum::body::Bytes;
use axum_ws_extractor::{Message, WebSocket};
use common::connection_counter::mut_to_connection_counter;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;

pub async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_err()
    {
        return;
    }

    let (mut sender, mut receiver) = socket.split();

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

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });
    mut_to_connection_counter(1);
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => {},
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) =>{},
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }
    mut_to_connection_counter(-1);
}
