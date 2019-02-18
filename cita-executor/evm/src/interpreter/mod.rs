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

//! Rust VM implementation

#[macro_use]
mod informant;
mod gasometer;
mod memory;
mod shared_cache;
mod stack;

use self::gasometer::Gasometer;
use self::memory::Memory;
pub use self::shared_cache::SharedCache;
use self::stack::{Stack, VecStack};
use super::call_type::CallType;
use super::error::{Error, Result};
use super::return_data::{GasLeft, ReturnData};
use action_params::{ActionParams, ActionValue};
use bit_set::BitSet;
use evm::{self, CostType};
use ext::{ContractCreateResult, Ext, MessageCallResult};
use instructions::{self, Instruction, InstructionInfo};
use std::cmp;
use std::mem;

use std::marker::PhantomData;
use std::sync::Arc;

use cita_types::{Address, H256, U256, U512};
use util::sha3;

type ProgramCounter = usize;

const ONE: U256 = U256([1, 0, 0, 0]);
const TWO: U256 = U256([2, 0, 0, 0]);
const TWO_POW_5: U256 = U256([0x20, 0, 0, 0]);
const TWO_POW_8: U256 = U256([0x100, 0, 0, 0]);
const TWO_POW_16: U256 = U256([0x1_0000, 0, 0, 0]);
const TWO_POW_24: U256 = U256([0x100_0000, 0, 0, 0]);
const TWO_POW_64: U256 = U256([0, 0x1, 0, 0]); // 0x1 00000000 00000000
const TWO_POW_96: U256 = U256([0, 0x1_0000_0000, 0, 0]); //0x1 00000000 00000000 00000000
const TWO_POW_224: U256 = U256([0, 0, 0, 0x1_0000_0000]); //0x1 00000000 00000000 00000000 00000000 00000000 00000000 00000000
const TWO_POW_248: U256 = U256([0, 0, 0, 0x100_0000_0000_0000]); //0x1 00000000 00000000 00000000 00000000 00000000 00000000 00000000 000000

/// Abstraction over raw vector of Bytes. Easier state management of PC.
struct CodeReader<'a> {
    position: ProgramCounter,
    code: &'a [u8],
}

impl<'a> CodeReader<'a> {
    /// Create new code reader - starting at position 0.
    fn new(code: &'a [u8]) -> Self {
        CodeReader { position: 0, code }
    }

    /// Get `no_of_bytes` from code and convert to U256. Move PC
    fn read(&mut self, no_of_bytes: usize) -> U256 {
        let pos = self.position;
        self.position += no_of_bytes;
        let max = cmp::min(pos + no_of_bytes, self.code.len());
        U256::from(&self.code[pos..max])
    }

    fn len(&self) -> usize {
        self.code.len()
    }
}

enum InstructionResult<Gas> {
    Ok,
    UnusedGas(Gas),
    JumpToPosition(U256),
    StopExecutionNeedsReturn {
        /// Gas left.
        gas: Gas,
        /// Return data offset.
        init_off: U256,
        /// Return data size.
        init_size: U256,
        /// Apply or revert state changes.
        apply: bool,
    },
    StopExecution,
}

/// Intepreter EVM implementation
pub struct Interpreter<Cost: CostType> {
    mem: Vec<u8>,
    cache: Arc<SharedCache>,
    return_data: ReturnData,
    _type: PhantomData<Cost>,
}

