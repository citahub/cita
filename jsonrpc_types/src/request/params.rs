// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

/// Params for different JSON-RPC methods.
use rpctypes::{
    BlockNumber, Boolean, CallRequest, Data, Data20, Data32, Filter, OneItemTupleTrick, Quantity,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaBlockNumberParams();

impl CitaBlockNumberParams {
    pub fn new() -> Self {
        CitaBlockNumberParams()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetPeerCountParams();

impl NetPeerCountParams {
    pub fn new() -> Self {
        NetPeerCountParams()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaSendRawTransactionParams(pub Data, #[serde(skip)] OneItemTupleTrick);

impl CitaSendRawTransactionParams {
    pub fn new(signed_tx: Data) -> Self {
        CitaSendRawTransactionParams(signed_tx, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaSendTransactionParams(pub Data, #[serde(skip)] OneItemTupleTrick);

impl CitaSendTransactionParams {
    pub fn new(signed_tx: Data) -> Self {
        CitaSendTransactionParams(signed_tx, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaGetBlockByHashParams(pub Data32, pub Boolean);

impl CitaGetBlockByHashParams {
    pub fn new(block_hash: Data32, with_txs: Boolean) -> Self {
        CitaGetBlockByHashParams(block_hash, with_txs)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaGetBlockByNumberParams(pub BlockNumber, pub Boolean);

impl CitaGetBlockByNumberParams {
    pub fn new(block_height: BlockNumber, with_txs: Boolean) -> Self {
        CitaGetBlockByNumberParams(block_height, with_txs)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetTransactionReceiptParams(pub Data32, #[serde(skip)] OneItemTupleTrick);

impl EthGetTransactionReceiptParams {
    pub fn new(tx_hash: Data32) -> Self {
        EthGetTransactionReceiptParams(tx_hash, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetLogsParams(pub Filter, #[serde(skip)] OneItemTupleTrick);

impl EthGetLogsParams {
    pub fn new(filter: Filter) -> Self {
        EthGetLogsParams(filter, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthCallParams(pub CallRequest, #[serde(default)] pub BlockNumber);

impl EthCallParams {
    pub fn new(call_req: CallRequest, block_height: BlockNumber) -> Self {
        EthCallParams(call_req, block_height)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaGetTransactionParams(pub Data32, #[serde(skip)] OneItemTupleTrick);

impl CitaGetTransactionParams {
    pub fn new(tx_hash: Data32) -> Self {
        CitaGetTransactionParams(tx_hash, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetTransactionCountParams(pub Data20, #[serde(default)] pub BlockNumber);

impl EthGetTransactionCountParams {
    pub fn new(address: Data20, block_height: BlockNumber) -> Self {
        EthGetTransactionCountParams(address, block_height)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetCodeParams(pub Data20, #[serde(default)] pub BlockNumber);

impl EthGetCodeParams {
    pub fn new(address: Data20, block_height: BlockNumber) -> Self {
        EthGetCodeParams(address, block_height)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetAbiParams(pub Data20, #[serde(default)] pub BlockNumber);

impl EthGetAbiParams {
    pub fn new(address: Data20, block_height: BlockNumber) -> Self {
        EthGetAbiParams(address, block_height)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetBalanceParams(pub Data20, #[serde(default)] pub BlockNumber);

impl EthGetBalanceParams {
    pub fn new(address: Data20, block_height: BlockNumber) -> Self {
        EthGetBalanceParams(address, block_height)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthNewFilterParams(pub Filter, #[serde(skip)] OneItemTupleTrick);

impl EthNewFilterParams {
    pub fn new(filter: Filter) -> Self {
        EthNewFilterParams(filter, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthNewBlockFilterParams();

impl EthNewBlockFilterParams {
    pub fn new() -> Self {
        EthNewBlockFilterParams()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthUninstallFilterParams(pub Quantity, #[serde(skip)] OneItemTupleTrick);

impl EthUninstallFilterParams {
    pub fn new(filter_id: Quantity) -> Self {
        EthUninstallFilterParams(filter_id, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetFilterChangesParams(pub Quantity, #[serde(skip)] OneItemTupleTrick);

impl EthGetFilterChangesParams {
    pub fn new(filter_id: Quantity) -> Self {
        EthGetFilterChangesParams(filter_id, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EthGetFilterLogsParams(pub Quantity, #[serde(skip)] OneItemTupleTrick);

impl EthGetFilterLogsParams {
    pub fn new(filter_id: Quantity) -> Self {
        EthGetFilterLogsParams(filter_id, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaGetTransactionProofParams(pub Data32, #[serde(skip)] OneItemTupleTrick);

impl CitaGetTransactionProofParams {
    pub fn new(tx_hash: Data32) -> Self {
        CitaGetTransactionProofParams(tx_hash, OneItemTupleTrick::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CitaGetMetaDataParams(pub BlockNumber, #[serde(skip)] OneItemTupleTrick);

impl CitaGetMetaDataParams {
    pub fn new(block_height: BlockNumber) -> Self {
        CitaGetMetaDataParams(block_height, OneItemTupleTrick::default())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CitaBlockNumberParams, CitaGetBlockByHashParams, CitaGetBlockByNumberParams,
        CitaGetMetaDataParams, CitaGetTransactionParams, CitaGetTransactionProofParams,
        CitaSendRawTransactionParams, EthCallParams, EthGetAbiParams, EthGetBalanceParams,
        EthGetCodeParams, EthGetFilterChangesParams, EthGetFilterLogsParams, EthGetLogsParams,
        EthGetTransactionCountParams, EthGetTransactionReceiptParams, EthNewBlockFilterParams,
        EthNewFilterParams, EthUninstallFilterParams, NetPeerCountParams,
    };
    use cita_types::{H160, H256, U256};
    use rpctypes::{BlockNumber, CallRequest, Filter, VariadicValue};
    use serde_json;
    use std::convert::Into;

    macro_rules! test_ser_and_de {
        ($type:tt, $json_params:tt, $value:tt) => {
            let data = $type::new$value;
            let serialized = serde_json::to_string(&data).unwrap();
            let jsonstr = json!($json_params).to_string();
            assert_eq!(serialized, jsonstr);
            let deserialized: $type = serde_json::from_str(&jsonstr).unwrap();
            assert_eq!(deserialized, data);
        };
        (value, $type:tt, $json_params:tt, $value:tt) => {
            let data = $type::new$value;
            let serialized = serde_json::to_value(&data).unwrap();
            let jsonval = json!($json_params);
            assert_eq!(serialized, jsonval);
            let jsonstr = jsonval.to_string();
            let deserialized: $type = serde_json::from_str(&jsonstr).unwrap();
            assert_eq!(deserialized, data);
        };
    }

    #[test]
    fn serialize_and_deserialize() {
        test_ser_and_de!(CitaBlockNumberParams, [], ());

        test_ser_and_de!(NetPeerCountParams, [], ());

        test_ser_and_de!(
            CitaSendRawTransactionParams,
            ["0xabcdef"],
            (vec![0xab, 0xcd, 0xef].into())
        );

        test_ser_and_de!(
            CitaGetBlockByHashParams,
            [
                "0x000000000000000000000000000000000000000000000000000000000000000a",
                true
            ],
            (H256::from(10).into(), true.into())
        );

        test_ser_and_de!(
            CitaGetBlockByNumberParams,
            ["0x10", false],
            (BlockNumber::new(16u64.into()), false.into())
        );

        test_ser_and_de!(
            EthGetTransactionReceiptParams,
            ["0x000000000000000000000000000000000000000000000000000000000000000a"],
            (H256::from(10).into())
        );

        test_ser_and_de!(
            value,
            EthGetLogsParams,
            [{
                "fromBlock": "0xb",
                "address": "0x0000000000000000000000000000000000000010",
                "topics": [
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                    [
                        "0x0000000000000000000000000000000000000000000000000000000000000002",
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    ],
                    null,
                ]
            }],
            (Filter::new(
                BlockNumber::new(11u64.into()),
                BlockNumber::latest(),
                Some(VariadicValue::single(H160::from(16).into())),
                Some(
                    vec![
                        VariadicValue::single(H256::from(1).into()),
                        VariadicValue::multiple(
                            vec![
                                H256::from(2).into(),
                                H256::from(3).into(),
                            ]),
                        VariadicValue::null(),
                    ]
                ),
            ))
        );

        test_ser_and_de!(
            EthCallParams,
            [
                {
                    "from": "0x000000000000000000000000000000000000000b",
                    "to": "0x000000000000000000000000000000000000000c",
                },
                "latest",
            ],
            (
                CallRequest::new(Some(H160::from(11).into()),
                H160::from(12).into(), None),
                BlockNumber::latest()
            ));

        test_ser_and_de!(
            CitaGetTransactionParams,
            ["0x000000000000000000000000000000000000000000000000000000000000000a"],
            (H256::from(10).into())
        );

        test_ser_and_de!(
            EthGetTransactionCountParams,
            ["0x000000000000000000000000000000000000000a", "0x10"],
            (H160::from(10).into(), BlockNumber::new(16u64.into()))
        );

        test_ser_and_de!(
            EthGetCodeParams,
            ["0x000000000000000000000000000000000000000b", "0x11"],
            (H160::from(11).into(), BlockNumber::new(17u64.into()))
        );

        test_ser_and_de!(
            EthGetAbiParams,
            ["0x000000000000000000000000000000000000000c", "0x12"],
            (H160::from(12).into(), BlockNumber::new(18u64.into()))
        );

        test_ser_and_de!(
            EthGetBalanceParams,
            ["0x000000000000000000000000000000000000000d", "0x13"],
            (H160::from(13).into(), BlockNumber::new(19u64.into()))
        );

        test_ser_and_de!(
            value,
            EthNewFilterParams,
            [{
                "fromBlock": "0xc",
                "address": "0x0000000000000000000000000000000000000010",
                "topics": [
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                    [
                        "0x0000000000000000000000000000000000000000000000000000000000000002",
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    ],
                    null,
                ]
            }],
            (Filter::new(
                BlockNumber::new(12u64.into()),
                BlockNumber::latest(),
                Some(VariadicValue::single(H160::from(16).into())),
                Some(
                    vec![
                        VariadicValue::single(H256::from(1).into()),
                        VariadicValue::multiple(
                            vec![
                                H256::from(2).into(),
                                H256::from(3).into(),
                            ]),
                        VariadicValue::null(),
                    ]
                ),
            ))
        );

        test_ser_and_de!(EthNewBlockFilterParams, [], ());

        test_ser_and_de!(EthUninstallFilterParams, ["0xa"], (U256::from(10).into()));

        test_ser_and_de!(EthGetFilterChangesParams, ["0xb"], (U256::from(11).into()));

        test_ser_and_de!(EthGetFilterLogsParams, ["0xc"], (U256::from(12).into()));

        test_ser_and_de!(
            CitaGetTransactionProofParams,
            ["0x000000000000000000000000000000000000000000000000000000000000000b"],
            (H256::from(11).into())
        );

        test_ser_and_de!(
            CitaGetMetaDataParams,
            ["earliest"],
            (BlockNumber::earliest())
        );
    }
}
