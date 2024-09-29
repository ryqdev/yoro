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
    size: i32,
}

#[derive(Debug)]
pub struct BaseOracle;

impl  BaseOracle {
    pub fn get_decision(data: Data) -> Decision {  
        Decision {
            symbol: data.symbol,
            side: TradeType::Long,
            size: 1,
        }
    }
}
