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

use cita_types::{Bloom, H160, H256, U256};
use rpctypes::log::Log;

/// Receipt
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    /// Receipt error message
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{deserialize, serialize, Infinite};
    use cita_types::{Bloom, H256};
    use serde_json;

    #[test]
    fn receipt_serialization() {
        let receipt = Receipt {
            transaction_hash: Some(H256::from(0).into()),
            transaction_index: Some(0.into()),
            block_hash: Some(
                "ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5"
                    .parse()
                    .unwrap(),
            ),
            block_number: Some(0x4510c.into()),
            cumulative_gas_used: 0x20.into(),
            gas_used: Some(0x10.into()),
            contract_address: None,
            logs: vec![Log {
                address: "33990122638b9132ca29c723bdf037f1a891a70c".parse().unwrap(),
                topics: vec![
                    "a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc"
                        .parse()
                        .unwrap(),
                    "4861736852656700000000000000000000000000000000000000000000000000"
                        .parse()
                        .unwrap(),
                ],
                data: vec![].into(),
                block_hash: Some(
                    "ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5"
                        .parse()
                        .unwrap(),
                ),
                block_number: Some(0x4510c.into()),
                transaction_hash: Some(H256::from(0).into()),
                transaction_index: Some(0.into()),
                transaction_log_index: None,
                log_index: Some(1.into()),
            }],
            logs_bloom: Bloom::from(15).into(),
            state_root: Some(H256::from(10).into()),
            error_message: None,
        };

        let serialized = serde_json::to_string(&receipt).unwrap();
        let deserialized: Receipt = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, receipt);
    }

    #[test]
    fn test_bincode_deserialization() {
        let receipt = Receipt {
            transaction_hash: Some(H256::from(0).into()),
            transaction_index: Some(0.into()),
            block_hash: Some(
                "ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5"
                    .parse()
                    .unwrap(),
            ),
            block_number: Some(0x4510c.into()),
            cumulative_gas_used: 0x20.into(),
            gas_used: Some(0x10.into()),
            contract_address: None,
            logs: vec![Log {
                address: "33990122638b9132ca29c723bdf037f1a891a70c".parse().unwrap(),
                topics: vec![
                    "a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc"
                        .parse()
                        .unwrap(),
                    "4861736852656700000000000000000000000000000000000000000000000000"
                        .parse()
                        .unwrap(),
                ],
                data: vec![].into(),
                block_hash: Some(
                    "ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5"
                        .parse()
                        .unwrap(),
                ),
                block_number: Some(0x4510c.into()),
                transaction_hash: Some(H256::from(0).into()),
                transaction_index: Some(0.into()),
                transaction_log_index: None,
                log_index: Some(1.into()),
            }],
            logs_bloom: Bloom::from(15).into(),
            state_root: Some(H256::from(10).into()),
            error_message: None,
        };

        println!("{:?}", receipt);

        let encoded: Vec<u8> = serialize(&receipt, Infinite).unwrap();

        println!("{:?}", encoded);

        let decoded: Receipt = deserialize(&encoded[..]).unwrap();

        println!("{:?}", decoded);

        assert_eq!(decoded, receipt);
    }
}
