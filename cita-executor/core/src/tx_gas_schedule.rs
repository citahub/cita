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

/// Definition of the cost schedule for a transaction.
pub struct TxGasSchedule {
    /// Transaction cost
    pub tx_gas: usize,
    /// `CREATE` transaction cost
    pub tx_create_gas: usize,
    /// Additional cost for empty data transaction
    pub tx_data_zero_gas: usize,
    /// Aditional cost for non-empty data transaction
    pub tx_data_non_zero_gas: usize,
    /// Cost for contract length when executing `CREATE`
    pub create_data_gas: usize,
}

impl Default for TxGasSchedule {
    fn default() -> Self {
        TxGasSchedule {
            tx_gas: 21_000,
            tx_create_gas: 53_000,
            tx_data_zero_gas: 4,
            tx_data_non_zero_gas: 68,
            create_data_gas: 200,
        }
    }
}
