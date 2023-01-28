use std::collections::HashMap;
use tungstenite::{connect, Message};
use url::Url;
use crate::entity::market::Market;

pub(crate) struct Binance {
    pub(crate) markets: HashMap<String, Market>
}

impl Binance {
    pub(crate) fn stream_prices(&self) {
        let (mut socket, _response) = connect(
            Url::parse("wss://stream.binance.com:443/ws").unwrap()
        ).expect("Can't connect");
        let _ = socket.write_message(Message::Text(r#"{
            "id": 1,
            "method": "SUBSCRIBE",
            "params": ["btcusdt@ticker"]
        }"#.into()));
        loop {
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);
        }
    }
}