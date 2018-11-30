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

use jsonrpc_types::rpctypes::EconomicalModel as RpcEconomicalModel;

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
