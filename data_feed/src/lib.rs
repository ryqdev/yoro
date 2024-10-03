mod restful;
use restful::binance::get_from_binance_api;
use serde::Deserialize;


#[derive(Deserialize, Debug, Default)]
pub struct Data {
    pub symbol: String,
    pub price: String,
}

pub fn get_data_feed(symbol: String) -> Data {
    get_from_binance_api(symbol).expect("Failed to get data from Binance API")
}

pub fn get_data_from_stream() -> Data {
    log::info!("get data from stream");
    Data{ symbol: "".to_string(), price: "".to_string() }
}