impl<Cost: CostType> evm::Evm for Interpreter<Cost> {
    fn exec(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft> {
        self.mem.clear();

        let mut informant = informant::EvmInformant::new(ext.depth());

        let code = &params
            .code
            .as_ref()
            .expect("exec always called with code; qed");
        let valid_jump_destinations = self.cache.jump_destinations(&params.code_hash, code);

        let mut gasometer = Gasometer::<Cost>::new(Cost::from_u256(params.gas)?);
        let mut stack = VecStack::with_capacity(ext.schedule().stack_limit, U256::zero());
        let mut reader = CodeReader::new(code);
        let infos = &*instructions::INSTRUCTIONS;

        while reader.position < code.len() {
            let instruction = code[reader.position];
            reader.position += 1;

            let info = &infos[instruction as usize];
            self.verify_instruction(ext, instruction, info, &stack)?;

            // Calculate gas cost
            let requirements =
                gasometer.requirements(ext, instruction, info, &stack, self.mem.size())?;
            // TODO: make compile-time removable if too much of a performance hit.
            let trace_executed = ext.trace_prepare_execute(
                reader.position - 1,
                instruction,
                &requirements.gas_cost.as_u256(),
            );

            gasometer.verify_gas(&requirements.gas_cost)?;
            self.mem.expand(requirements.memory_required_size);
            gasometer.current_mem_gas = requirements.memory_total_gas;
            gasometer.current_gas = gasometer.current_gas - requirements.gas_cost;

            evm_debug!({
                informant.before_instruction(
                    reader.position,
                    instruction,
                    info,
                    &gasometer.current_gas,
                    &stack,
                )
            });

            let (mem_written, store_written) = if trace_executed {
                (
                    Self::mem_written(instruction, &stack),
                    Self::store_written(instruction, &stack),
                )
            } else {
                (None, None)
            };

            // Execute instruction
            let result = self.exec_instruction(
                gasometer.current_gas,
                params,
                ext,
                instruction,
                &mut reader,
                &mut stack,
                requirements.provide_gas,
            )?;

            evm_debug!({ informant.after_instruction(instruction) });

            if let InstructionResult::UnusedGas(ref gas) = result {
                gasometer.current_gas = gasometer.current_gas + *gas;
            }

            if trace_executed {
                ext.trace_executed(
                    gasometer.current_gas.as_u256(),
                    stack.peek_top(info.ret),
                    mem_written.map(|(o, s)| (o, &(self.mem[o..(o + s)]))),
                    store_written,
                );
            }

            // Advance
            match result {
                InstructionResult::JumpToPosition(position) => {
                    let pos = self.verify_jump(position, &valid_jump_destinations)?;
                    reader.position = pos;
                }
                InstructionResult::StopExecutionNeedsReturn {
                    gas,
                    init_off,
                    init_size,
                    apply,
                } => {
                    informant.done();
                    let mem = mem::replace(&mut self.mem, Vec::new());
                    return Ok(GasLeft::NeedsReturn {
                        gas_left: gas.as_u256(),
                        data: mem.into_return_data(init_off, init_size),
                        apply_state: apply,
                    });
                }
                InstructionResult::StopExecution => break,
                _ => {}
            }
        }
        informant.done();
        Ok(GasLeft::Known(gasometer.current_gas.as_u256()))
    }
}

impl<Cost: CostType> Interpreter<Cost> {
    /// Create a new `Interpreter` instance with shared cache.
    pub fn new(cache: Arc<SharedCache>) -> Interpreter<Cost> {
        Interpreter {
            mem: Vec::new(),
            cache,
            return_data: ReturnData::empty(),
            _type: PhantomData::default(),
        }
    }

    fn verify_instruction(
        &self,
        ext: &Ext,
        instruction: Instruction,
        info: &InstructionInfo,
        stack: &Stack<U256>,
    ) -> Result<()> {
        let schedule = ext.schedule();

        if info.tier == instructions::GasPriceTier::Invalid {
            return Err(Error::BadInstruction { instruction });
        }

        if !stack.has(info.args) {
            Err(Error::StackUnderflow {
                instruction: info.name,
                wanted: info.args,
                on_stack: stack.size(),
            })
        } else if stack.size() - info.args + info.ret > schedule.stack_limit {
            Err(Error::OutOfStack {
                instruction: info.name,
                wanted: info.ret - info.args,
                limit: schedule.stack_limit,
            })
        } else {
            Ok(())
        }
    }

