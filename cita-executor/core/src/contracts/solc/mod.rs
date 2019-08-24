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
