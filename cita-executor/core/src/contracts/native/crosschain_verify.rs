use crate::contracts::{
    native::factory::Contract, solc::ChainManagement, tools::method as method_tools,
};
use crate::state::StateProof;
use cita_types::{Address, H256, U256};
use core::header::Header;
use core::libchain::chain::TxProof;
use ethabi;
use evm::action_params::ActionParams;
use evm::storage::Map;

use cita_vm::evm::DataProvider;
use cita_vm::evm::Error as EVMError;
use cita_vm::evm::InterpreterResult;

lazy_static! {
    static ref VERIFY_TRANSACTION_FUNC: u32 =
        method_tools::encode_to_u32(b"verifyTransaction(address,bytes4,uint64,bytes)");
    static ref VERIFY_STATE_FUNC: u32 =
        method_tools::encode_to_u32(b"verifyState(uint256,uint64,bytes)");
    static ref VERIFY_BLOCK_HEADER_FUNC: u32 =
        method_tools::encode_to_u32(b"verifyBlockHeader(uint256,bytes)");
    static ref GET_EXPECTED_BLOCK_NUMBER_FUNC: u32 =
        method_tools::encode_to_u32(b"getExpectedBlockNumber(uint256)");
}

#[derive(Clone)]
pub struct CrossChainVerify {
    block_headers: Map,
    state_roots: Map,
    output: Vec<u8>,
}

impl Contract for CrossChainVerify {
    fn exec(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError> {
        if let Some(ref data) = params.data {
            method_tools::extract_to_u32(&data[..]).and_then(|signature| match signature {
                sig if sig == *VERIFY_TRANSACTION_FUNC => self.verify_transaction(params, ext),
                sig if sig == *VERIFY_STATE_FUNC => self.verify_state(params, ext),
                sig if sig == *VERIFY_BLOCK_HEADER_FUNC => self.verify_block_header(params, ext),
                sig if sig == *GET_EXPECTED_BLOCK_NUMBER_FUNC => {
                    self.get_expected_block_number(params, ext)
                }
                _ => Err(EVMError::OutOfGas),
            })
        } else {
            Err(EVMError::OutOfGas)
        }
    }
    fn create(&self) -> Box<Contract> {
        Box::new(CrossChainVerify::default())
    }
}

impl Default for CrossChainVerify {
    fn default() -> Self {
        CrossChainVerify {
            block_headers: Map::new(H256::from(0)),
            state_roots: Map::new(H256::from(1)),
            output: Vec::new(),
        }
    }
}

impl CrossChainVerify {
    fn verify_transaction(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(EVMError::OutOfGas);
        }
        let gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(EVMError::Internal("no data".to_string()));
        }

        let data = params.data.to_owned().unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![
            ethabi::ParamType::Address,
            ethabi::ParamType::FixedBytes(4),
            ethabi::ParamType::Uint(64),
            ethabi::ParamType::Bytes,
        ];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(EVMError::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_address();
        if result.is_none() {
            return Err(EVMError::Internal("decode 1st param failed".to_string()));
        }
        let addr = Address::from(result.unwrap());
        trace!("addr = {}", addr);
        let result = decoded.remove(0).to_fixed_bytes();
        if result.is_none() {
            return Err(EVMError::Internal("decode 2nd param failed".to_string()));
        }
        let hasher = result.unwrap()[..4].iter().take(4).enumerate().fold(
            [0u8; 4],
            |mut acc, (idx, val)| {
                acc[idx] = *val;
                acc
            },
        );
        trace!("hasher = {:?}", hasher);
        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(EVMError::Internal("decode 3rd param failed".to_string()));
        }
        let nonce = U256::from_big_endian(&result.unwrap()).low_u64();
        trace!("nonce = {}", nonce);
        let result = decoded.remove(0).to_bytes();
        if result.is_none() {
            return Err(EVMError::Internal("decode 4th param failed".to_string()));
        }
        let proof_data = result.unwrap();
        trace!("data = {:?}", proof_data);

