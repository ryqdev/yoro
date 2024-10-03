use std::{
    io::Write,
    fs
};
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
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

use worker::{
    ThreadPool,
    Worker
};


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

fn main() {
    // Init log
    init_log();

    let listener = TcpListener::bind("127.0.0.1:18888").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.run(||{
            handle_data(stream);
        })
    }
}

const GET_REQUEST: &'static str = "GET / HTTP/1.1";

fn handle_data(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    match &request_line[..] {
        GET_REQUEST => handle_get(),
        _ => handle_error(),
    };
}

fn handle_post() {
    println!("POST request received");
}

fn handle_get() {
    println!("handle data");
    println!("handle strategy");
    println!("handle order");
}

fn handle_error() {
    println!("Error request received");
}