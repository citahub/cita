// use super::cons_error::ContractError;
use byteorder::{BigEndian, ByteOrder};
use common_types::errors::ContractError;
use tiny_keccak::keccak256;

pub fn extract_to_u32(data: &[u8]) -> Result<u32, ContractError> {
    if let Some(ref bytes4) = data.get(0..4) {
        // trace!("")
        Ok(BigEndian::read_u32(bytes4))
    // let encode = hex::encode(bytes4.to_vec());
    // Ok(encode)
    } else {
        Err(ContractError::Internal("out of gas".to_string()))
    }
}

pub fn encode_to_u32(name: &[u8]) -> u32 {
    BigEndian::read_u32(&keccak256(name)[..])
}

pub fn encode_to_vec(name: &[u8]) -> Vec<u8> {
    keccak256(name)[0..4].to_vec()
}

// keys: ordered list
pub fn get_latest_key(target: u64, keys: Vec<&u64>) -> u64 {
    if target == 0 {
        return 0;
    }

    let keys = [0, 4];
    for i in 0..keys.len() {
        if keys[i] >= target {
            return keys[i - 1];
        } else if i == keys.len() - 1 {
            return keys[i];
        }
        continue;
    }
    0
}
