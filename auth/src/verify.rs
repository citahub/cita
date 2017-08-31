use std::collections::HashMap;
use util::H256;

pub const BLOCKLIMIT: u64 = 100;

#[derive(Debug)]
pub struct Verifyer {
    height: Option<u64>,
    hashs: HashMap<u64, Vec<H256>>,
}

impl Verifyer {
    pub fn new() -> Self {
        Verifyer {
            height: None,
            hashs: HashMap::new(),
        }
    }

    pub fn set_height(&mut self, h: u64) {
        if !self.height.is_none() &&
        h <= self.height.unwrap() {
            return;
        }
        
        self.height = Some(h);
        self.update_hashs(h);
    }

    pub fn save_hash(&mut self, h: u64, hashs: Vec<H256>) {
        if let Some(height) = self.height {
            if h > (height - BLOCKLIMIT) && h <= height {
                self.hashs.insert(h, hashs);
            }
        }
    }

    fn update_hashs(&mut self, h: u64) {
        for (height, _) in self.hashs {
            if height <= (height - BLOCKLIMIT) {
                self.hashs.remove(&h);
            }
        }
    }

    pub fn check_hash(&self, hash: &H256) -> bool  {
        for (_, hashs) in self.hashs {
            if hashs.contains(hash) {
                return false;
            }
        }
        return true;
    }
}