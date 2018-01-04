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

use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;
    // Read the JSON contents of the file as an instance of `User`.
    let u = from_reader(file)?;

    // Return the `User`.
    Ok(u)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub max_connections: usize,
    pub servers: Vec<String>,
    pub param: Param,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
    pub number: usize,
    pub tx_param: BenchTxParam,
    pub peer_param: BenchPeerParam,
}

impl Default for Param {
    fn default() -> Self {
        Param {
            number: 0,
            tx_param: BenchTxParam::default(),
            peer_param: BenchPeerParam::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchTxParam {
    pub enable: bool,
    pub codes: Vec<String>,
    pub quota: u64,
    pub check_block_break: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchPeerParam {
    pub enable: bool,
}

impl Default for BenchPeerParam {
    fn default() -> Self {
        BenchPeerParam { enable: false }
    }
}

impl Default for BenchTxParam {
    fn default() -> Self {
        BenchTxParam {
            enable: true,
            codes: vec![
                "60606040523415600e57600080fd5b5b5b5b60948061001f6000396000f300\
                 60606040526000357c01000000000000000000000000000000000000000000\
                 00000000000000900463ffffffff1680635524107714603d575b600080fd5b\
                 3415604757600080fd5b605b6004808035906020019091905050605d565b00\
                 5b806000819055505b505600a165627a7a72305820c471b4376626da2540b2\
                 374e8b4110501051c426ff46814a6170ce9e219e49a80029"
                    .to_string(),
            ],
            quota: 2500,
            check_block_break: 30,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_connections: 1,
            servers: vec!["ws://127.0.0.1:4337".to_string()],
            param: Default::default(),
        }
    }
}
