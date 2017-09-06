extern crate protobuf;
use crypto::{PubKey, Signature, Sign, Error};
use libproto::auth::*;
use std::result::Result;
use util::H256;

pub struct Verifier {
    height: Option<u64>,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier { height: None }
    }

    pub fn set_height(&mut self, h: u64) {
        self.height = Some(h);

    }

    pub fn verify_sig(&self, req: &VerifyReqMsg) -> Result<PubKey, Error> {
        let hash = H256::from(req.get_hash());
        let sig = Signature::from(req.get_signature());
        match sig.recover(&hash) {
            Ok(pubkey) => Ok(pubkey),
            _ => Err(Error::InvalidSignature),
        }

    }


    pub fn verify_valid_until_block(&self, valid_until_block: u64) -> bool {
        if !self.height.is_none() && (valid_until_block == 0 || valid_until_block < self.height.unwrap()) {
            return true;
        } else {
            return false;
        }

    }
}
