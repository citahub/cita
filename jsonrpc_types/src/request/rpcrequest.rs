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

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;

use super::request::PartialRequest;

/// Represents jsonrpc request.
#[derive(Debug, Clone, PartialEq)]
pub enum RpcRequest {
    /// Single request
    Single(PartialRequest),
    /// Batch of requests
    Batch(Vec<PartialRequest>),
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RpcRequest::Single(ref req) => req.serialize(serializer),
            RpcRequest::Batch(ref reqs) => reqs.serialize(serializer),
        }
    }
}

impl<'a> Deserialize<'a> for RpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<RpcRequest, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: serde_json::Value = Deserialize::deserialize(deserializer)?;
        serde_json::from_value(v.clone())
            .map(RpcRequest::Batch)
            .or_else(|_| serde_json::from_value(v).map(RpcRequest::Single))
            .map_err(|_| D::Error::custom("parse rpcrequest failed")) // unreachable, but types must match
    }
}
