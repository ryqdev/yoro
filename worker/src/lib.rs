use std::fs;
use serde_derive::Deserialize;
use data_feed::{get_data_feed, Data};
use portfolio::make_order;
use strategy::{BaseOracle, Decision};

type DataFeedProcess = fn(String) -> Data;
type StrategyProcess = fn(&Data) -> Decision;
type OrderProcess = fn(&Decision);




#[derive(Debug)]
pub struct Worker {
    data_feed: DataFeedProcess,
    strategy: StrategyProcess,
    order: OrderProcess,
    config: Config
}

impl Worker {
    pub fn new() -> Self {
        Self {
            data_feed: get_data_feed,
            strategy: BaseOracle::get_decision,
            order: make_order,
            config: Config {
                broker: "".to_string(),
                symbol: "".to_string(),
                strategy: "".to_string(),
            },
        }
    }

    pub fn load_config(&mut self, config_file_path: &str) -> &mut Self{
        self.config = init_config(config_file_path).expect("Failed to parse config file");
        self
    }

    pub fn set_data_feed(&mut self) -> &mut Self{
        self.data_feed = get_data_feed;
        self
    }

    pub fn set_decision(&mut self) -> &mut Self {
        self.strategy = BaseOracle::get_decision;
        self
    }

    pub fn set_order(&mut self) -> &mut Self {
        self.order = make_order;
        log::info!("Make order {:?}", self);
        self
    }

    pub fn run(&self) {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let data = (self.data_feed)(self.config.symbol.clone());
            println!("self: {:?}", data);
        }
    }
}

// Top level struct to hold the TOML data.
#[derive(Deserialize, Debug)]
struct TomlData {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug)]
struct Config {
    broker: String,
    symbol: String,
    strategy: String,
}

fn init_config(config_file_path: &str) -> anyhow::Result<Config> {
    log::info!("start parsing config from {config_file_path}");
    let config_data: TomlData = toml::from_str(&*fs::read_to_string(config_file_path)?)?;
    log::info!("{:?}", config_data);
    Ok(config_data.config)
}

