// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Cost schedule and other parameterisations for the EVM.

/// Definition of the cost schedule and other parameterisations for the EVM.
pub struct Schedule {
    /// VM stack limit
    pub stack_limit: usize,
    /// Max number of nested calls/creates
    pub max_depth: usize,
    /// Gas prices for instructions in all tiers
    pub tier_step_gas: [usize; 8],
    /// Gas price for `EXP` opcode
    pub exp_gas: usize,
    /// Additional gas for `EXP` opcode for each byte of exponent
    pub exp_byte_gas: usize,
    /// Gas price for `SHA3` opcode
    pub sha3_gas: usize,
    /// Additional gas for `SHA3` opcode for each word of hashed memory
    pub sha3_word_gas: usize,
    /// Gas price for loading from storage
    pub sload_gas: usize,
    /// Gas price for setting new value to storage (`storage==0`, `new!=0`)
    pub sstore_set_gas: usize,
    /// Gas price for altering value in storage
    pub sstore_reset_gas: usize,
    /// Gas refund for `SSTORE` clearing (when `storage!=0`, `new==0`)
    pub sstore_refund_gas: usize,
    /// Gas price for `JUMPDEST` opcode
    pub jumpdest_gas: usize,
    /// Gas price for `LOG*`
    pub log_gas: usize,
    /// Additional gas for data in `LOG*`
    pub log_data_gas: usize,
    /// Additional gas for each topic in `LOG*`
    pub log_topic_gas: usize,
    /// Gas price for `CREATE` opcode
    pub create_gas: usize,
    /// Gas price for `*CALL*` opcodes
    pub call_gas: usize,
    /// Stipend for transfer for `CALL|CALLCODE` opcode when `value>0`
    pub call_stipend: usize,
    /// Additional gas required for value transfer (`CALL|CALLCODE`)
    pub call_value_transfer_gas: usize,
    /// Additional gas for creating new account (`CALL|CALLCODE`)
    pub call_new_account_gas: usize,
    /// Refund for SUICIDE
    pub suicide_refund_gas: usize,
    /// Gas for used memory
    pub memory_gas: usize,
    /// Coefficient used to convert memory size to gas price for memory
    pub quad_coeff_div: usize,
    /// Cost for contract length when executing `CREATE`
    pub create_data_gas: usize,
    /// Maximum code size when creating a contract.
    pub create_data_limit: usize,
    /// Transaction cost
    pub tx_gas: usize,
    /// `CREATE` transaction cost
    pub tx_create_gas: usize,
    /// Additional cost for empty data transaction
    pub tx_data_zero_gas: usize,
    /// Aditional cost for non-empty data transaction
    pub tx_data_non_zero_gas: usize,
    /// Gas price for copying memory
    pub copy_gas: usize,
    /// Price of EXTCODESIZE
    pub extcodesize_gas: usize,
    /// Base price of EXTCODECOPY
    pub extcodecopy_base_gas: usize,
    /// Price of BALANCE
    pub balance_gas: usize,
    /// Price of SUICIDE
    pub suicide_gas: usize,
    /// Amount of additional gas to pay when SUICIDE credits a non-existant account
    pub suicide_to_new_account_cost: usize,
    /// If Some(x): let limit = GAS * (x - 1) / x; let CALL's gas = min(requested, limit). let CREATE's gas = limit.
    /// If None: let CALL's gas = (requested > GAS ? [OOG] : GAS). let CREATE's gas = GAS
    pub sub_gas_cap_divisor: usize,
    /// Don't ever make empty accounts; contracts start with nonce=1. Also, don't charge 25k when sending/suicide zero-value.
    pub no_empty: bool,
    /// Kill empty accounts if touched.
    pub kill_empty: bool,
}

impl Schedule {
    /// Schedule for the v1 of the cita main net.
    pub fn new_v1() -> Schedule {
        Self::new()
    }

    fn new() -> Schedule {
        Schedule {
            stack_limit: 1024,
            max_depth: 1024,
            tier_step_gas: [0, 2, 3, 5, 8, 10, 20, 0],
            exp_gas: 10,
            exp_byte_gas: 10,
            sha3_gas: 30,
            sha3_word_gas: 6,
            sload_gas: 50,
            sstore_set_gas: 20_000,
            sstore_reset_gas: 5000,
            sstore_refund_gas: 15_000,
            jumpdest_gas: 1,
            log_gas: 375,
            log_data_gas: 8,
            log_topic_gas: 375,
            create_gas: 32_000,
            call_gas: 40,
            call_stipend: 2300,
            call_value_transfer_gas: 9000,
            call_new_account_gas: 25_000,
            suicide_refund_gas: 24_000,
            memory_gas: 3,
            quad_coeff_div: 512,
            create_data_gas: 200,
            create_data_limit: usize::max_value(),
            tx_gas: 21_000,
            tx_create_gas: 53_000,
            tx_data_zero_gas: 4,
            tx_data_non_zero_gas: 68,
            copy_gas: 3,
            extcodesize_gas: 20,
            extcodecopy_base_gas: 20,
            balance_gas: 20,
            suicide_gas: 0,
            suicide_to_new_account_cost: 0,
            sub_gas_cap_divisor: 64,
            no_empty: false,
            kill_empty: false,
        }
    }
}

#[test]
#[cfg(test)]
fn schedule_evm_assumptions() {
    let s1 = Schedule::new_v1();

    // To optimize division we assume 2**9 for quad_coeff_div
    assert_eq!(s1.quad_coeff_div, 512);
}
