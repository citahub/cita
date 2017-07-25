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
