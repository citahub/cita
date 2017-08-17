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


#![feature(test)]
extern crate tx_pool;
extern crate test;
extern crate libproto;
extern crate util;

use libproto::blockchain::{Transaction, UnverifiedTransaction, SignedTransaction};
use test::Bencher;
use tx_pool::pool::*;
use util::H256;


#[bench]
fn bench_base(b: &mut Bencher) {
    let mut tx = Transaction::new();
    b.iter(|| for i in 0..10000 {
               tx.set_data(format!("{}", i).as_bytes().to_vec());
           });
}

#[bench]
fn bench_enqueue(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let mut uv_tx = UnverifiedTransaction::new();
    let mut signed_tx = SignedTransaction::new();
    let pv = H256::from_slice(&[20,17]);
    b.iter(|| for i in 0..10000 {
               tx.set_data(format!("{}", i).as_bytes().to_vec());
               uv_tx.set_transaction(tx.clone());
               signed_tx.set_transaction_with_sig(uv_tx.clone());
               signed_tx.sign(pv);
               p.enqueue(signed_tx.clone());
           });
}

#[bench]
fn bench_package(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let mut uv_tx = UnverifiedTransaction::new();
    let mut signed_tx = SignedTransaction::new();
    let pv = H256::from_slice(&[20,17]);
    b.iter(|| {
               for i in 0..10000 {
                   tx.set_data(format!("{}", i).as_bytes().to_vec());
                   uv_tx.set_transaction(tx.clone());
                   signed_tx.set_transaction_with_sig(uv_tx.clone());
                   signed_tx.sign(pv);
                   p.enqueue(signed_tx.clone());
               }
               p.package(666);
           });
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let mut uv_tx = UnverifiedTransaction::new();
    let mut signed_tx = SignedTransaction::new();
    let pv = H256::from_slice(&[20,17]);

    b.iter(|| {
               for i in 0..10000 {
                   tx.set_data(format!("{}", i).as_bytes().to_vec());
                   uv_tx.set_transaction(tx.clone());
                   signed_tx.set_transaction_with_sig(uv_tx.clone());
                   signed_tx.sign(pv);
                   p.enqueue(signed_tx.clone());
               }
               let txs = p.package(666);
               p.update(&txs);
           });
}

