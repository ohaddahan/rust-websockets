use axum_ws_extractor::Message;
use std::net::SocketAddr;
use std::ops::ControlFlow;

pub fn process_message(msg: Message, _who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(_t) => {}
        Message::Binary(_d) => {}
        Message::Close(_c) => {
            return ControlFlow::Break(());
        }
        Message::Pong(_v) => {}
        Message::Ping(_v) => {}
    }
    ControlFlow::Continue(())
}
