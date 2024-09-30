use std::{
    io::Write,
    fs
};

use data_feed::{
    Data,
    get_data_feed
};

use strategy::{
    Decision,
    BaseOracle
};

use portfolio::{
    make_order
};

use serde_derive::Deserialize;
use anyhow::Result;
use env_logger;
use log;

fn init_log() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown_file"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();
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
    time: String,
    strategy: String,
    cash: f64
}

fn init_config(config_file_path: &str) -> Result<TomlData>{
    log::info!("start parsing config from {config_file_path}");
    let config_data: TomlData = toml::from_str(&*fs::read_to_string(config_file_path)?)?;
    log::info!("{:?}", config_data);
    Ok(config_data)
}

trait Worker {
    fn new() -> Box<dyn Worker> where Self: Sized;
    fn set_data_feed(&mut self) -> &mut dyn Worker;
    fn set_decision(&mut self) -> &mut dyn Worker;
    fn set_order(&mut self) -> &mut dyn Worker;
    fn run(&self);
}

type DataFeedProcess = fn(String) -> Data;
type StrategyProcess = fn(&Data) -> Decision;
type OrderProcess = fn(&Decision);

#[derive(Debug)]
struct BaseWorker {
    data_feed: DataFeedProcess,
    strategy: StrategyProcess,
    order: OrderProcess,
}

impl Worker for BaseWorker {
    fn new() -> Box<dyn Worker>
    where Self: Sized
    {
        Box::new(
            BaseWorker {
                data_feed: get_data_feed,
                strategy: BaseOracle::get_decision,
                order: make_order,
            }
        )
    }
    fn set_data_feed(&mut self) -> &mut dyn Worker {
        self.data_feed = get_data_feed;
        self
    }

    fn set_decision(&mut self) -> &mut dyn Worker {
        self.strategy = BaseOracle::get_decision;
        self
    }

    fn set_order(&mut self) -> &mut dyn Worker {
        self.order = make_order;
        log::info!("Make order {:?}", self);
        self
    }

    fn run(&self) {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let data = (self.data_feed)("BTCUSDT".to_string());
            println!("self: {:?}", data);
        }
    }
}


fn main() {
    // Init log
    init_log();

    // Load confing
    let conf = init_config("config.toml").expect("Failed to parse config file");

    // Pipeline
    // TODO: use spawn to create multiple workers
    BaseWorker::new().set_data_feed().set_decision().set_order().run();
}