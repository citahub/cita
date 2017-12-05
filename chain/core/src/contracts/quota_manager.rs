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

//! Quota manager.

use super::{encode_contract_name, parse_string_to_addresses, parse_string_to_quota};
use super::ContractCallExt;
use libchain::chain::Chain;
use libproto::blockchain::AccountGasLimit as ProtoAccountGasLimit;
use rustc_hex::ToHex;
use std::collections::HashMap;
use std::str::FromStr;
use util::*;

const QUOTA: &'static [u8] = &*b"getUsersQuota()";
const USERS_METHOD_NAME: &'static [u8] = &*b"getSpecialUsers()";
const BLOCK_GAS_LIMIT: &'static [u8] = &*b"getblockGasLimit()";
const ACCOUNT_GAS_LIMIT: &'static [u8] = &*b"getAccountGasLimit()";

lazy_static! {
    static ref QUOTA_ENCODED: Vec<u8> = encode_contract_name(QUOTA);
    static ref USERS_METHOD_HASH: Vec<u8> = encode_contract_name(USERS_METHOD_NAME);
    static ref BLOCK_GAS_LIMIT_HASH: Vec<u8> = encode_contract_name(BLOCK_GAS_LIMIT);
    static ref ACCOUNT_GAS_LIMIT_HASH: Vec<u8> = encode_contract_name(ACCOUNT_GAS_LIMIT);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a3").unwrap();
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct AccountGasLimit {
    pub common_gas_limit: u64,
    pub specific_gas_limit: HashMap<Address, u64>,
}

impl AccountGasLimit {
    pub fn new() -> Self {
        AccountGasLimit {
            common_gas_limit: 4_294_967_296,
            specific_gas_limit: HashMap::new(),
        }
    }

    pub fn set_common_gas_limit(&mut self, v: u64) {
        self.common_gas_limit = v;
    }

    pub fn get_common_gas_limit(&self) -> u64 {
        self.common_gas_limit
    }

    pub fn set_specific_gas_limit(&mut self, v: HashMap<Address, u64>) {
        self.specific_gas_limit = v;
    }

    pub fn get_specific_gas_limit(&self) -> &HashMap<Address, u64> {
        &self.specific_gas_limit
    }
}

impl Into<ProtoAccountGasLimit> for AccountGasLimit {
    fn into(self) -> ProtoAccountGasLimit {
        let mut r = ProtoAccountGasLimit::new();
        r.common_gas_limit = self.common_gas_limit;
        let specific_gas_limit: HashMap<String, u64> = self.get_specific_gas_limit()
            .iter()
            .map(|(k, v)| (k.hex(), *v))
            .collect();
        r.set_specific_gas_limit(specific_gas_limit);
        r
    }
}

pub struct QuotaManager;

impl QuotaManager {
    /// Special account gas limit
    pub fn specific(chain: &Chain) -> HashMap<Address, u64> {
        let users = QuotaManager::users(chain);
        let quota = QuotaManager::quota(chain);
        let mut specific = HashMap::new();
        for (k, v) in users.iter().zip(quota.iter()) {
            specific.insert(*k, *v);
        }
        specific
    }

    /// Quota array
    pub fn quota(chain: &Chain) -> Vec<u64> {
        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*QUOTA_ENCODED.as_slice());
        trace!("quota output: {:?}", output);

        let quota: Vec<u64> = parse_string_to_quota(&output.to_vec());
        trace!("quota is {:?}", quota);

        quota
    }

    /// Account array
    pub fn users(chain: &Chain) -> Vec<Address> {
        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*USERS_METHOD_HASH.as_slice());
        trace!("users output: {:?}", output);

        let users: Vec<Address> = parse_string_to_addresses(&output.to_vec());
        trace!("users: {:?}", users);

        users
    }

    /// Global gas limit
    pub fn block_gas_limit(chain: &Chain) -> u64 {
        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*BLOCK_GAS_LIMIT_HASH.as_slice());
        trace!("block_gas_limit output: {:?}", output);

        let output_hex = ToHex::to_hex(output.as_slice());
        let block_gas_limit = u64::from_str_radix(&*output_hex, 16).unwrap_or(0);
        trace!("block_gas_limit: {:?}", block_gas_limit);

        block_gas_limit
    }

    /// Global account gas limit
    pub fn account_gas_limit(chain: &Chain) -> u64 {
        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*ACCOUNT_GAS_LIMIT_HASH.as_slice());
        trace!("account_gas_limit output: {:?}", output);

        let output_hex = ToHex::to_hex(output.as_slice());
        let account_gas_limit = u64::from_str_radix(&*output_hex, 16).unwrap_or(0);
        trace!("account_gas_limit: {:?}", account_gas_limit);

        account_gas_limit
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;
    use self::Chain;
    use super::*;
    use cita_crypto::{PrivKey, SIGNATURE_NAME};
    use libchain::block::{Block, BlockBody};
    use libproto::blockchain;
    use std::time::UNIX_EPOCH;
    use tests::helpers::init_chain;
    use types::transaction::SignedTransaction;
    use util::{Address, U256};

    #[allow(dead_code)]
    fn create_block(chain: &Chain, privkey: &PrivKey, to: Address, data: Vec<u8>, nonce: (u32, u32)) -> Block {
        let mut block = Block::new();

        block.set_parent_hash(chain.get_current_hash());
        block.set_timestamp(UNIX_EPOCH.elapsed().unwrap().as_secs());
        block.set_number(chain.get_current_height() + 1);

        let mut body = BlockBody::new();
        let mut txs = Vec::new();
        for i in nonce.0..nonce.1 {
            let mut tx = blockchain::Transaction::new();
            if to == Address::from(0) {
                tx.set_to(String::from(""));
            } else {
                tx.set_to(to.hex());
            }
            tx.set_nonce(U256::from(i).to_hex());
            tx.set_data(data.clone());
            tx.set_valid_until_block(100);
            tx.set_quota(999999);
            let stx = tx.sign(*privkey);

            let new_tx = SignedTransaction::new(&stx).unwrap();
            txs.push(new_tx);
        }
        body.set_transactions(txs);
        block.set_body(body);
        block
    }

    #[test]
    fn test_users() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8c0e\
                 304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcfd9786\
                 61983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        println!("init chain finish");

        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*USERS_METHOD_HASH.as_slice());
        let users = parse_string_to_addresses(&output);

        assert_eq!(
            users,
            vec![
                H160::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap(),
            ]
        );
    }

    #[test]
    fn test_quota() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8\
                 c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcf\
                 d978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        println!("init chain finish");

        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*QUOTA_ENCODED.as_slice());
        let quota = parse_string_to_quota(&output);

        assert_eq!(quota, vec![61415926]);
    }

    #[test]
    fn test_block_gas_limit() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9\
                 ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed\
                 2bcbcfd978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        println!("init chain finish");

        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*BLOCK_GAS_LIMIT_HASH.as_slice());
        let output_hex = ToHex::to_hex(output.as_slice());
        let block_gas_limit = u32::from_str_radix(&*output_hex, 16).unwrap();

        assert_eq!(block_gas_limit, 61415926);
    }

    #[test]
    fn test_account_gas_limit() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c\
                 9ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223\
                 ed2bcbcfd978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        println!("init chain finish");

        let output = chain.call_contract_method(&*CONTRACT_ADDRESS, &*ACCOUNT_GAS_LIMIT_HASH.as_slice());
        let output_hex = ToHex::to_hex(output.as_slice());
        println!("output hex {:?}", output_hex);
        let account_gas_limit = u32::from_str_radix(&*output_hex, 16).unwrap();


        assert_eq!(account_gas_limit, 25141592);
    }
}
