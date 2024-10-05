use std::{fs, io::{Write}, thread};
use std::fs::File;
use std::time::Duration;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    response::{IntoResponse, Response},
};
use chrono::Local;

use serde::{Deserialize, Serialize};

fn init_log() {
    let output_file = Box::new(File::create("../btc_ustd.log").expect("Can't create file"));

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] [{}] - {}",
                record.file().unwrap_or("unknown_file"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args(),
            )
        })
        .target(env_logger::Target::Pipe(output_file))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .filter_level(log::LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() {

    init_log();

    let worker = worker::Worker::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/data", post(handle_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:18888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, yoro!"
}

#[derive(Deserialize, Debug)]
struct DataStream{
    price: String,
}

async fn handle_post(
    Json(payload): Json<DataStream>,
)  -> impl IntoResponse{
    log::info!("POST request received, payload: {:#?}", payload);

    worker::Worker::run();

    (StatusCode::OK, "[OK]")
}
