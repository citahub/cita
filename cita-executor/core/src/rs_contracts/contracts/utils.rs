use byteorder::{BigEndian, ByteOrder};
use cita_types::{Address, H256, U256};
use common_types::errors::ContractError;
use common_types::reserved_addresses;
use rlp::RlpStream;
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

pub fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

// keys: ordered list
pub fn get_latest_key(target: u64, keys: Vec<&u64>) -> u64 {
    if target == 0 {
        return 0;
    }

    for i in 0..keys.len() {
        if *keys[i] >= target {
            return *keys[i - 1];
        } else if i == keys.len() - 1 {
            return *keys[i];
        }
        continue;
    }
    0
}

pub fn check_same_length(conts: &[Address], funcs: &[Vec<u8>]) -> bool {
    if conts.len() == funcs.len() && conts.len() > 0 {
        return true;
    }
    false
}

pub fn is_permssion_contract(addr: Address) -> bool {
    if addr == Address::from(reserved_addresses::PERMISSION_SEND_TX)
        || addr == Address::from(reserved_addresses::PERMISSION_CREATE_CONTRACT)
        || addr == Address::from(reserved_addresses::PERMISSION_NEW_PERMISSION)
        || addr == Address::from(reserved_addresses::PERMISSION_DELETE_PERMISSION)
        || addr == Address::from(reserved_addresses::PERMISSION_UPDATE_PERMISSION)
        || addr == Address::from(reserved_addresses::PERMISSION_SET_AUTH)
        || addr == Address::from(reserved_addresses::PERMISSION_CANCEL_AUTH)
        || addr == Address::from(reserved_addresses::PERMISSION_NEW_ROLE)
        || addr == Address::from(reserved_addresses::PERMISSION_DELETE_ROLE)
        || addr == Address::from(reserved_addresses::PERMISSION_UPDATE_ROLE)
        || addr == Address::from(reserved_addresses::PERMISSION_SET_ROLE)
        || addr == Address::from(reserved_addresses::PERMISSION_CANCEL_ROLE)
        || addr == Address::from(reserved_addresses::PERMISSION_NEW_GROUP)
        || addr == Address::from(reserved_addresses::PERMISSION_DELETE_GROUP)
        || addr == Address::from(reserved_addresses::PERMISSION_UPDATE_GROUP)
        || addr == Address::from(reserved_addresses::PERMISSION_NEW_NODE)
        || addr == Address::from(reserved_addresses::PERMISSION_DELETE_NODE)
        || addr == Address::from(reserved_addresses::PERMISSION_UPDATE_NODE)
        || addr == Address::from(reserved_addresses::PERMISSION_ACCOUNT_QUOTA)
        || addr == Address::from(reserved_addresses::PERMISSION_BLOCK_QUOTA)
        || addr == Address::from(reserved_addresses::PERMISSION_BATCH_TX)
        || addr == Address::from(reserved_addresses::PERMISSION_EMERGENCY_INTERVENTION)
        || addr == Address::from(reserved_addresses::PERMISSION_QUOTA_PRICE)
        || addr == Address::from(reserved_addresses::PERMISSION_VERSION)
    {
        return true;
    }
    false
}

pub fn h256_to_bool(a: H256) -> bool {
    if a == H256::from(1) {
        return true;
    }
    false
}
