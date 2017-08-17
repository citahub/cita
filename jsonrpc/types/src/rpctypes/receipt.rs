// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use rpctypes::log::Log;
use types::receipt::{Receipt as EthReceipt, RichReceipt, LocalizedReceipt};
use util::{H160, H256, Bloom, U256};

/// Receipt
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    /// Transaction Hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<H256>,
    /// Transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<U256>,
    /// Block hash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<H256>,
    /// Block
    #[serde(rename = "blockNumber")]
    pub block_number: Option<U256>,
    /// Cumulative gas used
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: U256,
    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<U256>,
    /// Contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<H160>,
    /// Logs
    pub logs: Vec<Log>,
    /// State Root
    #[serde(rename = "root")]
    pub state_root: Option<H256>,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Bloom,
}

impl From<LocalizedReceipt> for Receipt {
    fn from(r: LocalizedReceipt) -> Self {
        Receipt {
            transaction_hash: Some(r.transaction_hash.into()),
            transaction_index: Some(r.transaction_index.into()),
            block_hash: Some(r.block_hash.into()),
            block_number: Some(r.block_number.into()),
            cumulative_gas_used: r.cumulative_gas_used.into(),
            gas_used: Some(r.gas_used.into()),
            contract_address: r.contract_address.map(Into::into),
            logs: r.logs.into_iter().map(Into::into).collect(),
            state_root: r.state_root.map(Into::into),
            logs_bloom: r.log_bloom.into(),
        }
    }
}

impl From<RichReceipt> for Receipt {
    fn from(r: RichReceipt) -> Self {
        Receipt {
            transaction_hash: Some(r.transaction_hash.into()),
            transaction_index: Some(r.transaction_index.into()),
            block_hash: None,
            block_number: None,
            cumulative_gas_used: r.cumulative_gas_used.into(),
            gas_used: Some(r.gas_used.into()),
            contract_address: r.contract_address.map(Into::into),
            logs: r.logs.into_iter().map(Into::into).collect(),
            state_root: r.state_root.map(Into::into),
            logs_bloom: r.log_bloom.into(),
        }
    }
}

impl From<EthReceipt> for Receipt {
    fn from(r: EthReceipt) -> Self {
        Receipt {
            transaction_hash: None,
            transaction_index: None,
            block_hash: None,
            block_number: None,
            cumulative_gas_used: r.gas_used.into(),
            gas_used: None,
            contract_address: None,
            logs: r.logs.into_iter().map(Into::into).collect(),
            state_root: r.state_root.map(Into::into),
            logs_bloom: r.log_bloom.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{serialize, deserialize, Infinite};
    use serde_json;
    use util::{H256 as Hash256, H2048 as Hash2048};

    #[test]
    fn receipt_serialization() {
        let s = r#"{"transactionHash":"0x0000000000000000000000000000000000000000000000000000000000000000","transactionIndex":"0x0","blockHash":"0xed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5","blockNumber":"0x4510c","cumulativeGasUsed":"0x20","gasUsed":"0x10","contractAddress":null,"logs":[{"address":"0x33990122638b9132ca29c723bdf037f1a891a70c","topics":["0xa6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc","0x4861736852656700000000000000000000000000000000000000000000000000"],"data":"0x","blockHash":"0xed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5","blockNumber":"0x4510c","transactionHash":"0x0000000000000000000000000000000000000000000000000000000000000000","transactionIndex":"0x0","logIndex":"0x1","transactionLogIndex":null}],"root":"0x000000000000000000000000000000000000000000000000000000000000000a","logsBloom":"0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f"}"#;

        let receipt = Receipt {
            transaction_hash: Some(Hash256::from(0).into()),
            transaction_index: Some(0.into()),
            block_hash: Some("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5".parse().unwrap()),
            block_number: Some(0x4510c.into()),
            cumulative_gas_used: 0x20.into(),
            gas_used: Some(0x10.into()),
            contract_address: None,
            logs: vec![
                Log {
                    address: "33990122638b9132ca29c723bdf037f1a891a70c".parse().unwrap(),
                    topics: vec![
                        "a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc".parse().unwrap(),
                        "4861736852656700000000000000000000000000000000000000000000000000".parse().unwrap(),
                    ],
                    data: vec![].into(),
                    block_hash: Some("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5".parse().unwrap()),
                    block_number: Some(0x4510c.into()),
                    transaction_hash: Some(Hash256::from(0).into()),
                    transaction_index: Some(0.into()),
                    transaction_log_index: None,
                    log_index: Some(1.into()),
                },
            ],
            logs_bloom: Hash2048::from(15).into(),
            state_root: Some(Hash256::from(10).into()),
        };

        let serialized = serde_json::to_string(&receipt).unwrap();
        assert_eq!(serialized, s);
    }

    #[test]
    fn test_bincode_deserialization() {
        let receipt = Receipt {
            transaction_hash: Some(Hash256::from(0).into()),
            transaction_index: Some(0.into()),
            block_hash: Some("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5".parse().unwrap()),
            block_number: Some(0x4510c.into()),
            cumulative_gas_used: 0x20.into(),
            gas_used: Some(0x10.into()),
            contract_address: None,
            logs: vec![
                Log {
                    address: "33990122638b9132ca29c723bdf037f1a891a70c".parse().unwrap(),
                    topics: vec![
                        "a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc".parse().unwrap(),
                        "4861736852656700000000000000000000000000000000000000000000000000".parse().unwrap(),
                    ],
                    data: vec![].into(),
                    block_hash: Some("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5".parse().unwrap()),
                    block_number: Some(0x4510c.into()),
                    transaction_hash: Some(Hash256::from(0).into()),
                    transaction_index: Some(0.into()),
                    transaction_log_index: None,
                    log_index: Some(1.into()),
                },
            ],
            logs_bloom: Hash2048::from(15).into(),
            state_root: Some(Hash256::from(10).into()),
        };

        println!("{:?}", receipt);

        let encoded: Vec<u8> = serialize(&receipt, Infinite).unwrap();

        println!("{:?}", encoded);

        let decoded: Receipt = deserialize(&encoded[..]).unwrap();

        println!("{:?}", decoded);

        assert_eq!(decoded, receipt);
    }
}
