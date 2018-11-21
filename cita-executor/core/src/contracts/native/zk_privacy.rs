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

use cita_types::{H256, U256};
use contracts::{native::factory::Contract, tools::method as method_tools};
use evm::action_params::ActionParams;
use evm::storage::*;
use evm::{Ext, GasLeft, ReturnData};
use std::collections::VecDeque;
use std::str::FromStr;
use zktx::base::*;
use zktx::c2p::*;
use zktx::convert::*;
use zktx::incrementalmerkletree::*;
use zktx::p2c::*;
use zktx::pedersen::PedersenDigest;

static TREE_DEPTH: usize = 60;
#[derive(Clone)]
// address 512  balance 512
pub struct ZkPrivacy {
    balances: Map,
    last_spent: Map,
    coins: Array,
    nullifier_set: Array,
    // merkle tree
    left: Scalar,
    right: Scalar,
    parents: Array,
    admin: Scalar,
    output: Vec<u8>,
}

impl Contract for ZkPrivacy {
    fn exec(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        if let Some(ref data) = params.data {
            method_tools::extract_to_u32(&data[..]).and_then(|signature| match signature {
                0 => self.init(params, ext),
                0x05e3_cb61 => self.set_balance(params, ext),
                0xd0b0_7e52 => self.get_balance(params, ext),
                0xc73b_5a8f => self.send_verify(params, ext),
                0x882b_30d2 => self.receive_verify(params, ext),
                _ => Err(evm::Error::OutOfGas),
            })
        } else {
            Err(evm::Error::OutOfGas)
        }
    }
    fn create(&self) -> Box<Contract> {
        Box::new(ZkPrivacy::default())
    }
}

impl Default for ZkPrivacy {
    fn default() -> Self {
        ZkPrivacy {
            output: Vec::new(),
            balances: Map::new(H256::from(0)),
            last_spent: Map::new(H256::from(1)),
            coins: Array::new(H256::from(2)),
            nullifier_set: Array::new(H256::from(3)),
            left: Scalar::new(H256::from(4)),
            right: Scalar::new(H256::from(5)),
            parents: Array::new(H256::from(6)),
            admin: Scalar::new(H256::from(7)),
        }
    }
}

impl ZkPrivacy {
    fn init(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(5000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }
        let sender = U256::from(H256::from(params.sender));
        self.admin.set(ext, sender)?;
        Ok(GasLeft::Known(params.gas - gas_cost))
    }

    fn set_balance(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }
        let data = params.data.to_owned().expect("invalid data");
        let mut index = 4;

        let mut len = 128;
        let addr = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        len = 128;
        let balance = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();

        trace!("set_balance {} {}", addr, balance);

