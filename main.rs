#![allow(unused)]

use std::collections::HashMap;
use exchange::binance::Binance;

mod entity;
mod exchange;

fn main() {
    let binance = Binance {
        markets: HashMap::from([])
    };
    binance.stream_prices()
}
