// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_types::traits::LowerHex;
use cita_types::{Address, H256};
use crypto::{pubkey_to_address, PubKey};
use libproto::blockchain::AccountGasLimit;
use libproto::blockchain::SignedTransaction;
use std::collections::HashMap;

pub struct Block {
    pub transaction_root: H256,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn transactions(&self) -> &[SignedTransaction] {
        &self.transactions
    }
}

impl Block {
    pub fn verify_quota(
        &self,
        block_quota_limit: u64,
        account_quota_limit: &AccountGasLimit,
        check_quota: bool,
    ) -> bool {
        let quota_limit = account_quota_limit.get_common_quota_limit();
        let mut specific_quota_limit = account_quota_limit.get_specific_quota_limit().clone();
        let mut account_gas_used: HashMap<Address, u64> = HashMap::new();
        let mut block_quota_limit = block_quota_limit;
        let transactions = self.transactions();
        for tx in transactions {
            let quota = tx.get_transaction_with_sig().get_transaction().get_quota();
            let signer = pubkey_to_address(&PubKey::from(tx.get_signer()));

            if block_quota_limit < quota {
                return false;
            }

            if check_quota {
                let value = account_gas_used.entry(signer).or_insert_with(|| {
                    if let Some(value) = specific_quota_limit.remove(&signer.lower_hex()) {
                        value
                    } else {
                        quota_limit
                    }
                });
                if *value < quota {
                    return false;
                } else {
                    *value -= quota;
                }
            }
            block_quota_limit -= quota;
        }
        true
    }

    pub fn verify_version() -> bool {
        unimplemented!();
    }

    pub fn verify_valid_until_block() -> bool {
        unimplemented!();
    }

    pub fn verify_emergency_brake() -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::{CreateKey, KeyPair};
    use libproto::Transaction;

    #[test]
    fn test_verify_quota() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let mut raw_tx = Transaction::new();
        raw_tx.quota = 1000;
        let tx = raw_tx.sign(*privkey);

        let block = Block {
            transaction_root: Default::default(),
            transactions: vec![tx],
        };

        let mut account_quota_limit = AccountGasLimit::new();
        account_quota_limit.set_common_quota_limit(5000);

        // block_quota_limit and account_quota_limit pass
        assert!(block.verify_quota(10000, &account_quota_limit, true));
        // block_quota_limit failed
        assert_eq!(block.verify_quota(1, &account_quota_limit, true), false);
        assert_eq!(block.verify_quota(1, &account_quota_limit, false), false);

        account_quota_limit.set_common_quota_limit(500);
        // common_quota_limit failed
        assert_eq!(block.verify_quota(10000, &account_quota_limit, true), false);

        account_quota_limit.set_common_quota_limit(500);
        let address = pubkey_to_address(keypair.pubkey());
        account_quota_limit
            .mut_specific_quota_limit()
            .insert(address.lower_hex(), 5000);
        // specific_quota_limit pass
        assert!(block.verify_quota(10000, &account_quota_limit, true));

        // specific_quota_limit failed
        account_quota_limit
            .mut_specific_quota_limit()
            .insert(address.lower_hex(), 500);
        assert_eq!(block.verify_quota(10000, &account_quota_limit, true), false);
    }
}
