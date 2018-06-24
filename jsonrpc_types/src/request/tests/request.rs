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

use cita_types::H256;
use error::Error;
use request::{BlockNumberParams, GetTransactionReceiptParams, PartialRequest, Request};
use serde_json;
use std::convert::Into;

macro_rules! test_ser_and_de {
    ($type:ty, $data:ident, $json_params:tt) => {
        let serialized = serde_json::to_value(&$data).unwrap();
        let jsonval = json!($json_params);
        assert_eq!(serialized, jsonval);
        let jsonstr = jsonval.to_string();
        let deserialized = serde_json::from_str::<$type>(&jsonstr);
        if let Ok(deserialized) = deserialized {
            assert_eq!(deserialized, $data);
        } else {
            assert_eq!(&jsonstr, "");
        }
    };
}

#[test]
fn serialize_and_deserialize() {
    let params = GetTransactionReceiptParams::new(H256::from(10).into());
    test_ser_and_de!(
        GetTransactionReceiptParams,
        params,
        ["0x000000000000000000000000000000000000000000000000000000000000000a"]
    );

    let full_req = params.into_request(1);
    test_ser_and_de!(Request, full_req,  {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });

    let req_str: String = full_req.clone().into();
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });
    assert_eq!(part_req.complete().unwrap(), full_req);

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt"
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": null,
        });
    assert_eq!(
        part_req.complete().err().unwrap(),
        Error::invalid_params("params is requeired")
    );

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2],
        });
    assert_eq!(
        part_req.complete().err().unwrap(),
        Error::invalid_params_len()
    );

    let params = BlockNumberParams::new();
    test_ser_and_de!(BlockNumberParams, params, []);

    let full_req = params.into_request(2);
    test_ser_and_de!(Request, full_req,  {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber",
            "params": [],
        });

    let req_str: String = full_req.clone().into();
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber",
            "params": [],
        });

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber"
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber",
            "params": null,
        });
    assert_eq!(part_req.complete().unwrap(), full_req);

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
        });
    assert_eq!(
        part_req.complete().err().unwrap(),
        Error::method_not_found()
    );

    let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "notAMethod",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
    let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
    test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
        });
    assert_eq!(
        part_req.complete().err().unwrap(),
        Error::method_not_found()
    );
}
