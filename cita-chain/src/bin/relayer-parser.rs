// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
extern crate clap;
extern crate core;
extern crate jsonrpc_types;
#[macro_use]
extern crate log;
extern crate logger;
extern crate serde_json;

use core::libchain::chain::TxProof;

fn main() {
    logger::init();

    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Cryptape Technologies")
        (about: "CITA Relay Info Parser by Rust")
        (@arg TxProofHexString: -h --hex_string +takes_value +required "Input a hex string of TxProof.")
    ).get_matches();

    let hexstr = matches.value_of("TxProofHexString").unwrap();
    trace!("The input hex string is {}.", hexstr);
    let hexstr = if hexstr.starts_with("0x") {
        &hexstr[2..]
    } else {
        hexstr
    };
    if let Some(tx_proof) = TxProof::from_hexstr(hexstr) {
        trace!("The input tx_proof is {:?}.", tx_proof);
        if let Some(relay_info) = tx_proof.extract_relay_info() {
            let json_str = serde_json::to_string(&relay_info).unwrap();
            println!("{}", json_str);
        } else {
            error!("Failed to parse RelayInfo.");
        };
    } else {
        error!("Failed to parse TxProof.");
    };
}
