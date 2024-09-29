use data_feed::Data;

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

pub fn get_decision(data: Data) -> Decision {
    Decision {
        symbol: "Test".to_string(),
        side: TradeType::Long,
        size: 100
    }
}