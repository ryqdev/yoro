use std::{
    io::Write,
};
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

use env_logger;
use log;

use worker::{
    ThreadPool,
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
    log::info!("POST request received");
}

fn handle_get() {
    let data = data_feed::get_data_from_stream();
    let order = strategy::get_order(data);
    portfolio::make_order(order);
}

fn handle_error() {
    log::info!("Error request received");
}