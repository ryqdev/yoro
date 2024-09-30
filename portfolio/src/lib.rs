use strategy;
use strategy::Decision;

#[derive(Debug)]
struct Order{
    symbol: String,
    side: String,
    size: i32,
}

pub fn make_order(_decision: &Decision)  {
    println!("Making Order");
}

