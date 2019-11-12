// use super::cons_error::ContractError;
use byteorder::{BigEndian, ByteOrder};
use common_types::errors::ContractError;

pub fn extract_to_u32(data: &[u8]) -> Result<u32, ContractError> {
    if let Some(ref bytes4) = data.get(0..4) {
        Ok(BigEndian::read_u32(bytes4))
    } else {
        Err(ContractError::Internal("out of gas".to_string()))
    }
}
