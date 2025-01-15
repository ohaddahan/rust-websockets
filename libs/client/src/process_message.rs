use std::ops::ControlFlow;
use tokio_tungstenite::tungstenite::Message;

pub fn process_message(msg: Message, _who: usize) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(_t) => {}
        Message::Binary(_d) => {}
        Message::Close(_c) => {
            return ControlFlow::Break(());
        }
        Message::Pong(_v) => {}
        Message::Ping(_v) => {}
        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
    }
    ControlFlow::Continue(())
}