        let proof = TxProof::from_bytes(&proof_data);

        let relay_info = proof.extract_relay_info();
        if relay_info.is_none() {
            return Err(EVMError::Internal("extract relay info failed".to_string()));
        }
        let relay_info = relay_info.unwrap();
        trace!("relay_info {:?}", proof_data);

        let ret = ChainManagement::ext_chain_id(ext, &gas_left, &params.sender);
        if ret.is_none() {
            return Err(EVMError::Internal("get chain id failed".to_owned()));
        }
        let (gas_left, chain_id) = ret.unwrap();

        let ret = ChainManagement::ext_authorities(
            ext,
            &gas_left,
            &params.sender,
            relay_info.from_chain_id,
        );
        if ret.is_none() {
            return Err(EVMError::Internal("get authorities failed".to_owned()));
        }
        let (gas_left, authorities) = ret.unwrap();

        let ret = proof.extract_crosschain_data(addr, hasher, nonce, chain_id, &authorities[..]);
        if ret.is_none() {
            return Err(EVMError::Internal(
                "extract_crosschain_data failed".to_string(),
            ));
        }
        let (sender, tx_data) = ret.unwrap();

        let tokens = vec![
            ethabi::Token::Address(sender.into()),
            ethabi::Token::Bytes(tx_data.clone()),
        ];
        let result = ethabi::encode(&tokens);
        trace!("encoded {:?}", result);

