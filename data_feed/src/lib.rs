use decimal;

#[derive(Debug)]
pub struct Data{
    time_stamp: u64,
    current_price: decimal::Price,
}

impl Data {
    pub fn new(time_stamp: u64, current_price: decimal::Price) -> Data {
        Data {
            time_stamp,
            current_price,
        }
    }
}

pub fn get_data() -> Data {
    Data::new(123456789, decimal::Price::new(100.0))
}