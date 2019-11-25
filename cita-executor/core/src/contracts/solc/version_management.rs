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
            .call_method(&*CONTRACT_ADDRESS, &*VERSION_HASH.as_slice(), None, block_tag)
            .and_then(|output| decode(&[ParamType::Uint(64)], &output).map_err(|_| "decode value error".to_string()))
            .ok()
            .and_then(|mut x| x.remove(0).to_uint())
            .map(|x| H256::from(x).low_u64() as u32)
    }

    pub fn default_version() -> u32 {
        info!("Use default version.");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::VersionManager;
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};

    #[test]
    fn test_get_version() {
        let executor = init_executor();
        let version_management = VersionManager::new(&executor);
        let version = version_management.get_version(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(version, 2);
    }
}