        self.output = result;
        Ok(InterpreterResult::Normal(
            self.output.clone(),
            gas_left.low_u64(),
            vec![],
        ))
    }

    fn verify_state(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(EVMError::OutOfGas);
        }
        let gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(EVMError::Internal("no data".to_string()));
        }

        let data = params.data.to_owned().unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![
            ethabi::ParamType::Uint(32),
            ethabi::ParamType::Uint(64),
            ethabi::ParamType::Bytes,
        ];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(EVMError::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(EVMError::Internal("decode 1th param failed".to_string()));
        }
        let chain_id = U256::from_big_endian(&result.unwrap());
        trace!("chain_id = {}", chain_id);

        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(EVMError::Internal("decode 2nd param failed".to_string()));
        }
        let block_number = U256::from_big_endian(&result.unwrap()).low_u64();
        trace!("block_number = {}", block_number);

        let result = self.state_roots.get_array(&chain_id).unwrap().get(
            ext,
            &params.code_address,
            block_number,
        );
        if result.is_err() {
            return Err(EVMError::Internal("get state root failed".to_string()));
        }
        let result1 = self.state_roots.get_array(&chain_id).unwrap().get(
            ext,
            &params.code_address,
            block_number + 1,
        );
        if result1.is_err() {
            return Err(EVMError::Internal("get next state root failed".to_string()));
        }
        let state_root: H256 = result.unwrap().into();
        trace!("state_root = {:?}", state_root);
        let next_state_root: H256 = result1.unwrap().into();
        trace!("next_state_root = {:?}", next_state_root);
        if state_root == H256::zero() || next_state_root == H256::zero() {
            return Err(EVMError::Internal(
                "state root have not confirmed".to_string(),
            ));
        }

        let result = decoded.remove(0).to_bytes();
        if result.is_none() {
            return Err(EVMError::Internal("decode 3rd param failed".to_string()));
        }
        let state_proof_bytes = result.unwrap();
        trace!("state_proof_bytes = {:?}", state_proof_bytes);

        let state_proof = StateProof::from_bytes(&state_proof_bytes);
        let maybe_val = state_proof.verify(state_root);
        if maybe_val.is_none() {
            return Err(EVMError::Internal("state proof verify failed".to_string()));
        }
        let val = maybe_val.unwrap();
        trace!("val = {:?}", val);

        let tokens = vec![
            ethabi::Token::Address((*state_proof.address()).into()),
            ethabi::Token::Uint((*state_proof.key()).into()),
            ethabi::Token::Uint(val.into()),
        ];
        let result = ethabi::encode(&tokens);
        trace!("encoded {:?}", result);

        self.output = result;
        Ok(InterpreterResult::Normal(
            self.output.clone(),
            gas_left.low_u64(),
            vec![],
        ))
    }

    fn verify_block_header(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(EVMError::OutOfGas);
        }
        let mut gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(EVMError::Internal("no data".to_string()));
        }

        let data = params.data.to_owned().unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![ethabi::ParamType::Uint(32), ethabi::ParamType::Bytes];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(EVMError::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(EVMError::Internal("decode 1th param failed".to_string()));
        }
        let chain_id = U256::from_big_endian(&result.unwrap());
        trace!("chain_id = {}", chain_id);
        let result = decoded.remove(0).to_bytes();
        if result.is_none() {
            return Err(EVMError::Internal("decode 2nd param failed".to_string()));
        }
        let block_header_curr_bytes = result.unwrap();
        trace!("data = {:?}", block_header_curr_bytes);
        let block_header_curr = Header::from_bytes(&block_header_curr_bytes);

        let block_header_prev_bytes: Vec<u8> =
            self.block_headers
                .get_bytes(ext, &params.code_address, &chain_id)?;

        let verify_result = if block_header_prev_bytes.is_empty() {
            trace!("sync first block header");
            block_header_curr.number() == 0
        } else {
            let block_header_prev = Header::from_bytes(&block_header_prev_bytes);

            let ret = ChainManagement::ext_authorities(ext, &gas_left, &params.sender, chain_id);
            if ret.is_none() {
                return Err(EVMError::Internal("get authorities failed".to_owned()));
            }
            let (gas_left_new, authorities) = ret.unwrap();
            gas_left = gas_left_new;

            block_header_prev.verify_next(&block_header_curr, &authorities[..])
        };

        if verify_result {
            trace!("store the {} block header", block_header_curr.number());
            self.block_headers.set_bytes(
                ext,
                &params.code_address,
                &chain_id,
                &block_header_curr_bytes,
            )?;
            trace!(
                "store the {} block state root {}",
                block_header_curr.number(),
                block_header_curr.state_root()
            );
            self.state_roots.get_array(&chain_id).unwrap().set(
                ext,
                &params.code_address,
                block_header_curr.number(),
                &U256::from(block_header_curr.state_root()),
            )?;
        }

        let tokens = vec![ethabi::Token::Bool(verify_result)];
        let result = ethabi::encode(&tokens);
        trace!("encoded {:?}", result);

        self.output = result;
        Ok(InterpreterResult::Normal(
            self.output.clone(),
            gas_left.low_u64(),
            vec![],
        ))
    }

    fn get_expected_block_number(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(EVMError::OutOfGas);
        }
        let gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(EVMError::Internal("no data".to_string()));
        }

        let data = params.data.to_owned().unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![ethabi::ParamType::Uint(32)];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(EVMError::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(EVMError::Internal("decode 1th param failed".to_string()));
        }
        let chain_id = U256::from_big_endian(&result.unwrap());
        trace!("chain_id = {}", chain_id);

        let block_header_bytes: Vec<u8> =
            self.block_headers
                .get_bytes(ext, &params.code_address, &chain_id)?;

        let block_number = if block_header_bytes.is_empty() {
            0
        } else {
            let block_header = Header::from_bytes(&block_header_bytes);
            block_header.number() + 1
        };
        trace!("block_number = {}", block_number);

        let tokens = vec![ethabi::Token::Uint(U256::from(block_number).into())];
        let result = ethabi::encode(&tokens);
        trace!("encoded {:?}", result);

        self.output = result;
        Ok(InterpreterResult::Normal(
            self.output.clone(),
            gas_left.low_u64(),
            vec![],
        ))
    }
}
