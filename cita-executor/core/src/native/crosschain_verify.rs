use super::*;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use core::libchain::chain::TxProof;
use rlp;
use util::{Address, H256, U256};

#[derive(Clone)]
pub struct CrossChainVerify {
    output: Vec<u8>,
}

impl Contract for CrossChainVerify {
    fn exec(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let signature = BigEndian::read_u32(params.clone().data.unwrap().get(0..4).unwrap());
        match signature {
            0 => self.verify(params, ext),
            _ => Err(evm::Error::OutOfGas),
        }
    }
    fn create(&self) -> Box<Contract> {
        Box::new(CrossChainVerify::default())
    }
}

impl Default for CrossChainVerify {
    fn default() -> Self {
        CrossChainVerify { output: Vec::new() }
    }
}

impl CrossChainVerify {
    fn verify(&mut self, params: ActionParams, _ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let gas_cost = U256::from(10000);
        if params.gas < gas_cost {
            return Err(evm::Error::OutOfGas);
        }

        if params.data.is_none() {
            return Err(evm::Error::Internal("no data".to_string()));
        }
        let data = params.data.unwrap();
        let data_len = data.len();
        if data_len < 4 + 32 * 4 {
            return Err(evm::Error::Internal("data too short".to_string()));
        }
        let mut index = 4;

        let mut len = 32;
        let addr_data = data.get(index..index + len);
        if addr_data.is_none() {
            return Err(evm::Error::Internal("no addr".to_string()));
        }
        let addr = Address::from(H256::from(addr_data.unwrap()));
        index = index + len;

        len = 32;
        let hasher_data = data.get(index..index + len);
        if hasher_data.is_none() {
            return Err(evm::Error::Internal("no hasher".to_string()));
        }
        // U256 to hex no leading zero
        let mut hasher = U256::from(hasher_data.unwrap()).to_hex();
        if hasher.len() > 8 {
            return Err(evm::Error::OutOfGas);
        }
        if hasher.len() < 8 {
            hasher = format!("{:08}", hasher);
        }
        index = index + len;

        len = 32;
        let nonce_data = data.get(index..index + len);
        if nonce_data.is_none() {
            return Err(evm::Error::Internal("no nonce".to_string()));
        }
        let nonce = U256::from(nonce_data.unwrap()).low_u64();
        index = index + len;

        len = 32;
        let proof_len_data = data.get(index..index + len);
        if proof_len_data.is_none() {
            return Err(evm::Error::Internal("no proof len".to_string()));
        }
        let proof_len = U256::from(proof_len_data.unwrap()).low_u64() as usize;
        index = index + len;

        if index + proof_len > data_len {
            return Err(evm::Error::Internal(
                "data shorter than proof len".to_string(),
            ));
        }

        let proof_data = data.get(index..index + proof_len);
        if proof_data.is_none() {
            return Err(evm::Error::Internal("no proof data".to_string()));
        }
        let proof_data = proof_data.unwrap();

        let proof = TxProof::from_bytes(&proof_data);

        let relay_info = proof.extract_relay_info();
        if relay_info.is_none() {
            return Err(evm::Error::Internal(
                "extract relay info failed".to_string(),
            ));
        }
        let relay_info = relay_info.unwrap();
        trace!("relay_info {:?}", proof_data);

        let ret = ChainManagement::ext_chain_id(ext, &gas_left, &params.sender);
        if ret.is_none() {
            return Err(evm::Error::Internal("get chain id failed".to_owned()));
        }
        let (gas_left, chain_id) = ret.unwrap();

        let ret = ChainManagement::ext_authorities(ext, &gas_left, &params.sender, relay_info.from_chain_id);
        if ret.is_none() {
            return Err(evm::Error::Internal("get authorities failed".to_owned()));
        }
        let (gas_left, authorities) = ret.unwrap();

        let ret = proof.extract_crosschain_data(addr, hasher, nonce, chain_id, &authorities[..]);
        if ret.is_none() {
            return Err(evm::Error::Internal(
                "extract_crosschain_data failed".to_string(),
            ));
        }
        let (sender, tx_data) = ret.unwrap();

        self.output.clear();
        for _ in 0..12 {
            self.output.push(0);
        }
        for v in sender.0.iter() {
            self.output.push(*v);
        }
        for v in tx_data.iter() {
            self.output.push(*v);
        }

        Ok(GasLeft::NeedsReturn {
            gas_left: U256::from(params.gas - gas_cost),
            data: ReturnData::new(self.output.clone(), 0, self.output.len()),
            apply_state: true,
        })
    }
}
