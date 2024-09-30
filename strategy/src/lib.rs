use data_feed::Data;

#[derive(Debug, Default)]
enum TradeType {
    #[default]
    Long,
    Short,
}

#[derive(Debug, Default)]
pub struct Decision {
    symbol: String,
    side: TradeType,
    size: i32,
}

#[derive(Debug)]
pub struct BaseOracle;

impl  BaseOracle {
    pub fn get_decision(data: &Data) -> Decision {
        Decision {
            symbol: data.symbol.clone(),
            side: TradeType::Long,
            size: 1,
        }
    }
}
