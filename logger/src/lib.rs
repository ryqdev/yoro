use std::{
    thread::JoinHandle,
};
use chrono::Local;

pub struct Handle {
    thread:  Option<JoinHandle<()>>,

}

#[derive(Debug)]
struct Context<P: ToString + Send> {
    path: Option<P>,
    date: chrono::NaiveDate,
}


fn worker<P: ToString + Send>(mut ctx: Context<P>) -> anyhow::Result<()> {
    Ok(())
}

pub fn init<P: ToString + Send + 'static>(path: Option<P>, level: log::LevelFilter) -> Handle {
    let ctx = Context {
        path,
        date: Local::now().date_naive(),
    };

    log::set_max_level(level);

    let thread = std::thread::spawn(move || {
        if let Err(msg) = worker(ctx) {
            eprintln!("error {}", msg);
        }
    });
    Handle{
        thread: Some(thread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    #[test]
    fn test_init_creates_handle() {
        let path = Some("test_path");
        let level = LevelFilter::Info;

        let handle = init(path, level);

        assert!(handle.thread.is_some(), "Expected handle to have a spawned thread");
    }
}
