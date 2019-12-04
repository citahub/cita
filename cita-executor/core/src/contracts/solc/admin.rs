// Copyright Rivtower Technologies LLC.
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

//! Get Admin Info

use super::ContractCallExt;
use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;
use cita_types::Address;
use std::str::FromStr;

lazy_static! {
    static ref GET_ADMIN: Vec<u8> = method_tools::encode_to_vec(b"admin()");
    static ref CONTRACT_ADDRESS: Address = Address::from_str(reserved_addresses::ADMIN).unwrap();
}

pub struct Admin<'a> {
    executor: &'a Executor,
}

impl<'a> Admin<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        Admin { executor }
    }

    /// Get Admin
    pub fn get_admin(&self, block_tag: BlockTag) -> Option<Address> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*GET_ADMIN.as_slice(), None, block_tag)
            .ok()
            .and_then(|output| decode_tools::to_address(&output))
    }
}

#[cfg(test)]
mod tests {
    use super::Admin;
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};
    use cita_types::Address;

    #[test]
    fn test_admin() {
        let executor = init_executor();
        let admin = Admin::new(&executor);
        let addr = admin.get_admin(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(
            addr,
            Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")
        );
    }
}
