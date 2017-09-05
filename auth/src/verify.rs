
use std::collections::HashMap;
use util::H256;

pub const BLOCKLIMIT: u64 = 100;

#[derive(Debug)]
pub struct Verifier {
    height_latest: Option<u64>,
    height_low: Option<u64>,
    hashes: HashMap<u64, Vec<H256>>,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier {
            height_latest: Some(0),
            height_low: Some(0),
            hashes: HashMap::new(),
        }
    }

    pub fn update_hashes(&mut self, h: u64, hashes: Vec<H256>) {
        trace!("update block's tx hashes for height:{} and the current low height:{} and latest height:{}",
               h, self.height_low.unwrap(), self.height_latest.unwrap());
        //check whether greater than the threshold value
        if (h - self.height_low.unwrap() + 1) > BLOCKLIMIT {
            //self.hashes.truncate(BLOCKLIMIT as usize);
            let max = h - BLOCKLIMIT + 1;
            for block_no in self.height_low.unwrap()..max {
                self.hashes.remove(&block_no);
            }
            self.height_low = Some(h - BLOCKLIMIT + 1);
        }
        self.hashes.insert(h,hashes);
        self.height_latest = Some(h);

    }

    pub fn check_hash_exist(&self, hash: &H256) -> bool  {
        for (_, hashes) in self.hashes.iter() {
            if hashes.contains(hash) {
                return true;
            }
        }
        return false;
    }
}