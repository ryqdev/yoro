use std::{fs, thread};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::Thread;
use serde_derive::Deserialize;
use data_feed::{get_data_feed, Data};
use portfolio::make_order;
use strategy::{BaseOracle, Decision};

type DataFeedProcess = fn(String) -> Data;
type StrategyProcess = fn(&Data) -> Decision;
type OrderProcess = fn(&Decision);


pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)))
        }
        Self {
            workers,
            tx
        }
    }

    pub fn run<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.tx.send(job).unwrap()
    }

}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    // data_feed: DataFeedProcess,
    // strategy: StrategyProcess,
    // order: OrderProcess,
    // config: Config
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move|| loop {
            let job = rx.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        Self {id, thread}
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

