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
    fn get_data_feed(&self) -> &BaseWorker;
    fn get_decision(&self) -> &BaseWorker;
    fn make_order(&self);
}

struct BaseWorker;

impl Worker for BaseWorker {
    fn get_data_feed(&self) -> &BaseWorker {
        log::info!("Get data feed");
        self
    }

    fn get_decision(&self) -> &BaseWorker{
        log::info!("Get decision");
        self
    }

    fn make_order(&self) {
        log::info!("Make order");
    }
}


fn main() {
    // Init log
    init_log();

    // Load confing
    let conf = init_config("config.toml").expect("Failed to parse config file");

    // Get a worker
    let worker: Box<dyn Worker> = match conf.config.strategy.as_str() {
        "BaseOracle" => Box::new(BaseWorker),
        _ => panic!("No such strategy"),
    };

    // Pipeline
    worker.get_data_feed().get_decision().make_order();
}