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

//! Account Permission manager.

use super::parse_output_to_addresses;
use libexecutor::call_request::CallRequest;
use libexecutor::executor::Executor;
use rustc_hex::FromHex;
use sha3::sha3_256;
use std::collections::HashSet;
use std::str::FromStr;
use types::ids::BlockId;
use util::*;

const METHOD_NAME: &'static [u8] = &*b"queryUsersOfPermission(uint8)";

lazy_static! {
    static ref METHOD_NAME_HASH: Vec<u8> = {
        let out :&mut[u8;32] = &mut [0;32];
        let outptr = out.as_mut_ptr();
        unsafe {
            sha3_256(outptr, 32, METHOD_NAME.as_ptr(), METHOD_NAME.len());
        }
        out[0..4].to_vec()
    };
    static ref QUERY_TX: Vec<u8> = "00000000000000000000000000000000000000000\
                                    00000000000000000000001".from_hex().unwrap().into();
    static ref QUERY_CONTRACT: Vec<u8> = "00000000000000000000000000000000000\
                                          00000000000000000000000000002".from_hex().unwrap().into();
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(
        "00000000000000000000000000000000013241a4"
    ).unwrap();
}

pub struct AccountManager;

impl AccountManager {
    pub fn load_senders(executor: &Executor) -> HashSet<Address> {
        let mut senders = HashSet::new();
        let mut tx_data = METHOD_NAME_HASH.to_vec().clone();
        tx_data.extend(QUERY_TX.to_vec());
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(tx_data),
        };

        trace!("data: {:?}", call_request.data);
        let output = executor
            .eth_call(call_request, BlockId::Latest)
            .expect("load senders eth call");
        trace!("read account which has tx permission output: {:?}", output);
        let accounts: Vec<Address> = parse_output_to_addresses(&output);
        trace!("accounts: {:?}", accounts);
        for account in accounts {
            senders.insert(account);
        }
        senders
    }

    pub fn load_creators(executor: &Executor) -> HashSet<Address> {
        let mut creators = HashSet::new();
        let mut contract_data = METHOD_NAME_HASH.to_vec().clone();
        contract_data.extend(QUERY_CONTRACT.to_vec());
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(contract_data),
        };

        trace!("data: {:?}", call_request.data);
        let output = executor
            .eth_call(call_request, BlockId::Latest)
            .expect("load creators eth call");
        trace!(
            "read account which has contract permission output: {:?}",
            output
        );
        let accounts: Vec<Address> = parse_output_to_addresses(&output);
        trace!("accounts: {:?}", accounts);
        for account in accounts {
            creators.insert(account);
        }
        assert!(!creators.is_empty(), "there must be at least one creator");
        creators
    }
}
