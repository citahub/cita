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

#[macro_use]
extern crate log;
extern crate logger;
extern crate rustc_serialize;
extern crate clap;
extern crate util;

pub mod config;
pub mod process;

use clap::{App, SubCommand};
use config::MonitorConfig;
use process::Processes;
use std::env;
use std::thread;
use std::time;

fn main() {
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    logger::init();

    let matches = App::new("monitor")
        .version("0.1")
        .author("Cryptape")
        .about("Monitor the processes")
        .subcommand(SubCommand::with_name("start")
                        .about("Start all proccesses in the background")
                        .version("0.1")
                        .author("Cryptape"))
        .subcommand(SubCommand::with_name("stop").about("Stop all proccesses").version("0.1").author("Cryptape"))
        .subcommand(SubCommand::with_name("logrotate").about("rotate logs").version("0.1").author("Cryptape"))
        .subcommand(SubCommand::with_name("")
                        .about("Start all proccesses in the foreground")
                        .version("0.1")
                        .author("Cryptape"))
        .get_matches();

    let config = MonitorConfig::new("monitor.toml");
    let mut daemon: Processes = Processes::new(config);

    match matches.subcommand_name() {
        Some("start") => {
            match daemon.find_process() {
                Some(pid) => {
                    let name = daemon.processcfg.name.clone().unwrap();
                    warn!("{} already started,pid is {}", name, pid);
                    return;
                }
                None => {
                    daemon.start()
                }
            }
        }
        Some("stop") => {
            daemon.stop_all()
        }
        Some("logrotate") => {
            daemon.logrotate()
        }
        Some(&_) => {}
        None => {
            daemon.start_all();
            loop {
                thread::sleep(time::Duration::from_secs(1))
            }

        }
    }
}
