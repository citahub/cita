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

use super::parse_string_to_addresses;
use libchain::call_request::CallRequest;
use libchain::chain::Chain;
use rustc_hex::FromHex;
use sha3::sha3_256;
use std::collections::HashMap;
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
        let func = out[0..4].to_vec();
        func
	};
    static ref QUERY_TX: Vec<u8> = "0000000000000000000000000000000000000000000000000000000000000001".from_hex().unwrap().into();
    static ref QUERY_CONTRACT: Vec<u8> = "0000000000000000000000000000000000000000000000000000000000000002".from_hex().unwrap().into();
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a4").unwrap();
}

pub struct AccountManager;

impl AccountManager {
    pub fn load_senders(chain: &Chain) -> HashMap<Address, bool> {
        let mut senders = HashMap::new();
        let mut tx_data = METHOD_NAME_HASH.to_vec().clone();
        tx_data.extend(QUERY_TX.to_vec());
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(tx_data),
        };

        trace!("data: {:?}", call_request.data);
        let output = chain.eth_call(call_request, BlockId::Latest).unwrap();
        trace!("read account which has tx permission output: {:?}", output);
        let accounts: Vec<Address> = parse_string_to_addresses(&output);
        trace!("accounts: {:?}", accounts);
        for account in accounts {
            senders.insert(account, true);
        }
        senders
    }

    pub fn load_creators(chain: &Chain) -> HashMap<Address, bool> {
        let mut creators = HashMap::new();
        let mut contract_data = METHOD_NAME_HASH.to_vec().clone();
        contract_data.extend(QUERY_CONTRACT.to_vec());
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(contract_data),
        };

        trace!("data: {:?}", call_request.data);
        let output = chain.eth_call(call_request, BlockId::Latest).unwrap();
        trace!("read account which has contract permission output: {:?}", output);
        let accounts: Vec<Address> = parse_string_to_addresses(&output);
        trace!("accounts: {:?}", accounts);
        for account in accounts {
            creators.insert(account, true);
        }
        creators
    }
}
