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

use clap::{App, Arg, ArgMatches};
use log::LogLevelFilter;
use std::fmt;

use std::str;

use cita_log;

const APPNAME: &str = "CITA Send Trans";
const VERNUM: &str = "0.0.1";

#[derive(Debug, Clone)]
pub struct Node {
    pub host: String,
    pub port: u16,
    pub key: String,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl str::FromStr for Node {
    type Err = ParseNodeError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(":").collect::<Vec<&str>>();
        if v.len() != 3 {
            return Err(ParseNodeError { address: s.to_string() });
        }
        let h = v[0].trim();
        let p = v[1].parse::<u16>();
        let k = v[2].trim();
        if h.is_empty() || p.is_err() || k.len() != 64 {
            return Err(ParseNodeError { address: s.to_string() });
        }
        Ok(Node {
               host: h.to_string(),
               port: p.unwrap(),
               key: k.to_string(),
           })
    }
}

pub struct ParseNodeError {
    address: String,
}

impl fmt::Display for ParseNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (&format!("the address [{}] for node is malformed", &self.address)).fmt(f)
    }
}

pub struct AppConfig {
    pub node: Vec<Node>,
    pub protocol: String,
    pub thread: usize,
    pub amount: usize,
    pub interval: usize,
    pub category: String,
}

impl<'a> From<&'a ArgMatches<'a>> for AppConfig {
    fn from(matches: &'a ArgMatches) -> Self {
        let node = values_t!(matches, "node", Node).unwrap();
        let protocol = value_t!(matches, "protocol", String).unwrap();
        let thread = value_t!(matches, "thread", usize).unwrap();
        let amount = value_t!(matches, "amount", usize).unwrap();
        let interval = value_t!(matches, "interval", usize).unwrap();
        let category = value_t!(matches, "category", String).unwrap();
        AppConfig {
            node: node,
            protocol: protocol,
            thread: thread,
            amount: amount,
            interval: interval,
            category: category,
        }
    }
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        ret.push_str(&format!("\nAppConfig: {{\n"));
        ret.push_str(&format!("    node[{}]:\n", self.node.len()));
        for node in self.node.iter() {
            ret.push_str(&format!("        {}\n", node));
        }
        ret.push_str(&format!("    thread: {}\n", self.thread));
        ret.push_str(&format!("    amount: {}\n", self.amount));
        ret.push_str(&format!("    interval: {}\n", self.interval));
        ret.push_str(&format!("    category: {}\n", self.category));
        ret.push_str(&format!("}}\n"));
        write!(f, "{}", ret)
    }
}

pub fn build_commandline<'a, 'b>() -> App<'a, 'a> {
    App::new(APPNAME)
        .version(VERNUM)
        .author("Boyu Yang <yangby@cryptape.com>")
        .about("Send transactions automaticlly.")
        .arg(Arg::with_name("quiet")
                 .short("q")
                 .conflicts_with("verbose")
                 .help("No output printed to stdout."))
        .arg(Arg::with_name("verbose")
                 .short("v")
                 .multiple(true)
                 .help("Use verbose [Warn] output (support -vv [Info] / -vvv [Debug] / -vvvv.. [Trace])."))
        .arg(Arg::with_name("node")
                 .long("node")
                 .short("N")
                 .required(true)
                 .takes_value(true)
                 .multiple(true)
                 .value_delimiter(",")
                 .help("Set the host:port:key[host:port:key,[...]] of nodes to send transactions."))
        .arg(Arg::with_name("protocol")
                 .long("protocol")
                 .short("p")
                 .takes_value(true)
                 .possible_value("http")
                 .possible_value("https")
                 .default_value("http")
                 .help("Set the protocol."))
        .arg(Arg::with_name("thread")
                 .long("thread")
                 .short("t")
                 .takes_value(true)
                 .default_value("1")
                 .help("Set the number of threads for each node."))
        .arg(Arg::with_name("amount")
                 .long("amount")
                 .short("a")
                 .takes_value(true)
                 .default_value("1")
                 .help("Set the amount of messages for each node. 0 means infinite."))
        .arg(Arg::with_name("interval")
                 .long("interval")
                 .short("i")
                 .takes_value(true)
                 .default_value("1000")
                 .help("Wait interval millisecond between sending each request. 0 means no wait."))
        .arg(Arg::with_name("category")
                 .long("category")
                 .short("c")
                 .takes_value(true)
                 .possible_value("create-account")
                 .possible_value("receive")
                 .possible_value("pay")
                 .possible_value("transfer")
                 .possible_value("arrival")
                 .default_value("create-account")
                 .help("[Unfinished!] Set the category of messages to send."))
}

pub fn parse_arguments(matches: ArgMatches) -> AppConfig {
    debug!("matches = {:?}", matches);
    let log_lv = if matches.is_present("quiet") {
        LogLevelFilter::Off
    } else {
        match matches.occurrences_of("verbose") {
            0 => LogLevelFilter::Error,
            1 => LogLevelFilter::Warn,
            2 => LogLevelFilter::Info,
            3 => LogLevelFilter::Debug,
            4 | _ => LogLevelFilter::Trace,
        }
    };
    cita_log::format(log_lv);
    AppConfig::from(&matches)
}
