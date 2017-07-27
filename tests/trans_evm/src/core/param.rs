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

//extern crate serde;
//use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use serde_json;

#[derive(Serialize, Deserialize,Debug)]
pub struct Param{
    pub category: i32,
    pub ipandport: Vec<String>,
    pub txnum: i32,
    pub threads: i32,
    pub code: String,
}


impl Param{

    #[allow(dead_code,unused_variables)]
    pub fn load_from_file(path: &str) -> Self {

        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        serde_json::from_reader(fconfig).expect(concat!("json is invalid."))
    }
    
}

