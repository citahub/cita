// Copyrighttape Technologies LLC.
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

//! Emergency Intervention

use std::str::FromStr;

use super::ContractCallExt;
use crate::contracts::tools::method as method_tools;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;

use cita_types::Address;
use ethabi::{decode, ParamType};

lazy_static! {
    static ref STATE_HASH: Vec<u8> = method_tools::encode_to_vec(b"state()");
    static ref CONTRACT_ADDRESS: Address = Address::from_str(reserved_addresses::EMERGENCY_INTERVENTION).unwrap();
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
            .and_then(|output| decode(&[ParamType::Bool], &output).map_err(|_| "decode value error".to_string()))
            .ok()
            .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_state() -> bool {
        info!("Use default emergency intervention state.");
        false
    }
}

#[cfg(test)]
mod tests {
    use super::EmergencyIntervention;
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};

    #[test]
    fn test_state() {
        let executor = init_executor();
        let emergency_intervention = EmergencyIntervention::new(&executor);
        let state = emergency_intervention.state(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(state, false);
    }
}
