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

use super::{encode_contract_name, to_address_vec, to_low_u64_vec};
use super::ContractCallExt;
use libexecutor::executor::Executor;
use rand::{Rng, SeedableRng, StdRng};
use rustc_hex::ToHex;
use std::cmp::Ordering;
use std::iter;
use std::str::FromStr;
use util::{Address, H160};
use rand::{Rng, SeedableRng, StdRng};

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

/// https://en.wikipedia.org/wiki/Largest_remainder_method
/// Hare quota
pub fn largest_remainder_electoral(votes: &Vec<u64>, seat_number: u64) -> Vec<u64> {
    let total = votes.iter().fold(0, |acc, &x| acc + x);
    // TODO: PLS fix me!!!
    let hare_quota = total as f64 / seat_number as f64;

    let votes_quota: Vec<f64> = votes
        .iter()
        .map(|vote| (*vote as f64) / hare_quota)
        .collect();

    // calculate automatic seats first
    let mut seats: Vec<u64> = votes_quota.iter().map(|v| v.floor() as u64).collect();

    let mut remainders: Vec<(f64, u64)> = votes_quota
        .iter()
        .enumerate()
        .map(|(i, v)| (v.fract(), i as u64))
        .collect();

    let remaining_seat: u64 = seat_number - seats.iter().fold(0, |acc, &x| acc + x);

    remainders.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    remainders.reverse();

    let highest_remainder_seats: Vec<u64> = remainders
        .iter()
        .take(remaining_seat as usize)
        .map(|v| v.1)
        .collect();

    for index in highest_remainder_seats {
        seats[index as usize] += 1;
    }

    seats
}

pub struct NodeManager;

impl NodeManager {
    pub fn nodes(executor: &Executor) -> Vec<Address> {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*LIST_NODE_ENCODED.as_slice());
        trace!(
            "node manager output: {:?}",
            ToHex::to_hex(output.as_slice())
        );

        let nodes: Vec<Address> = to_address_vec(&output);
        trace!("node manager nodes: {:?}", nodes);
        nodes
    }

    pub fn shuffle_node<T>(node_vec: &mut Vec<T>, rng_seed: u64) {
        let seed: &[_] = &[rng_seed as usize];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for i in 0..node_vec.len() {
            let j: usize = rng.gen::<usize>() % (i + 1);
            node_vec.swap(i, j);
        }
    }

    pub fn stakes(executor: &Executor) -> Vec<u64> {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*LIST_STAKE_ENCODED.as_slice());
        trace!("stakes output: {:?}", ToHex::to_hex(output.as_slice()));

        let stakes: Vec<u64> = to_low_u64_vec(&output);
        trace!("node manager stakes: {:?}", stakes);
        stakes
    }

    pub fn stake_nodes(exeuctor: &Executor) -> Vec<Address> {
        let nodes = NodeManager::nodes(&exeuctor);
        let stakes = NodeManager::stakes(&exeuctor);
        let total = stakes.iter().fold(0, |acc, &x| acc + x);
        let stake_nodes: Vec<Address>;

        if total == 0 {
            stake_nodes = nodes;
        } else {
            let total_seats = largest_remainder_electoral(&stakes, EPOCH);
            stake_nodes = party_seats(nodes, &total_seats);
        }
        stake_nodes
=======
            if j != i {
                ret[i] = ret[j];
            }
            ret[j] = node_vec[i];
        }
        ret
>>>>>>> provide the random-shuffle function
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::{largest_remainder_electoral, party_seats, NodeManager};
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use util::H160;

    #[test]
    fn test_node_manager_contract() {
        let executor = init_executor();
        let nodes = NodeManager::nodes(&executor);

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
    fn test_largest_remainder_electoral() {
        let votes = vec![47_000, 16_000, 15_800, 12_000, 6_100, 3_100];
        let seats = 10;
        let total_seats = largest_remainder_electoral(&votes, seats);
        assert_eq!(vec![5, 2, 1, 1, 1, 0], total_seats);
    }

    #[test]
    fn test_largest_remainder_electoral_again() {
        let votes = vec![1500, 1500, 900, 500, 500, 200];
        let seats = 25;
        let total_seats = largest_remainder_electoral(&votes, seats);
        assert_eq!(vec![7, 7, 4, 3, 3, 1], total_seats);

        let new_seats = 26;
        let new_total_seats = largest_remainder_electoral(&votes, new_seats);
        assert_eq!(vec![8, 8, 5, 2, 2, 1], new_total_seats);
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
