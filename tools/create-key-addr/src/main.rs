// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cita_crypto as crypto;

use crate::crypto::{CreateKey, KeyPair, PubKey};
use hashable::Hashable;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn to_hex_string(data: &[u8]) -> String {
    let strs: Vec<String> = data.iter().map(|a| format!("{:02x}", a)).collect();
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
