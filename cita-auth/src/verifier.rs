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

use cita_types::{Address, H256};
use cita_types::traits::LowerHex;
use crypto::{pubkey_to_address, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use libproto::{BlockTxHashesReq, Crypto, Message, Ret, UnverifiedTransaction, VerifyBlockReq, VerifyTxReq,
               VerifyTxResp};
use libproto::blockchain::AccountGasLimit;
use libproto::router::{MsgType, RoutingKey, SubModules};
use std::collections::{HashMap, HashSet};
use std::convert::{Into, TryInto};
use std::result::Result;
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use util::BLOCKLIMIT;

#[derive(Debug, Clone)]
pub enum VerifyRequestID {
    SingleVerifyRequestID(Vec<u8>),
    BlockVerifyRequestID(u64),
}

#[derive(Debug, Clone)]
pub enum VerifyRequestResponse {
    AuthRequest(VerifyTxReq),
    AuthResponse(VerifyTxResp),
}

#[derive(Debug, Clone)]
pub struct VerifyRequestResponseInfo {
    pub key: String,
    pub verify_type: VerifyType,
    pub request_id: VerifyRequestID,
    pub time_stamp: SystemTime,
    pub req_resp: VerifyRequestResponse,
    pub un_tx: Option<UnverifiedTransaction>,
}

#[derive(Debug, PartialEq)]
pub enum VerifyResult {
    VerifyNotBegin,
    VerifyOngoing,
    VerifyFailed,
    VerifySucceeded,
}

#[derive(Debug)]
pub struct BlockVerifyStatus {
    pub request_id: u64,
    pub block_verify_result: VerifyResult,
    pub verify_success_cnt_required: usize,
    pub verify_success_cnt_capture: usize,
    pub cache_hit: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerifyType {
    SingleVerify,
    BlockVerify,
}

#[derive(Debug, Clone)]
pub struct Verifier {
    inited: bool,
    height_latest: Option<u64>,
    height_low: Option<u64>,
    hashes: HashMap<u64, HashSet<H256>>,
    chain_id: Option<u32>,

    check_quota: bool,
    block_gas_limit: u64,
    account_gas_limit: AccountGasLimit,
}

impl Default for Verifier {
    fn default() -> Verifier {
        Verifier {
            inited: false,
            height_latest: None,
            height_low: None,
            hashes: HashMap::with_capacity(BLOCKLIMIT as usize),
            chain_id: None,
            check_quota: false,
            block_gas_limit: 0,
            account_gas_limit: AccountGasLimit::new(),
        }
    }
}

impl Verifier {
    pub fn new() -> Self {
        Verifier::default()
    }

    pub fn is_inited(&self) -> bool {
        self.inited
    }

    pub fn set_chain_id(&mut self, chain_id: u32) {
        self.chain_id = Some(chain_id)
    }

    pub fn get_chain_id(&self) -> Option<u32> {
        self.chain_id
    }

    pub fn get_height_latest(&self) -> Option<u64> {
        self.height_latest
    }

    pub fn get_height_low(&self) -> Option<u64> {
        self.height_low
    }

    pub fn set_quota_check_info(
        &mut self,
        check_quota: bool,
        block_gas_limit: u64,
        account_gas_limit: AccountGasLimit,
    ) {
        self.check_quota = check_quota;
        self.block_gas_limit = block_gas_limit;
        self.account_gas_limit = account_gas_limit;
    }

    pub fn send_txhashes_req(low: u64, high: u64, tx_pub: &Sender<(String, Vec<u8>)>) {
        for i in low..high {
            let mut req = BlockTxHashesReq::new();
            req.set_height(i);
            let msg: Message = req.into();
            tx_pub
                .send((
                    routing_key!(Auth >> BlockTxHashesReq).into(),
                    msg.try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    pub fn update_hashes(&mut self, h: u64, hashes: HashSet<H256>, tx_pub: &Sender<(String, Vec<u8>)>) {
        if self.height_latest.is_none() && self.height_low.is_none() {
            self.height_latest = Some(h);
            self.height_low = if h < BLOCKLIMIT {
                Some(0)
            } else {
                Some(h - BLOCKLIMIT + 1)
            };
            Verifier::send_txhashes_req(self.height_low.unwrap(), h, tx_pub);
        } else {
            let current_height = self.height_latest.unwrap();
            let current_height_low = self.height_low.unwrap();
            if h == current_height + 1 {
                self.height_latest = Some(h);
                self.height_low = if h < BLOCKLIMIT {
                    Some(0)
                } else {
                    Some(h - BLOCKLIMIT + 1)
                };
                for i in current_height_low..self.height_low.unwrap() {
                    self.hashes.remove(&i);
                }
            } else if h > current_height + 1 {
                /*if we lost some height blockhashs
                 we notify chain to re-trans txs*/
                Verifier::send_txhashes_req(current_height + 1, h + 1, tx_pub);
                return;
            }
            if h < self.height_low.unwrap() {
                return;
            }
        }
        trace!(
            "update block's tx hashes for height:{} and the current low height:{} and latest height:{}",
            h,
            self.height_low.unwrap(),
            self.height_latest.unwrap()
        );
        self.hashes.insert(h, hashes);
        if self.hashes.len() as u64 == (self.height_latest.unwrap() - self.height_low.unwrap() + 1) {
            self.inited = true;
        }
    }

    pub fn check_hash_exist(&self, hash: &H256) -> bool {
        if !self.inited {
            return true;
        }
        for (height, hashes) in &self.hashes {
            if hashes.contains(hash) {
                trace!(
                    "Tx with hash {:?} has already existed in height:{}",
                    hash.0,
                    height
                );
                return true;
            }
        }
        false
    }

    pub fn verify_sig(&self, req: &VerifyTxReq) -> Result<PubKey, ()> {
        let hash = H256::from(req.get_hash());
        let sig_bytes = req.get_signature();
        if sig_bytes.len() != SIGNATURE_BYTES_LEN {
            warn!("Unvalid signature bytes");
            return Err(());
        }
        let sig = Signature::from(sig_bytes);
        match req.get_crypto() {
            Crypto::SECP => sig.recover(&hash).map_err(|_| ()),
            _ => {
                warn!("Unexpected crypto");
                Err(())
            }
        }
    }

    pub fn verify_tx(&self, req: &VerifyTxReq) -> VerifyTxResp {
        let mut resp = VerifyTxResp::new();
        resp.set_tx_hash(req.get_tx_hash().to_vec());

        if req.get_nonce().len() > 128 {
            resp.set_ret(Ret::InvalidNonce);
            return resp;
        }

        if !self.chain_id.is_some() {
            resp.set_ret(Ret::NotReady);
            return resp;
        }
        let ret = self.verify_sig(req);
        if ret.is_err() {
            resp.set_ret(Ret::BadSig);
            return resp;
        }

        //check signer if req have
        let req_signer = req.get_signer();
        if !req_signer.is_empty() && req_signer != ret.unwrap().to_vec().as_slice() {
            resp.set_ret(Ret::BadSig);
            return resp;
        }

        // check the correctness of chainID
        if req.get_chain_id() != self.chain_id.unwrap() {
            resp.set_ret(Ret::BadChainId);
            return resp;
        }

        resp.set_signer(ret.unwrap().to_vec());
        resp.set_ret(Ret::OK);
        trace!(
            "verfiy_tx's result:tx_hash={:?}, ret={:?}, signer={:?}",
            resp.get_tx_hash(),
            resp.get_ret(),
            resp.get_signer()
        );
        resp
    }

    pub fn verify_valid_until_block(&self, valid_until_block: u64) -> bool {
        let mut result = false;
        if let Some(height) = self.height_latest {
            result = valid_until_block > height && valid_until_block <= (height + BLOCKLIMIT);
            if !result {
                warn!(
                    "The new tx is out of time valid_until_block: {:?}, height: {:?}, BLOCKLIMIT: {:?}",
                    valid_until_block, height, BLOCKLIMIT
                );
            }
        }
        result
    }

    pub fn verify_quota(&self, blkreq: &VerifyBlockReq) -> bool {
        let reqs = blkreq.get_reqs();
        let len = reqs.len();
        let mut gas_limit = self.account_gas_limit.get_common_gas_limit();
        let mut specific_gas_limit = self.account_gas_limit.get_specific_gas_limit().clone();
        let mut account_gas_used: HashMap<Address, u64> = HashMap::new();
        let mut n = self.block_gas_limit;
        for req in reqs {
            let quota = req.get_quota();
            let signer = pubkey_to_address(&PubKey::from(req.get_signer()));

            if n < quota {
                if len == 1 {
                    return true;
                }
                return false;
            }

            if self.check_quota {
                if account_gas_used.contains_key(&signer) {
                    if let Some(value) = account_gas_used.get_mut(&signer) {
                        if *value < quota {
                            return false;
                        } else {
                            *value = *value - quota;
                        }
                    }
                } else {
                    if let Some(value) = specific_gas_limit.remove(&signer.lower_hex()) {
                        gas_limit = value;
                    }
                    let mut _remainder = 0;
                    if quota < gas_limit {
                        _remainder = gas_limit - quota;
                    } else {
                        _remainder = 0;
                    }
                    account_gas_used.insert(Address::from(signer), _remainder);
                }
            }
            n = n - quota;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Verifier;
    use std::collections::HashSet;
    use std::sync::mpsc::channel;

    #[test]
    fn verify_init() {
        let mut v = Verifier::new();
        assert_eq!(v.is_inited(), false);
        let (tx_pub, _) = channel();
        v.update_hashes(0, HashSet::new(), &tx_pub);
        assert_eq!(v.is_inited(), true);
        assert_eq!(v.get_height_latest(), Some(0));
        assert_eq!(v.get_height_low(), Some(0));
    }

    #[test]
    fn verify_update() {
        let mut v = Verifier::new();
        let (tx_pub, _rx_pub) = channel();
        v.update_hashes(100, HashSet::new(), &tx_pub);
        assert_eq!(v.is_inited(), false);
        assert_eq!(v.get_height_latest(), Some(100));
        assert_eq!(v.get_height_low(), Some(1));
        for i in 0..99 {
            v.update_hashes(i, HashSet::new(), &tx_pub);
        }
        assert_eq!(v.is_inited(), false);
        v.update_hashes(99, HashSet::new(), &tx_pub);
        assert_eq!(v.is_inited(), true);

        v.update_hashes(101, HashSet::new(), &tx_pub);
        assert_eq!(v.get_height_latest(), Some(101));
        assert_eq!(v.get_height_low(), Some(2));
    }
}
