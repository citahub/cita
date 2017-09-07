use cache_2q::Cache;
use libproto::auth::*;
use util::H256;

#[derive(Debug)]
pub struct VerifyCache {
    inner: Cache,
}

impl VerifyCache {
    pub fn new(size: usize) -> Self {
        VerifyCache {inner: Cache::new(size)}
    }

    pub fn insert(&mut self, tx_hash: H256, resp: VerifyRespMsg) {
        self.inner.insert(tx_hash, resp);
    }

    pub fn get(&self, tx_hash: &H256) -> Option<&VerifyRespMsg> {
        self.inner.get(tx_hash)
    }
}


#[test]
fn basic() {
    let mut cache = VerifyCache::new(2);

    let hash1 = H256::random();
    let hash2 = H256::random();
    let hash3 = H256::random();

    let mut resp1 = VerifyRespMsg::new();
    resp1.set_tx_hash(hash1.to_vec());
    let mut resp2 = VerifyRespMsg::new();
    resp2.set_tx_hash(hash2.to_vec());
    let mut resp3 = VerifyRespMsg::new();
    resp3.set_tx_hash(hash3.to_vec());

    cache.insert(hash1.clone(), resp1.clone());
    cache.insert(hash2.clone(), resp2.clone());
    cache.insert(hash3.clone(), resp3.clone());

    assert_eq!(cache.get(&hash1), None);
    assert_eq!(cache.get(&hash2), Some(resp2));
    assert_eq!(cache.get(&hash3), Some(resp3));
}