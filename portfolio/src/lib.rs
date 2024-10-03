use strategy;

#[derive(Debug)]
struct Order{
    symbol: String,
    side: String,
    size: i32,
}

pub fn make_order(_decision: strategy::Order)  {
    log::info!("Making Order");
}

