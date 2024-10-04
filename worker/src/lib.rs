use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct TomlData {
    config: Config,
}

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

pub struct Worker {
    symbol: String
}


impl Worker {
    pub fn init(){
        let config = init_config("config.toml").unwrap();
    }

    pub fn run() {
        portfolio::make_order(strategy::get_order(data_feed::get_data_from_stream()));
    }
}
