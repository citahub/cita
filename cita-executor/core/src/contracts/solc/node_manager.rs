// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Node manager.

use super::ContractCallExt;
use std::iter;
use std::str::FromStr;

use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;

use cita_types::{Address, H160};
use largest_remainder_method::apportion;
use rand::{Rng, SeedableRng, StdRng};

const LIST_NODE: &[u8] = &*b"listNode()";
const LIST_STAKE: &[u8] = &*b"listStake()";
/// Each epoch is divided into 1000 slots, each slot represent one opportunity of block proposer
const EPOCH: u64 = 1000;

lazy_static! {
    static ref LIST_NODE_ENCODED: Vec<u8> = method_tools::encode_to_vec(LIST_NODE);
    static ref LIST_STAKE_ENCODED: Vec<u8> = method_tools::encode_to_vec(LIST_STAKE);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::NODE_MANAGER).unwrap();
}

pub fn party_seats<T>(parties: Vec<T>, seats: &[u64]) -> Vec<T>
where
    T: Clone,
{
    let mut party_seats: Vec<T> = Vec::new();
    let seats_len = seats.len();
    for (index, party) in parties.into_iter().enumerate() {
        if index < seats_len {
            party_seats.extend(iter::repeat(party).take(seats[index] as usize));
        }
    }
    party_seats
}

pub fn shuffle<T>(items: &mut Vec<T>, rng_seed: u64) {
    let seed: &[_] = &[rng_seed as usize];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    for i in 0..items.len() {
        let j: usize = rng.gen::<usize>() % (i + 1);
        items.swap(i, j);
    }
}

/// Configuration items from system contract
pub struct NodeManager<'a> {
    executor: &'a Executor,
    rng_seed: u64,
}

impl<'a> NodeManager<'a> {
    pub fn new(executor: &'a Executor, rng_seed: u64) -> Self {
        NodeManager { executor, rng_seed }
    }

    pub fn nodes(&self, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*LIST_NODE_ENCODED.as_slice(), None, block_tag)
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn stakes(&self, block_tag: BlockTag) -> Option<Vec<u64>> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*LIST_STAKE_ENCODED.as_slice(), None, block_tag)
            .ok()
            .and_then(|output| decode_tools::to_u64_vec(&output))
    }

    pub fn shuffled_stake_nodes(&self, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.stake_nodes(block_tag).map(|mut stake_nodes| {
            shuffle(&mut stake_nodes, self.rng_seed);
            stake_nodes
        })
    }

    pub fn default_shuffled_stake_nodes() -> Vec<Address> {
        info!("Use default shuffled stake nodes.");
        Vec::new()
    }

    pub fn stake_nodes(&self, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.nodes(block_tag).and_then(|nodes| {
            if let EconomicalModel::Quota = self.executor.sys_config.block_sys_config.economical_model {
                Some(nodes)
            } else {
                self.stakes(block_tag).map(|stakes| {
                    let total: u64 = stakes.iter().sum();
                    if total == 0 {
                        nodes
                    } else {
                        let total_seats = apportion(&stakes, EPOCH);
                        party_seats(nodes, &total_seats)
                    }
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_logger as logger;
    use super::{party_seats, shuffle};

    #[test]
    fn test_party_seats() {
        let parties = vec!["a", "b", "c"];
        let seats = vec![3, 5, 2, 2, 1];
        assert_eq!(
            party_seats(parties, &seats),
            vec!["a", "a", "a", "b", "b", "b", "b", "b", "c", "c"]
        );

        let parties = vec!["a", "b"];
        let seats = vec![2, 1];
        assert_eq!(party_seats(parties, &seats), vec!["a", "a", "b"]);

        let parties = vec!["a", "b", "c"];
        let seats = vec![2, 2];
        assert_eq!(party_seats(parties, &seats), vec!["a", "a", "b", "b"]);
    }

    #[test]
    fn test_shuffle() {
        let mut items = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2];
        shuffle(&mut items, 998);
        assert_eq!(items, vec![2, 1, 1, 2, 1, 2, 2, 1, 1, 2]);

        let mut items2 = vec![1; 50];
        items2.extend(vec![2; 50].iter());
        items2.extend(vec![3; 50].iter());
        shuffle(&mut items2, 1024);
        assert_eq!(
            items2,
            vec![
                2, 2, 1, 3, 2, 3, 1, 2, 1, 1, 1, 1, 3, 3, 1, 3, 3, 3, 1, 2, 3, 3, 3, 1, 1, 2, 2, 2, 2, 3, 1, 3, 3, 3, 3, 1,
                3, 3, 1, 3, 1, 2, 2, 1, 2, 2, 2, 1, 2, 3, 3, 1, 2, 2, 1, 2, 1, 3, 3, 2, 2, 1, 1, 1, 1, 2, 3, 2, 1, 3, 3, 2,
                2, 2, 2, 2, 2, 3, 1, 1, 1, 3, 1, 1, 2, 1, 1, 2, 3, 1, 3, 3, 2, 2, 1, 2, 2, 1, 3, 3, 3, 1, 3, 3, 3, 1, 1, 1,
                3, 1, 2, 1, 2, 2, 1, 2, 1, 3, 3, 2, 1, 2, 2, 3, 1, 2, 2, 1, 3, 1, 3, 3, 2, 1, 3, 2, 3, 1, 3, 3, 1, 3, 1, 2,
                3, 3, 2, 2, 2, 2,
            ]
        );
    }
}
