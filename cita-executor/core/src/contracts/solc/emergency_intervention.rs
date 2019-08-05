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

//! Emergency Intervention

use std::str::FromStr;

use super::ContractCallExt;
use crate::contracts::tools::method as method_tools;
use crate::libexecutor::executor::Executor;
use crate::types::block_tag::BlockTag;
use crate::types::reserved_addresses;

use cita_types::Address;
use ethabi::{decode, ParamType};

lazy_static! {
    static ref STATE_HASH: Vec<u8> = method_tools::encode_to_vec(b"state()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::EMERGENCY_INTERVENTION).unwrap();
}

/// Configuration items from system contract
pub struct EmergencyIntervention<'a> {
    executor: &'a Executor,
}

impl<'a> EmergencyIntervention<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        EmergencyIntervention { executor }
    }

    pub fn state(&self, block_tag: BlockTag) -> Option<bool> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*STATE_HASH.as_slice(), None, block_tag)
            .and_then(|output| {
                decode(&[ParamType::Bool], &output).map_err(|_| "decode value error".to_string())
            })
            .ok()
            .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_state() -> bool {
        info!("Use default emergency intervention state.");
        false
    }
}

//#[cfg(test)]
//mod tests {
//    use super::EmergencyIntervention;
//    use crate::tests::helpers::init_executor;
//    use crate::types::block_tag::BlockTag;
//
//    #[test]
//    fn test_state() {
//        let executor = init_executor();
//        let emergency_intervention = EmergencyIntervention::new(&executor);
//        let state = emergency_intervention.state(BlockTag::Pending).unwrap();
//        assert_eq!(state, false);
//    }
//}
