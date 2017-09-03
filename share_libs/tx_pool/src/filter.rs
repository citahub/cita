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
    use crypto::{KeyPair, PrivKey, CreateKey};
    use libproto::blockchain::{SignedTransaction, Transaction};

    pub fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        tx.set_quota("a7c5ac471b47".to_string());

        tx.sign(*privkey)
    }

    #[test]
    fn basic() {
        let mut f = Filter::new(2);
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let tx1 = generate_tx(vec![1], 0, privkey);
        let tx2 = generate_tx(vec![1], 0, privkey);
        let tx3 = generate_tx(vec![2], 0, privkey);
        let tx4 = generate_tx(vec![3], 0, privkey);

        assert_eq!(f.check(tx1.crypt_hash()), true);
        assert_eq!(f.check(tx2.crypt_hash()), false);
        assert_eq!(f.check(tx3.crypt_hash()), true);
        assert_eq!(f.check(tx4.crypt_hash()), true);
        assert_eq!(f.check(tx2.crypt_hash()), true);
    }
}
