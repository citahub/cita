use std::collections::VecDeque;
use util::H256;

pub const BLOCKLIMIT: u64 = 100;

#[derive(Debug)]
pub struct Verifier {
    height: Option<u64>,
    height_low: Option<u64>,
    hashes: VecDeque<Vec<H256>>,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier {
            height: None,
            height_low: Some(0),
            hashes: VecDeque::new(),
        }
    }

    pub fn set_height(&mut self, h: u64) {
        if !self.height.is_none() && h <= self.height.unwrap() {
            return;
        }
        
        self.height = Some(h);
        self.update_hashes(h);
    }

    pub fn save_hash(&mut self, h: u64, hashes: Vec<H256>) {
        if let Some(height) = self.height {
            if h > (height - BLOCKLIMIT) && h <= height {
                self.hashes.push_front(hashes);
            }
        }
    }

    fn update_hashes(&mut self, h: u64) {
        if (h - self.height_low.unwrap() + 1) > BLOCKLIMIT {
            self.hashes.truncate(BLOCKLIMIT as usize);
            self.height_low = Some(h - BLOCKLIMIT + 1);
        }
    }

    pub fn check_hash_exist(&self, hash: &H256) -> bool  {
        for hashes in self.hashes.iter() {
            if hashes.contains(hash) {
                return true;
            }
        }
        return false;
    }
}