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

extern crate time;
extern crate log;
extern crate env_logger;

use env_logger::LogBuilder;
use log::{LogLevelFilter, LogRecord};
use std::env;

pub fn format(level: LogLevelFilter) {
    let format = |record: &LogRecord| {
        let t = time::now();
        format!("{},{:03} - {} - {}", time::strftime("%Y-%m-%d %H:%M:%S", &t).unwrap(), t.tm_nsec / 1000_000, record.level(), record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, level);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}
