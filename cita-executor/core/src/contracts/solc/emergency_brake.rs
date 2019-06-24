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

//! Emergency Break

use std::str::FromStr;

use super::ContractCallExt;
use crate::contracts::tools::method as method_tools;
use crate::libexecutor::executor::Executor;

use crate::types::ids::BlockId;
use crate::types::reserved_addresses;
use cita_types::Address;
use ethabi::{decode, ParamType};

lazy_static! {
    static ref STATE_HASH: Vec<u8> = method_tools::encode_to_vec(b"state()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::EMERGENCY_BRAKE).unwrap();
}

/// Configuration items from system contract
pub struct EmergencyBrake<'a> {
    executor: &'a Executor,
}

impl<'a> EmergencyBrake<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        EmergencyBrake { executor }
    }

    pub fn state(&self, block_id: BlockId) -> Option<bool> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*STATE_HASH.as_slice(), None, block_id)
            .and_then(|output| {
                decode(&[ParamType::Bool], &output).map_err(|_| "decode value error".to_string())
            })
            .ok()
            .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_state() -> bool {
        info!("Use default emergency break state.");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::EmergencyBrake;
    use crate::tests::helpers::init_executor;
    use crate::types::ids::BlockId;

    #[test]
    fn test_state() {
        let executor = init_executor();
        let emergency_brake = EmergencyBrake::new(&executor);
        let state = emergency_brake.state(BlockId::Pending).unwrap();
        assert_eq!(state, false);
    }
}
