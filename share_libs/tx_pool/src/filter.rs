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
use util::hash::H256;

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
    use libproto::blockchain::Transaction;
    #[test]
    fn basic() {
        let mut f = Filter::new(2);
        let mut tx1 = Transaction::new();
        tx1.set_content(vec![1]);
        let mut tx2 = Transaction::new();
        tx2.set_content(vec![1]);
        let mut tx3 = Transaction::new();
        tx3.set_content(vec![2]);
        let mut tx4 = Transaction::new();
        tx4.set_content(vec![3]);

        assert_eq!(f.check(tx1.sha3()), true);
        assert_eq!(f.check(tx2.sha3()), false);
        assert_eq!(f.check(tx3.sha3()), true);
        assert_eq!(f.check(tx4.sha3()), true);
        assert_eq!(f.check(tx2.sha3()), true);
    }
}
