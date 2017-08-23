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

use lru_cache::LruCache;
use util::H256;

#[derive(Debug)]
pub struct Filter {
    inner: LruCache<H256, u32>,
}

impl Filter {
    pub fn new(capacity: usize) -> Self {
        Filter { inner: LruCache::new(capacity) }
    }

    pub fn check(&mut self, hash: H256) -> bool {
        let is_ok = !self.inner.contains_key(&hash);
        if is_ok {
            self.inner.insert(hash, 0);
        }
        is_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::H512;
    use libproto::blockchain::{SignedTransaction, UnverifiedTransaction, Transaction};

    pub fn generate_tx(data: Vec<u8>, valid_until_block: u64) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);

        let pv = H512::from_slice(&[20,17]);

        let mut uv_tx = UnverifiedTransaction::new();
        uv_tx.set_transaction(tx);

        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_transaction_with_sig(uv_tx);
        signed_tx.sign(pv);

        signed_tx
    }

    #[test]
    fn basic() {
        let mut f = Filter::new(2);
        let tx1 = generate_tx(vec![1], 0);
        let tx2 = generate_tx(vec![1], 0);
        let tx3 = generate_tx(vec![2], 0);
        let tx4 = generate_tx(vec![3], 0);

        assert_eq!(f.check(tx1.crypt_hash()), true);
        assert_eq!(f.check(tx2.crypt_hash()), false);
        assert_eq!(f.check(tx3.crypt_hash()), true);
        assert_eq!(f.check(tx4.crypt_hash()), true);
        assert_eq!(f.check(tx2.crypt_hash()), true);
    }
}
