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

//! version management

use std::str::FromStr;

use super::ContractCallExt;
use contracts::tools::method as method_tools;
use libexecutor::executor::Executor;

use cita_types::Address;
use cita_types::H256;
use ethabi::{decode, ParamType};
use types::ids::BlockId;
use types::reserved_addresses;

lazy_static! {
    static ref VERSION_HASH: Vec<u8> = method_tools::encode_to_vec(b"getVersion()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::VERSION_MANAGEMENT).unwrap();
}

pub struct VersionManager<'a> {
    executor: &'a Executor,
}

impl<'a> VersionManager<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        VersionManager { executor }
    }

    pub fn get_version(&self, block_id: BlockId) -> Option<u32> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*VERSION_HASH.as_slice(),
                None,
                block_id,
            )
            .and_then(|output| {
                decode(&[ParamType::Uint(64)], &output)
                    .map_err(|_| "decode value error".to_string())
            })
            .ok()
            .and_then(|mut x| x.remove(0).to_uint())
            .map(|x| H256::from(x).low_u64() as u32)
    }

    pub fn default_version() -> u32 {
        error!("Use default emergency break state.");
        0
    }
}
