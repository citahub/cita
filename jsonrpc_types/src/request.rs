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

use super::{Id, Params};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use serde_json::{from_value, Value};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Version {
    #[serde(rename = "1.0")] V1,
    #[serde(rename = "2.0")] V2,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Call {
    pub jsonrpc: Option<Version>,
    pub method: String,
    pub id: Id,
    pub params: Option<Params>,
}

/// Represents jsonrpc request.
#[derive(Debug, Clone, PartialEq)]
pub enum RpcRequest {
    /// Single request (call)
    Single(Call),
    /// Batch of requests (calls)
    Batch(Vec<Call>),
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RpcRequest::Single(ref call) => call.serialize(serializer),
            RpcRequest::Batch(ref calls) => calls.serialize(serializer),
        }
    }
}

impl<'a> Deserialize<'a> for RpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<RpcRequest, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(RpcRequest::Batch)
            .or_else(|_| from_value(v).map(RpcRequest::Single))
            .map_err(|_| D::Error::custom("")) // unreachable, but types must match
    }
}