    fn mem_written(instruction: Instruction, stack: &Stack<U256>) -> Option<(usize, usize)> {
        match instruction {
            instructions::MSTORE | instructions::MLOAD => {
                Some((stack.peek(0).low_u64() as usize, 32))
            }
            instructions::MSTORE8 => Some((stack.peek(0).low_u64() as usize, 1)),
            instructions::CALLDATACOPY | instructions::CODECOPY | instructions::RETURNDATACOPY => {
                Some((
                    stack.peek(0).low_u64() as usize,
                    stack.peek(2).low_u64() as usize,
                ))
            }
            instructions::EXTCODECOPY => Some((
                stack.peek(1).low_u64() as usize,
                stack.peek(3).low_u64() as usize,
            )),
            instructions::CALL | instructions::CALLCODE => Some((
                stack.peek(5).low_u64() as usize,
                stack.peek(6).low_u64() as usize,
            )),
            instructions::DELEGATECALL => Some((
                stack.peek(4).low_u64() as usize,
                stack.peek(5).low_u64() as usize,
            )),
            _ => None,
        }
    }

    fn store_written(instruction: Instruction, stack: &Stack<U256>) -> Option<(U256, U256)> {
        match instruction {
            instructions::SSTORE => Some((*stack.peek(0), *stack.peek(1))),
            _ => None,
        }
    }

    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    fn exec_instruction(
        &mut self,
        gas: Cost,
        params: &ActionParams,
        ext: &mut Ext,
        instruction: Instruction,
        code: &mut CodeReader,
        stack: &mut Stack<U256>,
        provided: Option<Cost>,
    ) -> Result<InstructionResult<Cost>> {
        match instruction {
            instructions::JUMP => {
                let jump = stack.pop_back();
                return Ok(InstructionResult::JumpToPosition(jump));
            }
            instructions::JUMPI => {
                let jump = stack.pop_back();
                let condition = stack.pop_back();
                if !self.is_zero(&condition) {
                    return Ok(InstructionResult::JumpToPosition(jump));
                }
            }
            instructions::JUMPDEST => {
                // ignore
            }
            instructions::CREATE => {
                let endowment = stack.pop_back();
                let init_off = stack.pop_back();
                let init_size = stack.pop_back();
                let create_gas = provided.expect("`provided` comes through Self::exec from `Gasometer::get_gas_cost_mem`; `gas_gas_mem_cost` guarantees `Some` when instruction is `CALL`/`CALLCODE`/`DELEGATECALL`/`CREATE`; this is `CREATE`; qed");

                if ext.is_static() {
                    return Err(Error::MutableCallInStaticContext);
                }

                let contract_code = self.mem.read_slice(init_off, init_size);
                let can_create = ext.balance(&params.address)? >= endowment
                    && ext.depth() < ext.schedule().max_depth;

                // clear return data buffer before creating new call frame.
                self.return_data = ReturnData::empty();

                if !can_create {
                    stack.push(U256::zero());
                    return Ok(InstructionResult::UnusedGas(create_gas));
                }

                let create_result = ext.create(&create_gas.as_u256(), &endowment, contract_code);
                return match create_result {
                    ContractCreateResult::Created(address, gas_left) => {
                        stack.push(address_to_u256(address));
                        Ok(InstructionResult::UnusedGas(
                            Cost::from_u256(gas_left).expect("Gas left cannot be greater."),
                        ))
                    }
                    ContractCreateResult::Reverted(gas_left, return_data) => {
                        stack.push(U256::zero());
                        self.return_data = return_data;
                        Ok(InstructionResult::UnusedGas(
                            Cost::from_u256(gas_left).expect("Gas left cannot be greater."),
                        ))
                    }
                    ContractCreateResult::Failed => {
                        stack.push(U256::zero());
                        Ok(InstructionResult::Ok)
                    }
                    ContractCreateResult::FailedInStaticCall => {
                        Err(Error::MutableCallInStaticContext)
                    }
                };
            }
            instructions::CALL
            | instructions::CALLCODE
            | instructions::DELEGATECALL
            | instructions::STATICCALL => {
                assert!(
                    ext.schedule().call_value_transfer_gas > ext.schedule().call_stipend,
                    "overflow possible"
                );
                stack.pop_back();
                let call_gas = provided.expect("`provided` comes through Self::exec from `Gasometer::get_gas_cost_mem`; `gas_gas_mem_cost` guarantees `Some` when instruction is `CALL`/`CALLCODE`/`DELEGATECALL`/`CREATE`; this is one of `CALL`/`CALLCODE`/`DELEGATECALL`; qed");
                let code_address = stack.pop_back();
                let code_address = u256_to_address(&code_address);

                let value = if instruction == instructions::DELEGATECALL {
                    None
                } else if instruction == instructions::STATICCALL {
                    Some(U256::zero())
                } else {
                    Some(stack.pop_back())
                };

                let in_off = stack.pop_back();
                let in_size = stack.pop_back();
                let out_off = stack.pop_back();
                let out_size = stack.pop_back();

                // Add stipend (only CALL|CALLCODE when value > 0)
                let call_gas = call_gas
                    + value.map_or_else(
                        || Cost::from(0),
                        |val| {
                            if val.is_zero() {
                                Cost::from(0)
                            } else {
                                Cost::from(ext.schedule().call_stipend)
                            }
                        },
                    );

                // Get sender & receive addresses, check if we have balance
                let (sender_address, receive_address, has_balance, call_type) = match instruction {
                    instructions::CALL => {
                        if ext.is_static() && value.map_or(false, |v| !v.is_zero()) {
                            return Err(Error::MutableCallInStaticContext);
                        }
                        let has_balance = ext.balance(&params.address)?
                            >= value
                                .expect("value set for all but delegate call and staticcall; qed");
                        (&params.address, &code_address, has_balance, CallType::Call)
                    }
                    instructions::CALLCODE => {
                        let has_balance = ext.balance(&params.address)?
                            >= value
                                .expect("value set for all but delegate call and staticcall; qed");
                        (
                            &params.address,
                            &params.address,
                            has_balance,
                            CallType::CallCode,
                        )
                    }
                    instructions::DELEGATECALL => (
                        &params.sender,
                        &params.address,
                        true,
                        CallType::DelegateCall,
                    ),
                    instructions::STATICCALL => {
                        (&params.address, &code_address, true, CallType::StaticCall)
                    }
                    _ => panic!(format!(
                        "Unexpected instruction {} in CALL branch.",
                        instruction
                    )),
                };

                // clear return data buffer before creating new call frame.
                self.return_data = ReturnData::empty();

                let can_call = has_balance && ext.depth() < ext.schedule().max_depth;
                if !can_call {
                    stack.push(U256::zero());
                    return Ok(InstructionResult::UnusedGas(call_gas));
                }

                let call_result = {
                    // we need to write and read from memory in the same time
                    // and we don't want to copy
                    let input = unsafe { &*(self.mem.read_slice(in_off, in_size) as *const [u8]) };
                    let output = self.mem.writeable_slice(out_off, out_size);
                    ext.call(
                        &call_gas.as_u256(),
                        sender_address,
                        receive_address,
                        value,
                        input,
                        &code_address,
                        output,
                        call_type,
                    )
                };

                return match call_result {
                    MessageCallResult::Success(gas_left, data) => {
                        stack.push(U256::one());
                        self.return_data = data;
                        Ok(InstructionResult::UnusedGas(
                            Cost::from_u256(gas_left)
                                .expect("Gas left cannot be greater than current one"),
                        ))
                    }
                    MessageCallResult::Reverted(gas_left, data) => {
                        stack.push(U256::zero());
                        self.return_data = data;
                        Ok(InstructionResult::UnusedGas(
                            Cost::from_u256(gas_left)
                                .expect("Gas left cannot be greater then current one"),
                        ))
                    }
                    MessageCallResult::Failed => {
                        stack.push(U256::zero());
                        Ok(InstructionResult::Ok)
                    }
                };
            }
            instructions::RETURN => {
                let init_off = stack.pop_back();
                let init_size = stack.pop_back();

                return Ok(InstructionResult::StopExecutionNeedsReturn {
                    gas,
                    init_off,
                    init_size,
                    apply: true,
                });
            }
            instructions::REVERT => {
                let init_off = stack.pop_back();
                let init_size = stack.pop_back();

                return Ok(InstructionResult::StopExecutionNeedsReturn {
                    gas,
                    init_off,
                    init_size,
                    apply: false,
                });
            }
            instructions::STOP => {
                return Ok(InstructionResult::StopExecution);
            }
            instructions::SUICIDE => {
                let address = stack.pop_back();
                ext.suicide(&u256_to_address(&address))?;
                return Ok(InstructionResult::StopExecution);
            }
            instructions::LOG0...instructions::LOG4 => {
                let no_of_topics = instructions::get_log_topics(instruction);

                let offset = stack.pop_back();
                let size = stack.pop_back();
                let topics = stack.pop_n(no_of_topics).iter().map(H256::from).collect();
                ext.log(topics, self.mem.read_slice(offset, size))?;
            }
            instructions::PUSH1...instructions::PUSH32 => {
                let bytes = instructions::get_push_bytes(instruction);
                let val = code.read(bytes);
                stack.push(val);
            }
            instructions::MLOAD => {
                let word = self.mem.read(stack.pop_back());
                stack.push(word);
            }
            instructions::MSTORE => {
                let offset = stack.pop_back();
                let word = stack.pop_back();
                Memory::write(&mut self.mem, offset, word);
            }
            instructions::MSTORE8 => {
                let offset = stack.pop_back();
                let byte = stack.pop_back();
                self.mem.write_byte(offset, byte);
            }
            instructions::MSIZE => {
                stack.push(U256::from(self.mem.size()));
            }
            instructions::SHA3 => {
                let offset = stack.pop_back();
                let size = stack.pop_back();
                let hash = U256::from(H256::from_slice(&sha3::keccak256(
                    self.mem.read_slice(offset, size),
                )));
                stack.push(hash);
            }
            instructions::SLOAD => {
                let key = H256::from(&stack.pop_back());
                let word = U256::from(&*ext.storage_at(&key)?);
                stack.push(word);
            }
            instructions::SSTORE => {
                let address = H256::from(&stack.pop_back());
                let val = stack.pop_back();

                let current_val = U256::from(&*ext.storage_at(&address)?);
                // Increase refund for clear
                if !self.is_zero(&current_val) && self.is_zero(&val) {
                    ext.inc_sstore_clears();
                }
                ext.set_storage(address, H256::from(&val))?;
            }
            instructions::PC => {
                stack.push(U256::from(code.position - 1));
            }
            instructions::GAS => {
                stack.push(gas.as_u256());
            }
            instructions::ADDRESS => {
                stack.push(address_to_u256(params.address));
            }
            instructions::ORIGIN => {
                stack.push(address_to_u256(params.origin));
            }
            instructions::BALANCE => {
                let address = u256_to_address(&stack.pop_back());
                let balance = ext.balance(&address)?;
                stack.push(balance);
            }
            instructions::CALLER => {
                stack.push(address_to_u256(params.sender));
            }
            instructions::CALLVALUE => {
                stack.push(match params.value {
                    ActionValue::Transfer(val) | ActionValue::Apparent(val) => val,
                });
            }
            instructions::CALLDATALOAD => {
                let big_id = stack.pop_back();
                let id = big_id.low_u64() as usize;
                let max = id.wrapping_add(32);
                if let Some(data) = params.data.as_ref() {
                    let bound = cmp::min(data.len(), max);
                    if id < bound && big_id < U256::from(data.len()) {
                        let mut v = [0u8; 32];
                        v[0..bound - id].clone_from_slice(&data[id..bound]);
                        stack.push(U256::from(&v[..]))
                    } else {
                        stack.push(U256::zero())
                    }
                } else {
                    stack.push(U256::zero())
                }
            }
            instructions::CALLDATASIZE => {
                stack.push(U256::from(params.data.clone().map_or(0, |l| l.len())));
            }
            instructions::CODESIZE => {
                stack.push(U256::from(code.len()));
            }
            instructions::RETURNDATASIZE => stack.push(U256::from(self.return_data.len())),
            instructions::EXTCODESIZE => {
                let address = u256_to_address(&stack.pop_back());
                let len = ext.extcodesize(&address)?;
                stack.push(U256::from(len));
            }
            instructions::CALLDATACOPY => {
                Self::copy_data_to_memory(
                    &mut self.mem,
                    stack,
                    params
                        .data
                        .as_ref()
                        .map_or_else(|| &[] as &[u8], |d| &*d as &[u8]),
                );
            }
            instructions::RETURNDATACOPY => {
                {
                    let source_offset = stack.peek(1);
                    let size = stack.peek(2);
                    let return_data_len = U256::from(self.return_data.len());
                    if source_offset.overflow_add(*size).0 > return_data_len {
                        return Err(Error::OutOfBounds);
                    }
                }
                Self::copy_data_to_memory(&mut self.mem, stack, &*self.return_data);
            }
            instructions::CODECOPY => {
                Self::copy_data_to_memory(
                    &mut self.mem,
                    stack,
                    params
                        .code
                        .as_ref()
                        .map_or_else(|| &[] as &[u8], |c| &**c as &[u8]),
                );
            }
            instructions::EXTCODECOPY => {
                let address = u256_to_address(&stack.pop_back());
                let code = ext.extcode(&address)?;
                Self::copy_data_to_memory(&mut self.mem, stack, &code);
            }
            instructions::GASPRICE => {
                stack.push(params.gas_price);
            }
            instructions::BLOCKHASH => {
                let block_number = stack.pop_back();
                let block_hash = ext.blockhash(&block_number);
                stack.push(U256::from(&*block_hash));
            }
            instructions::COINBASE => {
                stack.push(address_to_u256(ext.env_info().author));
            }
            instructions::TIMESTAMP => {
                stack.push(U256::from(ext.env_info().timestamp));
            }
            instructions::NUMBER => {
                stack.push(U256::from(ext.env_info().number));
            }
            instructions::DIFFICULTY => {
                stack.push(ext.env_info().difficulty);
            }
            instructions::GASLIMIT => {
                stack.push(ext.env_info().gas_limit);
            }
            _ => {
                self.exec_stack_instruction(instruction, stack)?;
            }
        };
        Ok(InstructionResult::Ok)
    }

