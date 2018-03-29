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

use super::ContractCallExt;
use super::encode_contract_name;
use ethabi::{decode, ParamType};
use libexecutor::executor::Executor;
use std::str::FromStr;
use util::*;

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

        let mut decoded = decode(&[ParamType::Uint(256)], &output).expect("decode delay number");
        let delay_number = decoded.remove(0);
        let delay_number = delay_number.to_uint();

        let h256 = H256::from(delay_number.expect("decode delay number"));
        debug!("delay block number: {:?}", h256.low_u64());
        h256.low_u64()
    }

    /// Whether check permission or not
    pub fn permission_check(executor: &Executor) -> bool {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*PERMISSION_CHECK_ENCODED.as_slice());
        trace!("check permission output: {:?}", output);

        let mut decoded = decode(&[ParamType::Bool], &output).expect("decode check permission");
        let check_permission = decoded.remove(0);
        let check_permission = check_permission.to_bool();

        let check = check_permission.expect("decode check permission");
        debug!("check permission: {:?}", check);
        check
    }

    /// Whether check quota or not
    pub fn quota_check(executor: &Executor) -> bool {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*QUOTA_CHECK_ENCODED.as_slice());
        trace!("check quota output: {:?}", output);

        let mut decoded = decode(&[ParamType::Bool], &output).expect("decode check quota");
        let check_quota = decoded.remove(0);
        let check_quota = check_quota.to_bool();

        let check = check_quota.expect("decode check quota");
        debug!("check quota: {:?}", check);
        check
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::*;
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