        self.balances.set_bytes(ext, &addr, &balance)?;
        Ok(GasLeft::Known(params.gas - gas_cost))
    }

    fn get_balance(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }
        let data = params.data.to_owned().expect("invalid data");
        let index = 4;

        let len = 128;
        let addr = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();

        self.output.clear();
        let balance: String = self.balances.get_bytes(ext, &addr)?;
        for v in balance.as_bytes() {
            self.output.push(*v);
        }
        trace!("get_balance {} {}", addr, balance);

        Ok(GasLeft::NeedsReturn {
            gas_left: params.gas - gas_cost,
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }

    fn send_verify(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(10_000_000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }
        let data = params.data.to_owned().expect("invalid data");
        let mut index = 4;

        // get address
        let mut len = 128;
        let addr = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get proof
        len = 770;
        let proof = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get coin
        len = 64;
        let coin = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get delt_ba
        len = 128;
        let delt_ba = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get enc
        len = 192;
        let enc = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();

        // get block_number
        let block_number = U256::from(ext.env_info().number);

        trace!(
            "send_verify args: {} {} {} {} {} {}",
            addr,
            proof,
            coin,
            delt_ba,
            enc,
            block_number
        );

        // check coin is dup
        let coins_len = self.coins.get_len(ext)?;
        for i in 0..coins_len {
            let coin_added: String = *self.coins.get_bytes(ext, i)?;
            if coin == coin_added {
                return Err(evm::Error::Internal("dup coin".to_string()));
            }
        }

        // compare block number
        let last_block_number = self.last_spent.get(ext, &addr)?;
        if last_block_number >= block_number {
            return Err(evm::Error::Internal(
                "block_number less than last".to_string(),
            ));
        }

        // get balance
        let balance: String = self.balances.get_bytes(ext, &addr)?;
        trace!("balance {}", balance);

        let ret = p2c_verify(
            balance.clone(),
            coin.clone(),
            delt_ba.clone(),
            enc.clone(),
            addr.clone(),
            proof,
        );
        if ret.is_err() {
            return Err(evm::Error::Internal("p2c_verify error".to_string()));
        }

        if !ret.unwrap() {
            return Err(evm::Error::Internal("p2c_verify failed".to_string()));
        }

        // update last spent
        self.last_spent.set(ext, &addr, block_number)?;
        // add coin
        self.coins.set_bytes(ext, coins_len, &coin)?;
        self.coins.set_len(ext, coins_len + 1)?;

        // restore merkle tree form storage
        let mut tree = IncrementalMerkleTree::new(TREE_DEPTH);
        let left_str: String = *self.left.get_bytes(ext)?;
        let tree_left = if left_str.is_empty() {
            None
        } else {
            Some(PedersenDigest(str2u644(left_str)))
        };
        let right_str: String = *self.right.get_bytes(ext)?;
        let tree_right = if right_str.is_empty() {
            None
        } else {
            Some(PedersenDigest(str2u644(right_str)))
        };
        let mut parents = Vec::new();
        let parents_len = self.parents.get_len(ext)?;
        for i in 0..parents_len {
            let hash_str: String = *self.parents.get_bytes(ext, i)?;
            let hash = if hash_str.is_empty() {
                None
            } else {
                Some(PedersenDigest(str2u644(hash_str)))
            };
            parents.push(hash);
        }
        tree.restore(tree_left, tree_right, parents);

        // add coin to merkle tree
        tree.append(PedersenDigest(str2u644(coin.clone())));

        // get path of the coin
        let path = tree.path(VecDeque::new());
        let authentication_path = path.authentication_path;
        let index = path.index;

        // export merkle tree to storage
        let left = match tree.export_left() {
            Some(hash) => u6442str(hash.0),
            None => "".to_string(),
        };
        self.left.set_bytes(ext, &left)?;

        let right = match tree.export_right() {
            Some(hash) => u6442str(hash.0),
            None => "".to_string(),
        };
        self.right.set_bytes(ext, &right)?;

        let mut i = 0;
        for opt_hash in tree.export_parents().iter() {
            let str = match opt_hash {
                Some(ref hash) => u6442str(hash.0),
                None => "".to_string(),
            };
            self.parents.set_bytes(ext, i, &str)?;
            i += 1;
        }
        self.parents.set_len(ext, i)?;

        // sub balance
        let new_balance = ecc_sub(balance, delt_ba);
        self.balances.set_bytes(ext, &addr, &new_balance)?;

        let mut data = Vec::new();
        data.extend_from_slice(coin.as_bytes());
        data.extend_from_slice(enc.as_bytes());
        for hash in authentication_path {
            data.extend_from_slice(u6442str(hash.0).as_bytes());
        }
        for flag in index {
            if flag {
                data.push(1u8);
            } else {
                data.push(0u8);
            }
        }
        let _ = ext.log(
            vec![
                H256::from_str("c73b5a8f31a1a078a14123cc93687f4a59389c76caf88d5d2154d3f3ce25ff49")
                    .unwrap(),
            ],
            &data,
        );

        trace!("send_verify OK data len {}", data.len());

        Ok(GasLeft::Known(params.gas - gas_cost))
    }

    fn receive_verify(
        &mut self,
        params: &ActionParams,
        ext: &mut Ext,
    ) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(1_000_000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }
        let data = params.data.to_owned().expect("invalid data");
        let mut index = 4;

        // get address
        let mut len = 128;
        let addr = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get proof
        len = 770;
        let proof = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get nullifier
        len = 64;
        let nullifier = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get root
        len = 64;
        let root = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();
        index += len;

        // get delt_ba
        len = 128;
        let delt_ba = String::from_utf8(Vec::from(
            data.get(index..index + len).expect("no enough data"),
        ))
        .unwrap();

        trace!(
            "receive_verify args: {} {} {} {} {}",
            addr,
            proof,
            nullifier,
            root,
            delt_ba
        );

        // check nullifier is dup
        let nullifier_set_len = self.nullifier_set.get_len(ext)?;
        for i in 0..nullifier_set_len {
            let nullifier_in_set: String = *self.nullifier_set.get_bytes(ext, i)?;
            if nullifier == nullifier_in_set {
                return Err(evm::Error::Internal("dup nullifier".to_string()));
            }
        }

        // check root
        // str2u644(root.clone()) == tree.root()
        // restore merkle tree form storage
        let mut tree = IncrementalMerkleTree::new(TREE_DEPTH);
        let left_str: String = *self.left.get_bytes(ext)?;
        let tree_left = if left_str.is_empty() {
            None
        } else {
            Some(PedersenDigest(str2u644(left_str)))
        };
        let right_str: String = *self.right.get_bytes(ext)?;
        let tree_right = if right_str.is_empty() {
            None
        } else {
            Some(PedersenDigest(str2u644(right_str)))
        };
        let mut parents = Vec::new();
        let parents_len = self.parents.get_len(ext)?;
        for i in 0..parents_len {
            let hash_str: String = *self.parents.get_bytes(ext, i)?;
            let hash = if hash_str.is_empty() {
                None
            } else {
                Some(PedersenDigest(str2u644(hash_str)))
            };
            parents.push(hash);
        }
        tree.restore(tree_left, tree_right, parents);
        if str2u644(root.clone()) != tree.root().0 {
            return Err(evm::Error::Internal("invalid root hash".to_string()));
        }

        let ret = c2p_verify(nullifier.clone(), root, delt_ba.clone(), proof);
        if ret.is_err() {
            return Err(evm::Error::Internal("c2p_verify error".to_string()));
        }

        if !ret.unwrap() {
            return Err(evm::Error::Internal("c2p_verify failed".to_string()));
        }
        // add nullifier into nullifier_set
        self.nullifier_set
            .set_bytes(ext, nullifier_set_len, &nullifier)?;
        self.nullifier_set.set_len(ext, nullifier_set_len + 1)?;
        // add balance
        let balance: String = self.balances.get_bytes(ext, &addr)?;
        trace!("balance {}", balance);
        let new_balance = ecc_add(balance, delt_ba);
        self.balances.set_bytes(ext, &addr, &new_balance)?;

        trace!("receive_verify OK");

        Ok(GasLeft::Known(params.gas - gas_cost))
    }
}
