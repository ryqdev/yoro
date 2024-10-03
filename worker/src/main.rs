use std::{
    io::Write,
};
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};

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

fn main() {
    // Init log
    init_log();

    let listener = TcpListener::bind("127.0.0.1:18888").unwrap();
    let pool = worker::ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.run(||{
            handle_data(stream);
        })
    }
}

const GET_REQUEST: &'static str = "GET / HTTP/1.1";
const POST_REQUEST: &'static str = "POST / HTTP/1.1";

fn handle_data(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    match &request_line[..] {
        GET_REQUEST => handle_get(),
        POST_REQUEST => handle_post(),
        _ => handle_error(),
    };

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
}

fn handle_post() {
    log::info!("POST request received");

    portfolio::make_order(strategy::get_order(data_feed::get_data_from_stream()));
}

fn handle_get() {
    log::info!("GET request received");
}

fn handle_error() {
    log::info!("Error request received");
}