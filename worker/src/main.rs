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

fn parse_config(config_file_path: &str) -> Result<TomlData>{
    log::info!("start parsing config from {config_file_path}");
    let config_data: TomlData = toml::from_str(&*fs::read_to_string(config_file_path)?)?;
    log::info!("{:?}", config_data);
    Ok(config_data)
}

fn main() {
    // Init log
    init_log();

    // Load confing
    let conf = parse_config("config.toml").expect("Failed to parse config file");

    let data = data_feed::get_data(conf.config.symbol);
    let decision = strategy::BaseOracle::get_decision(data);
    portfolio::make_order(decision);
}