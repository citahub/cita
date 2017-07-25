#![feature(test)]
extern crate tx_pool;
extern crate test;
extern crate libproto;
extern crate util;

use libproto::blockchain::Transaction;
use tx_pool::pool::*;
use test::Bencher;
use util::hash::H256;


#[bench]
fn bench_base(b: &mut Bencher) {
    let mut tx = Transaction::new();
    b.iter(|| {
        for i in 0..10000 {
            tx.set_content(format!("{}", i).as_bytes().to_vec());
        }
    });
}

#[bench]
fn bench_enqueue(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let hash: H256 = tx.sha3();

    b.iter(|| {
        for i in 0..10000 {
            tx.set_content(format!("{}", i).as_bytes().to_vec());
            p.enqueue(tx.clone(), hash);
        }
    });
}

#[bench]
fn bench_package(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let hash: H256 = tx.sha3();

    b.iter(|| {
        for i in 0..10000 {
            tx.set_content(format!("{}", i).as_bytes().to_vec());
            p.enqueue(tx.clone(), hash);
        }
        p.package(666);
    });
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut p = Pool::new(5000, 1000);
    let mut tx = Transaction::new();
    let hash: H256 = tx.sha3();

    b.iter(|| {
        for i in 0..10000 {
            tx.set_content(format!("{}", i).as_bytes().to_vec());
            p.enqueue(tx.clone(), hash);
        }
        let txs = p.package(666);
        p.update(&txs);
    });
}
