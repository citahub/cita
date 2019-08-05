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

//! Get Admin Info

use super::ContractCallExt;
use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::executor::Executor;
use crate::types::block_tag::BlockTag;
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

//#[cfg(test)]
//mod tests {
//    use super::Admin;
//    use crate::tests::helpers::init_executor;
//    use crate::types::block_tag::BlockTag;
//    use cita_types::Address;
//
//    #[test]
//    fn test_admin() {
//        let executor = init_executor();
//        let admin = Admin::new(&executor);
//        let addr = admin.get_admin(BlockTag::Pending).unwrap();
//        assert_eq!(
//            addr,
//            Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")
//        );
//    }
//}
