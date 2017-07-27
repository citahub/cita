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

#[derive(Clone,Debug)]
pub enum Methods{
    Sendtx(Trans),
    Height,
    Blockbyheiht(u64),
    Trans(String),
}

#[derive(Clone,Debug)]
pub enum Data{
    Sendtx(String),
    Height(String),
    Blockbyheiht(String),
    Trans(String),
}

#[derive(Clone,Debug)]
pub struct Jsontxdata{

    pub txdata: Data;

}

impl Jsontxdata{

    pub fn new(data: String) -> Self{

        Jsontxdata{
            txdata: Sendtx(data),
        }
    }

    pub fn generate_tx_data($self, method: Methods) -> Self{

        let txdata = match method{
            Methods::Sendtx(tx) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}",tx.write_to_bytes().unwrap().to_hex());
                Data::Sendtx(txdata);
            },
            Methods::Height => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_blockHeight\",\"params\":[],\"id\":2}}");
                Data::Height(txdata);
            },
            Methods::Blockbyheiht(h) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getBlockByHeight\",\"params\":[{},false],\"id\":2}}", h);
                Data::Blockbyheiht(txdata);
            },
            Methods::Trans(hash) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getTransaction\",\"params\":[\"{}\"],\"id\":2}}",hash);
                Data::Trans(txdata);
            },
        }
        Jsontxdata{
           txdata: Sendtx(txdata), 
        }
        //Self::new(txdata)
    }

}
