// Copyright Cryptape Technologies LLC.
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

use jsonrpc_types::rpc_types::EconomicalModel as RpcEconomicalModel;

enum_from_primitive! {
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
    pub enum EconomicalModel {
        /// Default model. Sending Transaction is free, should work with authority together.
        Quota,
        /// Transaction charges for gas * gasPrice. BlockProposer get the block reward.
        Charge,
    }
}

impl Default for EconomicalModel {
    fn default() -> Self {
        EconomicalModel::Quota
    }
}

impl From<EconomicalModel> for RpcEconomicalModel {
    fn from(em: EconomicalModel) -> Self {
        match em {
            EconomicalModel::Quota => RpcEconomicalModel::Quota,
            EconomicalModel::Charge => RpcEconomicalModel::Charge,
        }
    }
}

impl Into<EconomicalModel> for RpcEconomicalModel {
    fn into(self) -> EconomicalModel {
        match self {
            RpcEconomicalModel::Quota => EconomicalModel::Quota,
            RpcEconomicalModel::Charge => EconomicalModel::Charge,
        }
    }
}
