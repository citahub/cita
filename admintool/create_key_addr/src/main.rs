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
        OpenOptions::new().create(true).append(true).open(path).unwrap()
    } else {
        File::create(path).unwrap()
    };
    write!(&mut file, "{}", data).unwrap();
}

fn create_key(path: String) -> PubKey {
    let keypair = KeyPair::gen_keypair();
    let privkey = keypair.privkey().clone();
    let hex_str = to_hex_string(&privkey);
    write_to_file(path, &hex_str, false);
    keypair.pubkey().clone()
}

fn create_addr(path: String, pubkey: PubKey) {
    let hash = pubkey.crypt_hash();
    let addr = &hash.0[12..];
    let str = to_hex_string(addr);
    let hex_str = String::from("0x") + &str + "\n";
    write_to_file(path, &hex_str, true);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let pubkey = create_key(args.next().unwrap());
    create_addr(args.next().unwrap(), pubkey);
}
