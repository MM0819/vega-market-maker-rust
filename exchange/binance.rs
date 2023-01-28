use std::collections::HashMap;
use entity::market::Market;

#[derive(Debug)]
pub(crate) struct Binance {
    pub(crate) markets: HashMap<String, Market>
}