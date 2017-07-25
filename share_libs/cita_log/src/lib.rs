extern crate time;
extern crate log;
extern crate env_logger;

use env_logger::LogBuilder;
use log::{LogLevelFilter, LogRecord};
use std::env;

pub fn format(level: LogLevelFilter) {
    let format = |record: &LogRecord| {
        let t = time::now();
        format!("{},{:03} - {} - {}",
                time::strftime("%Y-%m-%d %H:%M:%S", &t).unwrap(),
                t.tm_nsec / 1000_000,
                record.level(),
                record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, level);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
