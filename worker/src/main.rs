use std::{
    io::Write,
    fs
};

use data_feed;
use strategy;
use portfolio;

use serde_derive::Deserialize;
use anyhow::Result;
use env_logger;
use log;
use data_feed::Data;
use strategy::Decision;

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
    fn get_data_feed(&mut self) -> &mut BaseWorker;
    fn get_decision(&mut self) -> &mut BaseWorker;
    fn make_order(&self);
}

#[derive(Debug,Default)]
struct BaseWorker {
    data_feed: Data,
    decision: Decision
}

impl Worker for BaseWorker {
    fn new() -> Box<dyn Worker>
    where Self: Sized
    {
        Box::new(
            BaseWorker {
                data_feed: Data::default(),
                decision: Decision::default()
            }
        )
    }
    fn get_data_feed(&mut self) -> &mut BaseWorker{
        self.data_feed = data_feed::get_data_feed("BTCUSDT".to_string());
        self
    }

    fn get_decision(&mut self) -> &mut BaseWorker{
        self.decision = strategy::BaseOracle::get_decision(&self.data_feed);
        self
    }

    fn make_order(&self) {
        portfolio::make_order();
        log::info!("Make order {:?}", self);
    }
}


fn main() {
    // Init log
    init_log();

    // Load confing
    let conf = init_config("config.toml").expect("Failed to parse config file");

    // Pipeline
    BaseWorker::new()
        .get_data_feed()
        .get_decision()
        .make_order();
}