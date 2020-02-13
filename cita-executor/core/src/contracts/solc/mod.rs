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

//! System contracts.

pub mod admin;
pub mod chain_manager;
pub mod emergency_intervention;
pub mod node_manager;
pub mod permission_management;
pub mod price_manager;
pub mod quota_manager;
pub mod sys_config;
pub mod user_management;
pub mod version_management;

pub use self::chain_manager::ChainManagement;
pub use self::emergency_intervention::EmergencyIntervention;
pub use self::node_manager::NodeManager;
pub use self::permission_management::{PermissionManagement, Resource};
pub use self::price_manager::PriceManagement;
pub use self::quota_manager::{AccountQuotaLimit, QuotaManager, AUTO_EXEC_QL_VALUE};
pub use self::sys_config::SysConfig;
pub use self::user_management::UserManagement;
pub use self::version_management::VersionManager;

use crate::libexecutor::call_request::CallRequest;
use crate::libexecutor::command::Commander;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use cita_types::Address;
use types::Bytes;

/// Extend `Executor` with some methods related to contract
trait ContractCallExt {
    /// Call a contract method
    fn call_method(
        &self,
        address: &Address,
        encoded_method: &[u8],
        from: Option<Address>,
        block_id: BlockTag,
    ) -> Result<Bytes, String>;
}

impl<'a> ContractCallExt for Executor {
    fn call_method(
        &self,
        address: &Address,
        encoded_method: &[u8],
        from: Option<Address>,
        block_tag: BlockTag,
    ) -> Result<Bytes, String> {
        let call_request = CallRequest {
            from,
            to: *address,
            data: Some(encoded_method.to_vec()),
        };
        trace!("call method request: {:?}", call_request);
        self.eth_call(call_request, block_tag)
    }
}
