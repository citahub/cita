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
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate test;
extern crate tx_pool;
extern crate util;

use crypto::{CreateKey, KeyPair};
use libproto::blockchain::AccountGasLimit;
use libproto::blockchain::Transaction;
use std::collections::HashMap;
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
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_base",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_enqueue(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 2000*10000 <= account_gas_limit <= block_gas_limit
        tx.set_quota(2000);
        p.enqueue(tx.sign(*pv));
    }
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_enqueue",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_package(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();
    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 6000*10000 <= account_gas_limit <= block_gas_limit
        tx.set_quota(6000);
        p.enqueue(tx.sign(*pv));
    }
    let mut account_gas_limit = AccountGasLimit::new();
    // set block_gas_limit default
    let block_gas_limit = 61415926;
    // height should less than valid_until_block
    let height = 0;
    // set account_gas_limit be equal as block_gas_limit
    account_gas_limit.set_common_gas_limit(block_gas_limit);
    account_gas_limit.set_specific_gas_limit(HashMap::new());

    p.package(height, block_gas_limit, account_gas_limit.clone());
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_package",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let start = SystemTime::now();
    let mut p = Pool::new(1000);
    let mut tx = Transaction::new();
    let keypair = KeyPair::gen_keypair();
    let pv = keypair.privkey();

    for i in 0..10000 {
        tx.set_data(format!("{}", i).as_bytes().to_vec());
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(99);
        // 6000*10000 <= account_gas_limit <= block_gas_limit
        tx.set_quota(6000);
        p.enqueue(tx.sign(*pv));
    }
    let mut account_gas_limit = AccountGasLimit::new();
    // set block_gas_limit default
    let block_gas_limit = 61415926;
    // height should less than valid_until_block
    let height = 0;
    // set account_gas_limit be equal as block_gas_limit
    account_gas_limit.set_common_gas_limit(block_gas_limit);
    account_gas_limit.set_specific_gas_limit(HashMap::new());

    let txs = p.package(height, block_gas_limit, account_gas_limit.clone());
    p.update(&txs);
    let sys_time = SystemTime::now();
    let diff = sys_time
        .duration_since(start)
        .expect("SystemTime::duration_since failed");
    println!("pass");
    println!(
        "test {:20} ... bench: {}.{} s/iter",
        "bench_update",
        diff.as_secs(),
        diff.subsec_nanos()
    );
    b.iter(|| {});
}
