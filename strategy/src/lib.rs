use data_feed::TickerPrice;

#[derive(Debug)]
enum TradeType {
    Long,
    Short,
}

#[derive(Debug)]
pub struct Decision {
    symbol: String,
    side: TradeType,
    size: i32
}

pub fn get_decision(data: TickerPrice) -> Decision {
    Decision {
        symbol: "Test".to_string(),
        side: TradeType::Long,
        size: 100
    }
}