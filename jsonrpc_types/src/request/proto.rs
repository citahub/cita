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

/// Convert JSON-RPC request to proto request.
use rustc_serialize::hex::FromHex;
use serde_json;
use std::convert::TryFrom;
use std::convert::{Into, TryInto};
use uuid::Uuid;

use cita_types::clean_0x;
use cita_types::traits::LowerHex;
use libproto::request::{Call as ProtoCall, Request as ProtoRequest};
use libproto::UnverifiedTransaction;

use super::request::{
    BlockNumberParams, CallParams, GetAbiParams, GetBalanceParams, GetBlockByHashParams,
    GetBlockByNumberParams, GetCodeParams, GetFilterChangesParams, GetFilterLogsParams,
    GetLogsParams, GetMetaDataParams, GetTransactionCountParams, GetTransactionParams,
    GetTransactionProofParams, GetTransactionReceiptParams, NewBlockFilterParams, NewFilterParams,
    PeerCountParams, SendRawTransactionParams, SendTransactionParams, UninstallFilterParams,
};
use error::Error;
use rpctypes::{BlockParamsByHash, BlockParamsByNumber, CountOrCode};

fn create_request() -> ProtoRequest {
    let request_id = Uuid::new_v4().as_bytes().to_vec();
    let mut request = ProtoRequest::new();
    request.set_request_id(request_id);
    request
}

impl TryInto<ProtoRequest> for BlockNumberParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_block_number(true);
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for PeerCountParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_peercount(true);
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for SendRawTransactionParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let data: Vec<u8> = self.0.into();
        match SendRawTransactionParams::extract_unverified_tx(&data[..]) {
            Ok(un_tx) => {
                request.set_un_tx(un_tx);
                Ok(request)
            }
            Err(err) => Err(err),
        }
    }
}

impl TryInto<ProtoRequest> for SendTransactionParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let data: Vec<u8> = self.0.into();
        match SendRawTransactionParams::extract_unverified_tx(&data[..]) {
            Ok(un_tx) => {
                request.set_un_tx(un_tx);
                Ok(request)
            }
            Err(err) => Err(err),
        }
    }
}

impl SendRawTransactionParams {
    fn extract_unverified_tx(data: &[u8]) -> Result<UnverifiedTransaction, Error> {
        let un_tx = UnverifiedTransaction::try_from(data).map_err(|_err| {
            let err_msg = format!(
                "parse protobuf UnverifiedTransaction data error : {:?}",
                _err
            );
            Error::parse_error_with_message(err_msg)
        })?;
        {
            let tx = un_tx.get_transaction();
            let to = clean_0x(tx.get_to());
            if to.len() != 40 && !to.is_empty() {
                return Err(Error::invalid_params(
                    "param 'to' length too short, or are you create contract?",
                ));
            } else {
                let _ = to.from_hex().map_err(|err| {
                    let err_msg = format!("param not hex string : {:?}", err);
                    Error::parse_error_with_message(err_msg)
                })?;
            }
            trace!(
                "SEND ProtoTransaction: nonce {:?}, block_limit {:?}, data {}, quota {:?}, to {:?}, hash {}",
                tx.get_nonce(),
                tx.get_valid_until_block(),
                tx.get_data().lower_hex(),
                tx.get_quota(),
                tx.get_to(),
                un_tx.crypt_hash().lower_hex()
            );
        }
        Ok(un_tx)
    }
}

impl TryInto<ProtoRequest> for GetBlockByHashParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&BlockParamsByHash::new(self.0.into(), self.1.into()))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_hash| {
                request.set_block_by_hash(block_hash);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetBlockByNumberParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&BlockParamsByNumber::new(self.0, self.1.into()))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_height| {
                request.set_block_by_height(block_height);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetTransactionReceiptParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_transaction_receipt(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetLogsParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_filter(serde_json::to_string(&self.0).unwrap());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for CallParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let mut call = ProtoCall::new();
        call.set_from(self.0.from.unwrap_or_default().into());
        call.set_to(self.0.to.into());
        call.set_data(self.0.data.unwrap_or_default().into());
        serde_json::to_string(&self.1)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                call.set_height(height);
                request.set_call(call);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetTransactionParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_transaction(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetTransactionCountParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_transaction_count(jsonstr);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetCodeParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_code(jsonstr);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetAbiParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_abi(jsonstr);
                request
            })
    }
}

impl TryInto<ProtoRequest> for GetBalanceParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&CountOrCode::new(self.0.into(), self.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|jsonstr| {
                request.set_balance(jsonstr);
                request
            })
    }
}

impl TryInto<ProtoRequest> for NewFilterParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        let filter = serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(format!("{:?}", err)))?;
        request.set_new_filter(filter);
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for NewBlockFilterParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_new_block_filter(true);
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for UninstallFilterParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_uninstall_filter(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetFilterChangesParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_filter_changes(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetFilterLogsParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_filter_logs(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetTransactionProofParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        request.set_transaction_proof(self.0.into());
        Ok(request)
    }
}

impl TryInto<ProtoRequest> for GetMetaDataParams {
    type Error = Error;
    fn try_into(self) -> Result<ProtoRequest, Self::Error> {
        let mut request = create_request();
        serde_json::to_string(&self.0)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|data| {
                request.set_meta_data(data);
                request
            })
    }
}
