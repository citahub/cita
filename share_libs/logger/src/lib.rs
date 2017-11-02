extern crate log;
extern crate env_logger;
extern crate time;

use env_logger::{LogBuilder, LogTarget};
use log::{LogRecord, LogLevelFilter};
use std::env;
use std::sync::{Once, ONCE_INIT};

static INIT_LOG: Once = ONCE_INIT;
// use in application
pub fn init() {
    init_filter(LogLevelFilter::Info);
}

// use in unit case
pub fn silent() {
    init_filter(LogLevelFilter::Off);
}
fn init_filter(filter: LogLevelFilter) {
    INIT_LOG.call_once(|| {
        let format = |record: &LogRecord| {
            let now = time::now();
            format!("{},{:03} - {} - {}", time::strftime("%Y%m%d %H:%M:%S", &now).unwrap(), now.tm_nsec / 1000_000, record.level(), record.args())
        };

        let mut builder = LogBuilder::new();
        // log to stdout
        builder.target(LogTarget::Stdout);
        builder.format(format).filter(None, filter);

        if env::var("RUST_LOG").is_ok() {
            builder.parse(&env::var("RUST_LOG").unwrap());
        }
        let _ = builder.init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn it_works() {
        let mut handlers = Vec::new();
        const THREAD_NUM: i64 = 2;
        for _ in 0..THREAD_NUM {
            handlers.push(thread::spawn(move || {
                                            init();
                                            thread::sleep(std::time::Duration::from_millis(10));
                                        }));
        }
        loop {
            if handlers.is_empty() {
                break;
            }
            let _ = handlers.pop().unwrap().join();
        }
    }
}
