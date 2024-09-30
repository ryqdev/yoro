use std::{
    io::Write,
    fs
};

use data_feed;
use strategy;
use portfolio;

use serde_derive::Deserialize;
use anyhow::Result;
use env_logger::{Builder, Env};
use log::LevelFilter;

fn init_log() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    let level = env.log_level();

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record.file().unwrap_or_else(|| "unknown_file"),
                record.line().unwrap_or_else(|| 0),
                record.level(),
                record.args()
            )
        })
        .filter_level(level)
        .init()
        .expect("Logger initialization failed");
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