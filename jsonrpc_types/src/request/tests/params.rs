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

use cita_types::{H160, H256, U256};
use request::{
    BlockNumberParams, CallParams, GetAbiParams, GetBalanceParams, GetBlockByHashParams,
    GetBlockByNumberParams, GetCodeParams, GetFilterChangesParams, GetFilterLogsParams,
    GetLogsParams, GetMetaDataParams, GetTransactionCountParams, GetTransactionParams,
    GetTransactionProofParams, GetTransactionReceiptParams, NewBlockFilterParams, NewFilterParams,
    PeerCountParams, SendRawTransactionParams, UninstallFilterParams,
};
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
    test_ser_and_de!(BlockNumberParams, [], ());

    test_ser_and_de!(PeerCountParams, [], ());

    test_ser_and_de!(
        SendRawTransactionParams,
        ["0xabcdef"],
        (vec![0xab, 0xcd, 0xef].into())
    );

    test_ser_and_de!(
        GetBlockByHashParams,
        [
            "0x000000000000000000000000000000000000000000000000000000000000000a",
            true
        ],
        (H256::from(10).into(), true.into())
    );

    test_ser_and_de!(
        GetBlockByNumberParams,
        ["0x10", false],
        (BlockNumber::new(16u64.into()), false.into())
    );

    test_ser_and_de!(
        GetTransactionReceiptParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        (H256::from(10).into())
    );

    test_ser_and_de!(
            value,
            GetLogsParams,
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
            CallParams,
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
        GetTransactionParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        (H256::from(10).into())
    );

    test_ser_and_de!(
        GetTransactionCountParams,
        ["0x000000000000000000000000000000000000000a", "0x10"],
        (H160::from(10).into(), BlockNumber::new(16u64.into()))
    );

    test_ser_and_de!(
        GetCodeParams,
        ["0x000000000000000000000000000000000000000b", "0x11"],
        (H160::from(11).into(), BlockNumber::new(17u64.into()))
    );

    test_ser_and_de!(
        GetAbiParams,
        ["0x000000000000000000000000000000000000000c", "0x12"],
        (H160::from(12).into(), BlockNumber::new(18u64.into()))
    );

    test_ser_and_de!(
        GetBalanceParams,
        ["0x000000000000000000000000000000000000000d", "0x13"],
        (H160::from(13).into(), BlockNumber::new(19u64.into()))
    );

    test_ser_and_de!(
            value,
            NewFilterParams,
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

    test_ser_and_de!(NewBlockFilterParams, [], ());

    test_ser_and_de!(UninstallFilterParams, ["0xa"], (U256::from(10).into()));

    test_ser_and_de!(GetFilterChangesParams, ["0xb"], (U256::from(11).into()));

    test_ser_and_de!(GetFilterLogsParams, ["0xc"], (U256::from(12).into()));

    test_ser_and_de!(
        GetTransactionProofParams,
        ["0x000000000000000000000000000000000000000000000000000000000000000b"],
        (H256::from(11).into())
    );

    test_ser_and_de!(GetMetaDataParams, ["earliest"], (BlockNumber::earliest()));
}
