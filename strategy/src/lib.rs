#[derive(Debug, Default)]
enum TradeType {
    #[default]
    Long,
    Short,
}

#[derive(Debug, Default)]
pub struct Order {
    symbol: String,
    side: TradeType,
    size: i32,
}


pub fn get_order(data: data_feed::Data) -> Order {
    log::info!("handle strategy");
    Order {
        symbol: data.symbol.clone(),
        side: TradeType::Long,
        size: 1,
    }
}
