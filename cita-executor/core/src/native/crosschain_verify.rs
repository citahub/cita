use cita_types::{Address, H256, U256};
use contracts::ChainManagement;
use core::header::Header;
use core::libchain::chain::TxProof;
use ethabi;
use evm::action_params::ActionParams;
use evm::storage::Scalar;
use evm::{Error, Ext, GasLeft, ReturnData};
use native::{calc_func_sig, extract_func_sig, factory::Contract};

lazy_static! {
    static ref VERIFY_TRANSACTION_FUNC: u32 =
        calc_func_sig(b"verifyTransaction(address,bytes4,uint64,bytes)");
    static ref VERIFY_STATE_FUNC: u32 = calc_func_sig(b"verifyState(bytes)");
    static ref VERIFY_BLOCK_HEADER_FUNC: u32 = calc_func_sig(b"verifyBlockHeader(uint32,bytes)");
}

#[derive(Clone)]
pub struct CrossChainVerify {
    block_header: Scalar,
    output: Vec<u8>,
}

impl Contract for CrossChainVerify {
    fn exec(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, Error> {
        extract_func_sig(&params).and_then(|signature| match signature {
            sig if sig == *VERIFY_TRANSACTION_FUNC => self.verify_transaction(params, ext),
            sig if sig == *VERIFY_STATE_FUNC => self.verify_state(params, ext),
            sig if sig == *VERIFY_BLOCK_HEADER_FUNC => self.verify_block_header(params, ext),
            _ => Err(Error::OutOfGas),
        })
    }
    fn create(&self) -> Box<Contract> {
        Box::new(CrossChainVerify::default())
    }
}

impl Default for CrossChainVerify {
    fn default() -> Self {
        CrossChainVerify {
            block_header: Scalar::new(H256::from(0)),
            output: Vec::new(),
        }
    }
}

impl CrossChainVerify {
    fn verify_transaction(
        &mut self,
        params: ActionParams,
        ext: &mut Ext,
    ) -> Result<GasLeft, Error> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(Error::OutOfGas);
        }
        let gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(Error::Internal("no data".to_string()));
        }

        let data = params.data.unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![
            ethabi::ParamType::Address,
            ethabi::ParamType::FixedBytes(4),
            ethabi::ParamType::Uint(64),
            ethabi::ParamType::Bytes,
        ];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(Error::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_address();
        if result.is_none() {
            return Err(Error::Internal("decode 1st param failed".to_string()));
        }
        let addr = Address::from(result.unwrap());
        trace!("addr = {}", addr);
        let result = decoded.remove(0).to_fixed_bytes();
        if result.is_none() {
            return Err(Error::Internal("decode 2nd param failed".to_string()));
        }
        let hasher = result.unwrap()[..4].into_iter().take(4).enumerate().fold(
            [0u8; 4],
            |mut acc, (idx, val)| {
                acc[idx] = *val;
                acc
            },
        );
        trace!("hasher = {:?}", hasher);
        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(Error::Internal("decode 3rd param failed".to_string()));
        }
        let nonce = U256::from_big_endian(&result.unwrap()).low_u64();
        trace!("nonce = {}", nonce);
        let result = decoded.remove(0).to_bytes();
        if result.is_none() {
            return Err(Error::Internal("decode 4th param failed".to_string()));
        }
        let proof_data = result.unwrap();
        trace!("data = {:?}", proof_data);

        let proof = TxProof::from_bytes(&proof_data);

        let relay_info = proof.extract_relay_info();
        if relay_info.is_none() {
            return Err(Error::Internal("extract relay info failed".to_string()));
        }
        let relay_info = relay_info.unwrap();
        trace!("relay_info {:?}", proof_data);

        let ret = ChainManagement::ext_chain_id(ext, &gas_left, &params.sender);
        if ret.is_none() {
            return Err(Error::Internal("get chain id failed".to_owned()));
        }
        let (gas_left, chain_id) = ret.unwrap();

        let ret = ChainManagement::ext_authorities(
            ext,
            &gas_left,
            &params.sender,
            relay_info.from_chain_id,
        );
        if ret.is_none() {
            return Err(Error::Internal("get authorities failed".to_owned()));
        }
        let (gas_left, authorities) = ret.unwrap();

        let ret = proof.extract_crosschain_data(addr, hasher, nonce, chain_id, &authorities[..]);
        if ret.is_none() {
            return Err(Error::Internal(
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

        Ok(GasLeft::NeedsReturn {
            gas_left: gas_left,
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }

    fn verify_state(&mut self, _params: ActionParams, _ext: &mut Ext) -> Result<GasLeft, Error> {
        // TODO to be continued ...
        Err(Error::OutOfGas)
    }

    fn verify_block_header(
        &mut self,
        params: ActionParams,
        ext: &mut Ext,
    ) -> Result<GasLeft, Error> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(Error::OutOfGas);
        }
        let mut gas_left = params.gas - gas_cost;

        if params.data.is_none() {
            return Err(Error::Internal("no data".to_string()));
        }

        let data = params.data.unwrap();
        trace!("data = {:?}", data);
        let tokens = vec![ethabi::ParamType::Uint(32), ethabi::ParamType::Bytes];

        let result = ethabi::decode(&tokens, &data[4..]);
        if result.is_err() {
            return Err(Error::Internal("decode failed".to_string()));
        }
        let mut decoded = result.unwrap();
        trace!("decoded = {:?}", decoded);

        let result = decoded.remove(0).to_uint();
        if result.is_none() {
            return Err(Error::Internal("decode 3rd param failed".to_string()));
        }
        let chain_id = U256::from_big_endian(&result.unwrap()).low_u32();
        trace!("chain_id = {}", chain_id);
        let result = decoded.remove(0).to_bytes();
        if result.is_none() {
            return Err(Error::Internal("decode 1th param failed".to_string()));
        }
        let block_header_curr_bytes = result.unwrap();
        trace!("data = {:?}", block_header_curr_bytes);
        let block_header_curr = Header::from_bytes(&block_header_curr_bytes);

        let block_header_prev_bytes = self.block_header.get_bytes::<Vec<u8>>(ext)?;

        let verify_result = if block_header_prev_bytes.len() == 0 {
            block_header_curr.number() == 1
        } else {
            let block_header_prev = Header::from_bytes(&block_header_prev_bytes);

            let ret = ChainManagement::ext_authorities(ext, &gas_left, &params.sender, chain_id);
            if ret.is_none() {
                return Err(Error::Internal("get authorities failed".to_owned()));
            }
            let (gas_left_new, authorities) = ret.unwrap();
            gas_left = gas_left_new;

            block_header_prev.verify_next(&block_header_curr, &authorities[..])
        };

        if verify_result {
            self.block_header.set_bytes(ext, block_header_curr_bytes)?;
        }

        let tokens = vec![ethabi::Token::Bool(verify_result)];
        let result = ethabi::encode(&tokens);
        trace!("encoded {:?}", result);

        self.output = result;

        Ok(GasLeft::NeedsReturn {
            gas_left: gas_left,
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }
}
