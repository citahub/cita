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

//! Node manager.

use super::{encode_contract_name, to_address_vec, to_u256_vec};
use super::ContractCallExt;
use cita_types::{Address, H160};
use largest_remainder_method::apportion;
use libexecutor::executor::{EconomicalModel, Executor};
use rand::{Rng, SeedableRng, StdRng};
use rustc_hex::ToHex;
use std::iter;
use std::str::FromStr;

const LIST_NODE: &'static [u8] = &*b"listNode()";
const LIST_STAKE: &'static [u8] = *&b"listStake()";
/// Each epoch is divided into 1000 slots, each slot represent one opportunity of block proposer
const EPOCH: u64 = 1000;

lazy_static! {
    static ref LIST_NODE_ENCODED: Vec<u8> = encode_contract_name(LIST_NODE);
    static ref LIST_STAKE_ENCODED: Vec<u8> = encode_contract_name(LIST_STAKE);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a2").unwrap();
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

pub fn shuffle<T>(node_vec: &mut Vec<T>, rng_seed: u64) {
    let seed: &[_] = &[rng_seed as usize];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    for i in 0..node_vec.len() {
        let j: usize = rng.gen::<usize>() % (i + 1);
        node_vec.swap(i, j);
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

    pub fn nodes(&self) -> Vec<Address> {
        let output = self.executor
            .call_method_latest(&*CONTRACT_ADDRESS, &*LIST_NODE_ENCODED.as_slice());

        trace!(
            "node manager output: {:?}",
            ToHex::to_hex(output.as_slice())
        );

        let nodes: Vec<Address> = to_address_vec(&output);
        trace!("node manager nodes: {:?}", nodes);
        nodes
    }

    pub fn stakes(&self) -> Vec<u64> {
        let output = self.executor
            .call_method_latest(&*CONTRACT_ADDRESS, &*LIST_STAKE_ENCODED.as_slice());

        trace!("stakes output: {:?}", ToHex::to_hex(output.as_slice()));

        let stakes: Vec<u64> = to_u256_vec(&output).iter().map(|i| i.low_u64()).collect();
        trace!("node manager stakes: {:?}", stakes);
        stakes
    }

    pub fn shuffled_stake_nodes(&self) -> Vec<Address> {
        let mut stake_nodes = self.stake_nodes();
        shuffle(&mut stake_nodes, self.rng_seed);
        stake_nodes
    }

    pub fn stake_nodes(&self) -> Vec<Address> {
        let nodes = self.nodes();
        if let EconomicalModel::Quota = *self.executor.economical_model.read() {
            return nodes;
        }
        let stakes = self.stakes();
        let total = stakes.iter().fold(0, |acc, &x| acc + x);

        if total == 0 {
            return nodes;
        }
        let total_seats = apportion(&stakes, EPOCH);
        party_seats(nodes, &total_seats)
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::{party_seats, NodeManager};
    use cita_types::H160;
    use std::str::FromStr;
    use tests::helpers::init_executor;

    #[test]
    fn test_node_manager_contract() {
        let executor = init_executor();
        let node_manager = NodeManager::new(&executor, executor.genesis_header().timestamp());
        let nodes = node_manager.nodes();

        assert_eq!(
            nodes,
            vec![
                H160::from_str("50ad2b9d6946d9c75ae978534043e3021ee1bfb1").unwrap(),
                H160::from_str("eeb3a71c4046f63a941013f826fccc503be26b77").unwrap(),
                H160::from_str("a2bbb65d4f8c3ada29f7471abe416e18061127f3").unwrap(),
                H160::from_str("72eb1e258c9cdccebb7b62930a35cfb6ef4cd24b").unwrap(),
            ]
        )
    }

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
}
