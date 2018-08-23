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

extern crate cita_crypto as crypto;
extern crate util;

use crypto::{CreateKey, KeyPair, PubKey};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use util::hashable::Hashable;

fn to_hex_string(data: &[u8]) -> String {
    let strs: Vec<String> = data.into_iter().map(|a| format!("{:02x}", a)).collect();
    strs.join("")
}

fn write_to_file(path: String, data: &str, append: bool) {
    let mut file = if append {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap()
    } else {
        File::create(path).unwrap()
    };
    write!(&mut file, "{}", data).unwrap();
}

fn create_key(path: String) -> PubKey {
    let keypair = KeyPair::gen_keypair();
    let privkey = *keypair.privkey();
    let hex_str = to_hex_string(&privkey);
    let hex_str_with_0x = String::from("0x") + &hex_str + "\n";
    write_to_file(path, &hex_str_with_0x, false);
    *keypair.pubkey()
}

fn create_addr(path: String, pubkey: PubKey) {
    let hash = pubkey.crypt_hash();
    let addr = &hash.0[12..];
    let hex_str = to_hex_string(addr);
    let hex_str_with_0x = String::from("0x") + &hex_str + "\n";
    write_to_file(path, &hex_str_with_0x, true);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let pubkey = create_key(args.next().unwrap());
    create_addr(args.next().unwrap(), pubkey);
}
