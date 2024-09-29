mod restful;
use restful::binance::get_from_binance_api;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Data {
    pub symbol: String,
    pub price: String,
}

pub fn get_data() -> Data {
    get_from_binance_api().expect("Failed to get data from Binance API")
}