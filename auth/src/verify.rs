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


use crypto::{PubKey, Signature, Sign, SIGNATURE_BYTES_LEN};
use libproto::*;
use libproto::blockchain::*;
use protobuf::Message;
use std::collections::{HashMap, HashSet};
use std::result::Result;
use std::sync::mpsc::Sender;
use util::{H256, BLOCKLIMIT};

#[derive(Debug, Clone)]
pub struct Verifier {
    inited: bool,
    height_latest: Option<u64>,
    height_low: Option<u64>,
    hashes: HashMap<u64, HashSet<H256>>,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier {
            inited: false,
            height_latest: None,
            height_low: None,
            hashes: HashMap::with_capacity(BLOCKLIMIT as usize),
        }
    }

    pub fn is_inited(&self) -> bool {
        self.inited
    }

    pub fn get_height_latest(&self) -> Option<u64> {
        self.height_latest
    }

    pub fn get_height_low(&self) -> Option<u64> {
        self.height_low
    }

    pub fn update_hashes(&mut self, h: u64, hashes: HashSet<H256>, tx_pub: &Sender<(String, Vec<u8>)>) {
        if self.height_latest.is_none() && self.height_low.is_none() {
            self.height_latest = Some(h);
            self.height_low = if h < BLOCKLIMIT { Some(0) } else { Some(h - BLOCKLIMIT + 1) };
            for i in self.height_low.unwrap()..h {
                let mut req = BlockTxHashesReq::new();
                req.set_height(i as u64);
                let msg = factory::create_msg(submodules::AUTH, topics::BLOCK_TXHASHES_REQ, communication::MsgType::BLOCK_TXHASHES_REQ, req.write_to_bytes().unwrap());
                tx_pub.send(("auth.blk_tx_hashs_req".to_string(), msg.write_to_bytes().unwrap())).unwrap();
            }
        } else {
            let current_height = self.height_latest.unwrap();
            let current_height_low = self.height_low.unwrap();
            if h > current_height {
                self.height_latest = Some(h);
                self.height_low = if h < BLOCKLIMIT { Some(0) } else { Some(h - BLOCKLIMIT + 1) };
                for i in current_height_low..self.height_low.unwrap() {
                    self.hashes.remove(&i);
                }
            }
            if h < self.height_low.unwrap() {
                return;
            }
        }
        trace!("update block's tx hashes for height:{} and the current low height:{} and latest height:{}", h, self.height_low.unwrap(), self.height_latest.unwrap());
        self.hashes.insert(h, hashes);
        if self.hashes.len() as u64 == (self.height_latest.unwrap() - self.height_low.unwrap() + 1) {
            self.inited = true;
        }
    }

    pub fn check_hash_exist(&self, hash: &H256) -> bool {
        if !self.inited {
            return true;
        }
        for (height, hashes) in self.hashes.iter() {
            if hashes.contains(hash) {
                trace!("Tx with hash {:?} has already existed in height:{}", hash.0, height);
                return true;
            }
        }
        return false;
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
            Crypto::SECP => {
                sig.recover(&hash).map_err(|_| ())
            }
            _ => {
                warn!("Unexpected crypto");
                Err(())
            }
        }
    }

    pub fn verify_valid_until_block(&self, valid_until_block: u64) -> bool {
        let mut result = false;
        if let Some(height) = self.height_latest {
            result = valid_until_block > height && valid_until_block <= (height + BLOCKLIMIT);
            if false == result {
                warn!("The new tx is out of time valid_until_block: {:?}, height: {:?}, BLOCKLIMIT: {:?}", valid_until_block, height, BLOCKLIMIT);
            }

        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
