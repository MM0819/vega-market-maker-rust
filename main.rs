use std::collections::HashMap;
use exchange::binance::Binance;

mod entity;
mod exchange;

fn main() {
    let binance = Binance {
        markets: HashMap::from([])
    };
    println!("{:?}", binance);
}
