// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

use super::ContractCallExt;
use std::str::FromStr;

use crate::contracts::tools::method as method_tools;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;

use cita_types::{Address, H256};
use ethabi::{decode, ParamType};

lazy_static! {
    // Will use `getProtocolVersion` at next version after `v0.25.0`.
    // And the `getVersion` will be *Deprecated*.
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

    pub fn get_version(&self, block_tag: BlockTag) -> Option<u32> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*VERSION_HASH.as_slice(),
                None,
                block_tag,
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
        info!("Use default version.");
        0
    }
}

//#[cfg(test)]
//mod tests {
//    use super::VersionManager;
//    use crate::tests::helpers::init_executor;
//    use crate::types::block_number::BlockTag;
//
//    #[test]
//    fn test_get_version() {
//        let executor = init_executor();
//        let version_management = VersionManager::new(&executor);
//        let version = version_management.get_version(BlockTag::Pending).unwrap();
//        assert_eq!(version, 2);
//    }
//}