    fn copy_data_to_memory(mem: &mut Vec<u8>, stack: &mut Stack<U256>, source: &[u8]) {
        let dest_offset = stack.pop_back();
        let source_offset = stack.pop_back();
        let size = stack.pop_back();
        let source_size = U256::from(source.len());

        let output_end = if source_offset > source_size
            || size > source_size
            || source_offset + size > source_size
        {
            let zero_slice = if source_offset > source_size {
                mem.writeable_slice(dest_offset, size)
            } else {
                mem.writeable_slice(
                    dest_offset + source_size - source_offset,
                    source_offset + size - source_size,
                )
            };
            for i in zero_slice.iter_mut() {
                *i = 0;
            }
            source.len()
        } else {
            (size.low_u64() + source_offset.low_u64()) as usize
        };

        if source_offset < source_size {
            let output_begin = source_offset.low_u64() as usize;
            mem.write_slice(dest_offset, &source[output_begin..output_end]);
        }
    }

    fn verify_jump(&self, jump_u: U256, valid_jump_destinations: &BitSet) -> Result<usize> {
        let jump = jump_u.low_u64() as usize;

        if valid_jump_destinations.contains(jump) && U256::from(jump) == jump_u {
            Ok(jump)
        } else {
            Err(Error::BadJumpDestination { destination: jump })
        }
    }

