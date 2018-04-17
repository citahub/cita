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

//! Constant Config

use super::{encode_contract_name, to_bool, to_low_u64};
use super::ContractCallExt;
use libexecutor::executor::Executor;
use std::str::FromStr;
use util::H160;

const VALID_NUMBER: &'static [u8] = &*b"getNumber()";
const PERMISSION_CHECK: &'static [u8] = &*b"getPermissionCheck()";
const QUOTA_CHECK: &'static [u8] = &*b"getQuotaCheck()";

lazy_static! {
    static ref VALID_NUMBER_ENCODED: Vec<u8> = encode_contract_name(VALID_NUMBER);
    static ref PERMISSION_CHECK_ENCODED: Vec<u8> = encode_contract_name(PERMISSION_CHECK);
    static ref QUOTA_CHECK_ENCODED: Vec<u8> = encode_contract_name(QUOTA_CHECK);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("0000000000000000000000000000000031415926").unwrap();
}

pub struct ConstantConfig;

impl ConstantConfig {
    /// Delay block number before validate
    pub fn valid_number(executor: &Executor) -> u64 {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*VALID_NUMBER_ENCODED.as_slice());
        trace!("delay block number output: {:?}", output);

        to_low_u64(&output)
    }

    /// Whether check permission or not
    pub fn permission_check(executor: &Executor) -> bool {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*PERMISSION_CHECK_ENCODED.as_slice());
        trace!("check permission output: {:?}", output);

        to_bool(&output)
    }

    /// Whether check quota or not
    pub fn quota_check(executor: &Executor) -> bool {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*QUOTA_CHECK_ENCODED.as_slice());
        trace!("check quota output: {:?}", output);

        to_bool(&output)
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::ConstantConfig;
    use tests::helpers::init_executor;

    #[test]
    fn test_valid_number() {
        let executor = init_executor();
        let number = ConstantConfig::valid_number(&executor);

        assert_eq!(number, 1);
    }

    #[test]
    fn test_permission_check() {
        let executor = init_executor();
        let check_permission = ConstantConfig::permission_check(&executor);

        // Is true in the test module.
        assert_eq!(check_permission, true);
    }

    #[test]
    fn test_quota_check() {
        let executor = init_executor();
        let check_quota = ConstantConfig::quota_check(&executor);

        // Is true in the test module.
        assert_eq!(check_quota, true);
    }
}
