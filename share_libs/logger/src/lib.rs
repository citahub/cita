// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
extern crate chan_signal;
extern crate chrono;
extern crate log4rs;
#[macro_use]
extern crate log;

use chan_signal::Signal;
use chrono::Local;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::fs;
use std::str::FromStr;
use std::sync::{Once, ONCE_INIT};
use std::thread;
use std::vec::Vec;

static INIT_LOG: Once = ONCE_INIT;

pub fn init_config(service_name: &str) {
    INIT_LOG.call_once(|| {
        // parse RUST_LOG
        let mut filter: (Vec<String>, LogLevelFilter) = (Vec::new(), LogLevelFilter::Info);
        if let Ok(s) = env::var("RUST_LOG") {
            filter = env_parse(&s);
        }

        // log4rs config
        let log_name = format!("logs/{}.log", service_name.to_string());
        let filter_clone = filter.clone();
        let config = config_file_appender(&log_name, filter_clone);
        let handle = log4rs::init_config(config).unwrap();

        // logrotate via signal(USR1)
        let signal = chan_signal::notify(&[Signal::USR1]);

        // Any and all threads spawned must come after the first call to notify (or notify_on).
        // This is so all spawned threads inherit the blocked status of signals.
        // If a thread starts before notify is called, it will not have the correct signal mask.
        // When a signal is delivered, the result is indeterminate.
        let service_name_clone = service_name.to_string();
        thread::spawn(move || {
            loop {
                //Blocks until this process is sent an USR1 signal.
                signal.recv().unwrap();

                //rotate current log file
                let time_stamp = Local::now().format("_%Y-%m-%d_%H-%M-%S");
                let log_rotate_name = format!("logs/{}{}.log", &service_name_clone, time_stamp);
                if let Err(e) = fs::rename(&log_name, log_rotate_name) {
                    warn!("logrotate failed because of {:?}", e.kind());
                    continue;
                }

                //reconfig
                let filter_clone = filter.clone();
                let new_config = config_file_appender(&log_name, filter_clone);
                handle.set_config(new_config);
            }
        });
    });
}

// use in tests
pub fn init() {
    INIT_LOG.call_once(|| {
        // parse RUST_LOG
        let mut filter: (Vec<String>, LogLevelFilter) = (Vec::new(), LogLevelFilter::Info);
        if let Ok(s) = env::var("RUST_LOG") {
            filter = env_parse(&s);
        }
        let config = config_console_appender(filter);
        log4rs::init_config(config).unwrap();
    });
}

// use in unit case
pub fn silent() {
    INIT_LOG.call_once(|| {
        let config = Config::builder()
            .build(Root::builder().build(LogLevelFilter::Off))
            .unwrap();
        log4rs::init_config(config).unwrap();
    });
}

// simple parse env (e.g: crate1,crate2::mod,crate3::mod=trace)
fn env_parse(s: &str) -> (Vec<String>, LogLevelFilter) {
    let mut mod_list = Vec::new();
    let mut section = s.split('=');

    //parse crate or modules
    let mods = section.next();
    if mods.is_some() {
        for mod_name in mods.unwrap().split(',') {
            if mod_name.len() != 0 {
                mod_list.push(mod_name.to_string());
            }
        }
    }

    //parse log level
    let level = match section.next() {
        Some(level_str) => match LogLevelFilter::from_str(level_str.trim()) {
            Ok(log_level) => (log_level),
            Err(_) => (LogLevelFilter::Info),
        },
        None => (LogLevelFilter::Info),
    };

    (mod_list, level)
}

// create loggers
fn creat_loggers(filter: (Vec<String>, LogLevelFilter), appender: String) -> Vec<Logger> {
    let mut loggers = Vec::new();

    if filter.0.len() == 0 {
        return loggers;
    }

    //creat loggers via module/crate and log level
    for mod_name in filter.0 {
        let appender_clone = appender.clone();
        let logger = Logger::builder()
            .appender(appender_clone)
            .additive(false)
            .build(mod_name, filter.1);
        loggers.push(logger);
    }

    loggers
}
// creat FileAppender config
fn config_file_appender(file_path: &str, filter: (Vec<String>, LogLevelFilter)) -> Config {
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build(file_path)
        .unwrap();

    let mut config_builder = Config::builder().appender(Appender::builder().build("requests", Box::new(requests)));

    let loggers = creat_loggers(filter, "requests".to_string());

    // config crate or module log level
    if loggers.len() != 0 {
        config_builder = config_builder.loggers(loggers.into_iter());
    }

    //config global log level
    let config = config_builder
        .build(
            Root::builder()
                .appender("requests")
                .build(LogLevelFilter::Info),
        )
        .unwrap();

    config
}

// creat ConsoleAppender config
fn config_console_appender(filter: (Vec<String>, LogLevelFilter)) -> Config {
    let stdout = ConsoleAppender::builder().build();

    let mut config_builder = Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)));

    let loggers = creat_loggers(filter, "stdout".to_string());

    // config crate or module log level
    if loggers.len() != 0 {
        config_builder = config_builder.loggers(loggers.into_iter());
    }

    //config global log level
    let config = config_builder
        .build(
            Root::builder()
                .appender("stdout")
                .build(LogLevelFilter::Info),
        )
        .unwrap();

    config
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
