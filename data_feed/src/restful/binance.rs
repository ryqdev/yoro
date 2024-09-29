use reqwest;
use std::error::Error;
use crate::TickerPrice;


pub fn get_from_binance_api() -> Result<TickerPrice, Box<dyn Error>> {
    let symbol = "BTCUSDT";

    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(format!("Failed to get data: HTTP {}", response.status()).into());
    }

    let ticker_price: TickerPrice = response.json()?;
    println!("Symbol: {}, Price: {}", ticker_price.symbol, ticker_price.price);

    Ok(ticker_price)
}