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

use cache_2q::Cache;
use libproto::auth::*;
use util::H256;

#[derive(Debug)]
pub struct VerifyCache {
    inner: Cache<H256, VerifyTxResp>,
}

impl VerifyCache {
    pub fn new(size: usize) -> Self {
        // size x4  because cache_2q
        VerifyCache { inner: Cache::new(size * 4) }
    }

    pub fn insert(&mut self, tx_hash: H256, resp: VerifyTxResp) {
        self.inner.insert(tx_hash, resp);
    }

    pub fn get(&self, tx_hash: &H256) -> Option<&VerifyTxResp> {
        self.inner.peek(tx_hash)
    }

    pub fn remove(&mut self, tx_hash: &H256) -> Option<VerifyTxResp> {
        self.inner.remove(tx_hash)
    }

}

#[derive(Debug, PartialEq)]
pub enum VerifyResult {
    VerifyOngoing,
    VerifyFailed,
    VerifySucceeded,
}

#[derive(Debug)]
pub struct BlockVerifyStatus {
    pub block_verify_result: VerifyResult,
    pub verify_success_cnt_required: usize,
    pub verify_success_cnt_capture: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockVerifyId {
    pub request_id: u64,
    pub sub_module: u32,
}


#[derive(Debug)]
pub struct VerifyBlockCache {
    inner: Cache<BlockVerifyId, BlockVerifyStatus>,
}

impl VerifyBlockCache {
    pub fn new(size: usize) -> Self {
        VerifyBlockCache { inner: Cache::new(size * 4) }
    }

    pub fn insert(&mut self, block_verify_id: BlockVerifyId, result: BlockVerifyStatus) {
        self.inner.insert(block_verify_id, result);
    }

    pub fn get(&self, block_verify_id: &BlockVerifyId) -> Option<&BlockVerifyStatus> {
        self.inner.peek(block_verify_id)
    }

    pub fn get_mut(&mut self, block_verify_id: &BlockVerifyId) -> Option<&mut BlockVerifyStatus> {
        (&mut self.inner).get_mut(block_verify_id)
    }
}


#[test]
fn basic() {
    let mut cache = VerifyCache::new(2);

    let hash1 = H256::from_slice(&vec![1]);
    let hash2 = H256::from_slice(&vec![2]);
    let hash3 = H256::from_slice(&vec![3]);

    let mut resp1 = VerifyTxResp::new();
    resp1.set_tx_hash(hash1.to_vec());
    let mut resp2 = VerifyTxResp::new();
    resp2.set_tx_hash(hash2.to_vec());
    let mut resp3 = VerifyTxResp::new();
    resp3.set_tx_hash(hash3.to_vec());

    cache.insert(hash1.clone(), resp1.clone());
    cache.insert(hash2.clone(), resp2.clone());
    cache.insert(hash3.clone(), resp3.clone());

    assert_eq!(cache.get(&hash1), None);
    assert_eq!(cache.get(&hash2), Some(&resp2));
    assert_eq!(cache.get(&hash3), Some(&resp3));
}
