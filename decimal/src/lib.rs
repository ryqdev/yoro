// TODO: implement the Price struct

#[derive(Debug)]
pub struct Price {
    value: f64, 
}

impl Price {
    pub fn new(value: f64) -> Price {
        Price {
            value,
        }
    }
}