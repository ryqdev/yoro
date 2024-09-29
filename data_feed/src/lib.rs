mod restful;
use restful::binance::get_from_binance_api;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct TickerPrice {
    symbol: String,
    price: String,
}

pub fn get_data() -> TickerPrice {
    get_from_binance_api().expect("Failed to get data from Binance API")
}