use strategy;

struct Portfolio {
    symbol: String,
    balance: f32,
    position: f32,
    commission: f32,
}

pub fn make_order(decision: strategy::Decision)  {
    println!("Order made from decision: {:?}", decision);
}
