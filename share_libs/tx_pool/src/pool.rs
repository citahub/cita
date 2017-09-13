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

use filter::Filter;
use libproto::blockchain::SignedTransaction;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use util::H256;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Strategy {
    FIFO,
    PRIORITY,
    VIP,
}

#[derive(Clone, Debug)]
struct TxOrder {
    hash: H256,
    order: u64,
}

impl TxOrder {
    fn new(hash: H256, order: u64) -> Self {
        TxOrder { hash: hash, order: order }
    }
}

impl Eq for TxOrder {}
impl PartialEq for TxOrder {
    fn eq(&self, other: &TxOrder) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl PartialOrd for TxOrder {
    fn partial_cmp(&self, other: &TxOrder) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TxOrder {
    fn cmp(&self, b: &TxOrder) -> Ordering {
        self.order.cmp(&b.order)
    }
}

#[derive(Debug)]
pub struct Pool {
    package_limit: usize,
    filter: Filter,
    order_set: BTreeSet<TxOrder>,
    txs: HashMap<H256, SignedTransaction>,
    strategy: Strategy,
    order: u64,
}

impl Pool {
    pub fn new(capacity: usize, package_limit: usize) -> Self {
        Pool {
            package_limit: package_limit,
            filter: Filter::new(capacity),
            order_set: BTreeSet::new(),
            txs: HashMap::new(),
            strategy: Strategy::FIFO,
            order: 0,
        }
    }

    pub fn new_with_strategy(capacity: usize, package_limit: usize, strategy: Strategy) -> Self {
        Pool {
            package_limit: package_limit,
            filter: Filter::new(capacity),
            order_set: BTreeSet::new(),
            txs: HashMap::new(),
            strategy: strategy,
            order: 0,
        }
    }

    fn get_order(&mut self) -> u64 {
        let order = self.order;
        let (new_order, _) = order.overflowing_add(1);
        self.order = new_order;
        order
    }

    #[allow(unused_variables)]
    fn get_order_by_priority(&mut self, tx: &SignedTransaction) -> u64 {
        return self.get_order();
    }

    #[allow(unused_variables)]
    fn get_order_by_vip(&mut self, tx: &SignedTransaction) -> u64 {
        return self.get_order();
    }

    pub fn enqueue(&mut self, tx: SignedTransaction) -> bool {
        let hash = H256::from_slice(tx.get_tx_hash());

        let is_ok = self.filter.check(hash);
        if is_ok {
            let order = match self.strategy {
                Strategy::FIFO => self.get_order(),
                Strategy::PRIORITY => self.get_order_by_priority(&tx),
                Strategy::VIP => self.get_order_by_vip(&tx),
            };
            let tx_order = TxOrder::new(hash, order);
            self.order_set.insert(tx_order);
            self.txs.insert(hash, tx);
        }
        is_ok
    }

    fn update_order_set(&mut self, hash_list: &[H256]) {
        self.order_set = self.order_set.iter().cloned().filter(|order| !hash_list.contains(&order.hash)).collect();
    }

    pub fn update(&mut self, txs: &[SignedTransaction]) {
        let mut hash_list = Vec::new();
        for tx in txs {
            let hash = tx.crypt_hash();
            self.txs.remove(&hash);
            hash_list.push(hash);
        }
        self.update_order_set(&hash_list);
    }

    pub fn package(&mut self, height: u64, block_gas_limit: u64) -> Vec<SignedTransaction> {
        let mut tx_list = Vec::new();
        let mut invalid_tx_list = Vec::new();
        let mut n = block_gas_limit;

        {
            let mut iter = self.order_set.iter();
            loop {
                let order = iter.next();
                if order.is_none() {
                    break;
                }
                let hash = order.unwrap().hash;
                let tx = self.txs.get(&hash);
                if let Some(tx) = tx {
                    if tx.get_transaction_with_sig().get_transaction().valid_until_block == 0 || tx.get_transaction_with_sig().get_transaction().valid_until_block >= height {
                        let quota = tx.get_transaction_with_sig().get_transaction().quota;
                        if n <= quota {
                            if tx_list.is_empty() {
                                tx_list.push(tx.clone());
                            }
                            break;
                        }
                        n = n - quota;
                        tx_list.push(tx.clone());
                    } else {
                        invalid_tx_list.push(tx.clone());
                    }
                } else {
                    panic!("invalid tx order {:?}", order);
                }
            }
        }

        self.update(&invalid_tx_list);
        tx_list
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }
}

//FIXME
#[cfg(test)]
mod tests {
    use super::*;
    use crypto::{KeyPair, PrivKey, CreateKey};
    use libproto::blockchain::{SignedTransaction, Transaction};

    pub fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        tx.set_quota(20);

        tx.sign(*privkey)
    }

    #[test]
    fn basic() {
        let mut p = Pool::new(2, 1);
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let tx1 = generate_tx(vec![1], 999, privkey);
        let tx2 = generate_tx(vec![1], 999, privkey);
        let tx3 = generate_tx(vec![2], 999, privkey);
        let tx4 = generate_tx(vec![3], 5, privkey);

        assert_eq!(p.enqueue(tx1.clone()), true);
        assert_eq!(p.enqueue(tx2.clone()), false);
        assert_eq!(p.enqueue(tx3.clone()), true);
        assert_eq!(p.enqueue(tx4.clone()), true);
        assert_eq!(p.len(), 3);
        p.update(&vec![tx1.clone()]);
        assert_eq!(p.len(), 2);
        assert_eq!(p.package(5, 30), vec![tx3.clone()]);
        p.update(&vec![tx3.clone()]);
        assert_eq!(p.package(5, 30), vec![tx4]);
        assert_eq!(p.len(), 1);
        assert_eq!(p.package(6, 30), vec![]);
        assert_eq!(p.len(), 0);
    }
}
