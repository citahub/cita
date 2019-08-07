use crate::block_number::BlockNumber;
use bloomchain::group::GroupPosition;
use cita_types::{H256, H264};

pub trait DBIndex {
    fn get_index(&self) -> Vec<u8>;
}

pub struct CurrentHash;

impl DBIndex for CurrentHash {
    fn get_index(&self) -> Vec<u8> {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66").to_vec()
    }
}

pub struct CurrentProof;

impl DBIndex for CurrentProof {
    fn get_index(&self) -> Vec<u8> {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f67").to_vec()
    }
}

pub struct CurrentHeight;

impl DBIndex for CurrentHeight {
    fn get_index(&self) -> Vec<u8> {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f68").to_vec()
    }
}

// pub struct HashToHeader;
pub struct Hash2Header(H256);

impl DBIndex for Hash2Header {
    fn get_index(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

pub struct Hash2BlockBody(H256);

impl DBIndex for Hash2BlockBody {
    fn get_index(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

pub struct Hash2BlockNumber(H256);

impl DBIndex for Hash2BlockNumber {
    fn get_index(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

pub struct BlockNumber2Header(BlockNumber);

impl DBIndex for BlockNumber2Header {
    fn get_index(&self) -> Vec<u8> {
        let mut result = [0u8; 9];
        // result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[0] = 4 as u8;
        result[1] = (self.0 >> 56) as u8;
        result[2] = (self.0 >> 48) as u8;
        result[3] = (self.0 >> 40) as u8;
        result[4] = (self.0 >> 32) as u8;
        result[5] = (self.0 >> 24) as u8;
        result[6] = (self.0 >> 16) as u8;
        result[7] = (self.0 >> 8) as u8;
        result[8] = self.0 as u8;
        result.to_vec()
    }
}

pub struct BlockNumber2Body(BlockNumber);

impl DBIndex for BlockNumber2Body {
    fn get_index(&self) -> Vec<u8> {
        let mut result = [0u8; 9];
        // result[0] = ExtrasIndex::BlockBodyHash as u8;
        result[0] = 5 as u8;
        result[1] = (self.0 >> 56) as u8;
        result[2] = (self.0 >> 48) as u8;
        result[3] = (self.0 >> 40) as u8;
        result[4] = (self.0 >> 32) as u8;
        result[5] = (self.0 >> 24) as u8;
        result[6] = (self.0 >> 16) as u8;
        result[7] = (self.0 >> 8) as u8;
        result[8] = self.0 as u8;
        result.to_vec()
    }
}

pub struct BlockNumber2Hash(BlockNumber);

impl DBIndex for BlockNumber2Hash {
    fn get_index(&self) -> Vec<u8> {
        let mut result = [0u8; 5];
        // result[0] = ExtrasIndex::BlockHash as u8;
        result[0] = 3 as u8;
        result[1] = (self.0 >> 24) as u8;
        result[2] = (self.0 >> 16) as u8;
        result[3] = (self.0 >> 8) as u8;
        result[4] = self.0 as u8;
        result.to_vec()
    }
}

pub struct Hash2TransactionIndex(H256);

impl DBIndex for Hash2TransactionIndex {
    fn get_index(&self) -> Vec<u8> {
        let mut result = H264::default();
        result[0] = 0 as u8;
        (*result)[1..].clone_from_slice(&self.0);
        result.to_vec()
    }
}

pub struct Hash2BlockReceipts(H256);

impl DBIndex for Hash2BlockReceipts {
    fn get_index(&self) -> Vec<u8> {
        let mut result = H264::default();
        result[0] = 1 as u8;
        (*result)[1..].clone_from_slice(&self.0);
        result.to_vec()
    }
}

pub struct LogGroupPosition(GroupPosition);

impl DBIndex for LogGroupPosition {
    fn get_index(&self) -> Vec<u8> {
        let mut result = [0u8; 6];
        result[0] = 2 as u8;
        result[1] = self.0.level as u8;
        result[2] = (self.0.index >> 24) as u8;
        result[3] = (self.0.index >> 16) as u8;
        result[4] = (self.0.index >> 8) as u8;
        result[5] = self.0.index as u8;
        result.to_vec()
    }
}
