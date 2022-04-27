#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
#[macro_use]
extern crate log;

use std::{process, env::var, fs::{OpenOptions, File}, thread::sleep, time};

use chrono::Local;
use rand::Rng;
use slog::Drain;

lazy_static! {
    static ref LOG_PATH: String = var("LOG_PATH").unwrap_or("main.log".to_owned());
    static ref LIFE_TIME: i64 = var("LIFE_TIME").unwrap_or("10".to_owned()).parse().unwrap();
    static ref FILE: File = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&(*LOG_PATH))
        .map_err(|_| {
            eprintln!("failed to access to log file {}", *LOG_PATH);
            process::exit(1);
        })
        .unwrap();
}

// create logger
fn init_logger(file: &'static File) -> slog::Logger {
    let decorator = slog_term::PlainSyncDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0..1000);
    slog_stdlog::init().map_err(|_| {
        eprintln!("failed to access to log file {}", *LOG_PATH);
        process::exit(2);
    }).unwrap();
    slog::Logger::root(drain, o!("id" => key))
}

fn is_alive(start: i64) -> bool {
    sleep(time::Duration::from_secs(1));
    *LIFE_TIME + start > Local::now().timestamp()
}

fn log_loop() {
    let start = Local::now().timestamp();
    while is_alive(start) {
        info!("global file logger");
    }
}

fn main() {
    // init logger here
    let logger = init_logger(&(*FILE));
    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(logger);
    log_loop()
}
