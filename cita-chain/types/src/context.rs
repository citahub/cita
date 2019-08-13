use crate::header::BlockNumber;
use cita_types::{Address, H256, U256};
use std::sync::Arc;

pub type LastHashes = Vec<H256>;

#[derive(Debug, Clone)]
pub struct Context {
    pub block_number: BlockNumber,
    pub coin_base: Address,
    pub timestamp: u64,
    pub difficulty: U256,
    pub last_hashes: Arc<LastHashes>,
    pub quota_used: U256,
    pub block_quota_limit: U256,
    pub account_quota_limit: U256,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            block_number: 0,
            coin_base: Address::default(),
            timestamp: 0,
            difficulty: U256::default(),
            block_quota_limit: U256::from(u64::max_value()),
            last_hashes: Arc::new(vec![]),
            quota_used: U256::default(),
            account_quota_limit: U256::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use cita_types::U256;

    #[test]
    fn test_default() {
        let context = Context::default();
        assert_eq!(context.quota_used, U256::zero());
        assert_eq!(context.block_quota_limit, U256::from(u64::max_value()));
    }
}
