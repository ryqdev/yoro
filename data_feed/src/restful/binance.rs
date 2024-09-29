use reqwest;
use std::error::Error;
use crate::Data;


pub fn get_from_binance_api(symbol: String) -> Result<Data, Box<dyn Error>> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(format!("Failed to get data: HTTP {}", response.status()).into());
    }

    let ticker_price: Data = response.json()?;
    println!("Symbol: {}, Price: {}", ticker_price.symbol, ticker_price.price);

    Ok(ticker_price)
}