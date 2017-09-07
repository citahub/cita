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


#![cfg_attr(test, feature(test))]
extern crate tx_pool;
extern crate test;
extern crate libproto;
extern crate util;
extern crate cita_crypto as crypto;

use crypto::{KeyPair, CreateKey};
use libproto::blockchain::{Transaction, UnverifiedTransaction, SignedTransaction};
use std::time::SystemTime;
use test::Bencher;
use tx_pool::pool::*;

#[bench]
fn bench_base(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut tx = Transaction::new();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
    }
    let sys_time = SystemTime::now();
    let diff = sys_time.duration_since(start).expect("SystemTime::duration_since failed");
    println!{"time: {:?}", diff};
    b.iter(|| {});
}

#[bench]
fn bench_enqueue(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);
        p.enqueue(tx.sign(*pv));
    }
    let sys_time = SystemTime::now();
    let diff = sys_time.duration_since(start).expect("SystemTime::duration_since failed");
    println!{"time: {:?}", diff};
    b.iter(|| {});
}

#[bench]
fn bench_package(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(9999999999);
        p.enqueue(tx.sign(*pv));
    }
    p.package(666);
    let sys_time = SystemTime::now();
    let diff = sys_time.duration_since(start).expect("SystemTime::duration_since failed");
    println!{"time: {:?}", diff};
    b.iter(|| {});
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();

    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);
        p.enqueue(tx.sign(*pv));
    }
    let txs = p.package(666);
    p.update(&txs);
    let sys_time = SystemTime::now();
    let diff = sys_time.duration_since(start).expect("SystemTime::duration_since failed");
    println!{"time: {:?}", diff};
    b.iter(|| {});
}
