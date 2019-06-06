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

//! Get Admin Info

use super::ContractCallExt;
use cita_types::Address;
use contracts::tools::{decode as decode_tools, method as method_tools};
use libexecutor::executor::Executor;
use std::str::FromStr;
use types::ids::BlockId;
use types::reserved_addresses;

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
    pub fn get_admin(&self, block_id: BlockId) -> Option<Address> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*GET_ADMIN.as_slice(), None, block_id)
            .ok()
            .and_then(|output| decode_tools::to_address(&output))
    }
}

#[cfg(test)]
mod tests {
    use super::Admin;
    use cita_types::Address;
    use tests::helpers::init_executor;
    use types::ids::BlockId;

    #[test]
    fn test_admin() {
        let executor = init_executor();
        let admin = Admin::new(&executor);
        let addr = admin.get_admin(BlockId::Pending).unwrap();
        assert_eq!(
            addr,
            Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")
        );
    }
}