    fn is_zero(&self, val: &U256) -> bool {
        val.is_zero()
    }

    fn bool_to_u256(&self, val: bool) -> U256 {
        if val {
            U256::one()
        } else {
            U256::zero()
        }
    }

    fn exec_stack_instruction(
        &self,
        instruction: Instruction,
        stack: &mut Stack<U256>,
    ) -> Result<()> {
        match instruction {
            instructions::DUP1...instructions::DUP16 => {
                let position = instructions::get_dup_position(instruction);
                let val = *stack.peek(position);
                stack.push(val);
            }
            instructions::SWAP1...instructions::SWAP16 => {
                let position = instructions::get_swap_position(instruction);
                stack.swap_with_top(position)
            }
            instructions::POP => {
                stack.pop_back();
            }
            instructions::ADD => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a.overflowing_add(b).0);
            }
            instructions::MUL => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a.overflowing_mul(b).0);
            }
            instructions::SUB => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a.overflowing_sub(b).0);
            }
            instructions::DIV => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(if !self.is_zero(&b) {
                    match b {
                        ONE => a,
                        TWO => a >> 1,
                        TWO_POW_5 => a >> 5,
                        TWO_POW_8 => a >> 8,
                        TWO_POW_16 => a >> 16,
                        TWO_POW_24 => a >> 24,
                        TWO_POW_64 => a >> 64,
                        TWO_POW_96 => a >> 96,
                        TWO_POW_224 => a >> 224,
                        TWO_POW_248 => a >> 248,
                        _ => a.overflowing_div(b).0,
                    }
                } else {
                    U256::zero()
                });
            }
            instructions::MOD => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(if !self.is_zero(&b) {
                    a.overflowing_rem(b).0
                } else {
                    U256::zero()
                });
            }
            instructions::SDIV => {
                let (a, sign_a) = get_and_reset_sign(stack.pop_back());
                let (b, sign_b) = get_and_reset_sign(stack.pop_back());

                // -2^255
                let min = (U256::one() << 255) - U256::one();
                stack.push(if self.is_zero(&b) {
                    U256::zero()
                } else if a == min && b == !U256::zero() {
                    min
                } else {
                    let c = a.overflowing_div(b).0;
                    set_sign(c, sign_a ^ sign_b)
                });
            }
            instructions::SMOD => {
                let ua = stack.pop_back();
                let ub = stack.pop_back();
                let (a, sign_a) = get_and_reset_sign(ua);
                let b = get_and_reset_sign(ub).0;

                stack.push(if !self.is_zero(&b) {
                    let c = a.overflowing_rem(b).0;
                    set_sign(c, sign_a)
                } else {
                    U256::zero()
                });
            }
            instructions::EXP => {
                let base = stack.pop_back();
                let expon = stack.pop_back();
                let res = base.overflowing_pow(expon).0;
                stack.push(res);
            }
            instructions::NOT => {
                let a = stack.pop_back();
                stack.push(!a);
            }
            instructions::LT => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(self.bool_to_u256(a < b));
            }
            instructions::SLT => {
                let (a, neg_a) = get_and_reset_sign(stack.pop_back());
                let (b, neg_b) = get_and_reset_sign(stack.pop_back());

                let is_positive_lt = a < b && !(neg_a | neg_b);
                let is_negative_lt = a > b && (neg_a & neg_b);
                let has_different_signs = neg_a && !neg_b;

                stack
                    .push(self.bool_to_u256(is_positive_lt | is_negative_lt | has_different_signs));
            }
            instructions::GT => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(self.bool_to_u256(a > b));
            }
            instructions::SGT => {
                let (a, neg_a) = get_and_reset_sign(stack.pop_back());
                let (b, neg_b) = get_and_reset_sign(stack.pop_back());

                let is_positive_gt = a > b && !(neg_a | neg_b);
                let is_negative_gt = a < b && (neg_a & neg_b);
                let has_different_signs = !neg_a && neg_b;

                stack
                    .push(self.bool_to_u256(is_positive_gt | is_negative_gt | has_different_signs));
            }
            instructions::EQ => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(self.bool_to_u256(a == b));
            }
            instructions::ISZERO => {
                let a = stack.pop_back();
                stack.push(self.bool_to_u256(self.is_zero(&a)));
            }
            instructions::AND => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a & b);
            }
            instructions::OR => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a | b);
            }
            instructions::XOR => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                stack.push(a ^ b);
            }
            instructions::BYTE => {
                let word = stack.pop_back();
                let val = stack.pop_back();
                let byte = if word < U256::from(32) {
                    (val >> (8 * (31 - word.low_u64() as usize))) & U256::from(0xff)
                } else {
                    U256::zero()
                };
                stack.push(byte);
            }
            instructions::ADDMOD => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                let c = stack.pop_back();

                stack.push(if !self.is_zero(&c) {
                    // upcast to 512
                    let a5 = U512::from(a);
                    let res = a5.overflowing_add(U512::from(b)).0;
                    let x = res.overflowing_rem(U512::from(c)).0;
                    U256::from(x)
                } else {
                    U256::zero()
                });
            }
            instructions::MULMOD => {
                let a = stack.pop_back();
                let b = stack.pop_back();
                let c = stack.pop_back();

                stack.push(if !self.is_zero(&c) {
                    let a5 = U512::from(a);
                    let res = a5.overflowing_mul(U512::from(b)).0;
                    let x = res.overflowing_rem(U512::from(c)).0;
                    U256::from(x)
                } else {
                    U256::zero()
                });
            }
            instructions::SIGNEXTEND => {
                let bit = stack.pop_back();
                if bit < U256::from(32) {
                    let number = stack.pop_back();
                    let bit_position = (bit.low_u64() * 8 + 7) as usize;

                    let bit = number.bit(bit_position);
                    let mask = (U256::one() << bit_position) - U256::one();
                    stack.push(if bit { number | !mask } else { number & mask });
                }
            }
            instructions::SHL => {
                const CONST_256: U256 = U256([256, 0, 0, 0]);

                let shift = stack.pop_back();
                let value = stack.pop_back();

                let result = if shift >= CONST_256 {
                    U256::zero()
                } else {
                    value << (shift.as_u32() as usize)
                };
                stack.push(result);
            }
            instructions::SHR => {
                const CONST_256: U256 = U256([256, 0, 0, 0]);

                let shift = stack.pop_back();
                let value = stack.pop_back();

                let result = if shift >= CONST_256 {
                    U256::zero()
                } else {
                    value >> (shift.as_u32() as usize)
                };
                stack.push(result);
            }
            instructions::SAR => {
                // We cannot use get_and_reset_sign/set_sign here, because the rounding looks different.

                const CONST_256: U256 = U256([256, 0, 0, 0]);
                const CONST_HIBIT: U256 = U256([0, 0, 0, 0x8000_0000_0000_0000]);

                let shift = stack.pop_back();
                let value = stack.pop_back();
                let sign = value & CONST_HIBIT != U256::zero();

                let result = if shift >= CONST_256 {
                    if sign {
                        U256::max_value()
                    } else {
                        U256::zero()
                    }
                } else {
                    let shift = shift.as_u32() as usize;
                    let mut shifted = value >> shift;
                    if sign {
                        shifted = shifted | (U256::max_value() << (256 - shift));
                    }
                    shifted
                };
                stack.push(result);
            }
            _ => {
                return Err(Error::BadInstruction { instruction });
            }
        }
        Ok(())
    }
}

fn get_and_reset_sign(value: U256) -> (U256, bool) {
    let U256(arr) = value;
    let sign = arr[3].leading_zeros() == 0;
    (set_sign(value, sign), sign)
}

fn set_sign(value: U256, sign: bool) -> U256 {
    if sign {
        (!U256::zero() ^ value).overflowing_add(U256::one()).0
    } else {
        value
    }
}

#[inline]
fn u256_to_address(value: &U256) -> Address {
    Address::from(H256::from(value))
}

#[inline]
fn address_to_u256(value: Address) -> U256 {
    U256::from(&*H256::from(value))
}
