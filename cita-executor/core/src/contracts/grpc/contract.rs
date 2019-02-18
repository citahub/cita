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

use cita_types::traits::LowerHex;
use cita_types::Address;
use contracts::grpc::{contract_state::ConnectInfo, grpc_vm::CallEvmImpl};
use evm::action_params::ActionParams;
use evm::env_info::EnvInfo;
use grpc::Result as GrpcResult;
use libproto::citacode::{
    ActionParams as ProtoActionParams, EnvInfo as ProtoEnvInfo, InvokeRequest, InvokeResponse,
};
use state::backend::Backend as StateBackend;
use state::State;
use std::str::FromStr;
use types::reserved_addresses;

lazy_static! {
    static ref CONTRACT_CREATION_ADDRESS: Address =
        Address::from_str(reserved_addresses::GO_CONTRACT).unwrap();
    static ref LOW_CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::GO_CONTRACT_MIN).unwrap();
    static ref HIGH_CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::GO_CONTRACT_MAX).unwrap();
}

pub fn low_contract_address() -> Address {
    *LOW_CONTRACT_ADDRESS
}

pub fn high_contract_address() -> Address {
    *HIGH_CONTRACT_ADDRESS
}

pub fn contract_creation_address() -> Address {
    *CONTRACT_CREATION_ADDRESS
}

pub fn is_create_grpc_address(addr: Address) -> bool {
    addr == *CONTRACT_CREATION_ADDRESS
}

pub fn is_grpc_contract(caddr: Address) -> bool {
    caddr >= *LOW_CONTRACT_ADDRESS && caddr <= *HIGH_CONTRACT_ADDRESS
}

pub fn invoke_grpc_contract<B>(
    env_info: &EnvInfo,
    params: &ActionParams,
    state: &mut State<B>,
    _check_quota: bool,
    connect_info: &ConnectInfo,
) -> GrpcResult<InvokeResponse>
where
    B: StateBackend,
{
    let invoke_request = new_invoke_request(params, connect_info, env_info);
    let mut evm_impl = CallEvmImpl::new(state);
    evm_impl.call(
        connect_info.get_ip(),
        connect_info.get_port(),
        invoke_request,
    )
}

pub fn create_grpc_contract<B>(
    env_info: &EnvInfo,
    params: &ActionParams,
    state: &mut State<B>,
    _check_quota: bool,
    connect_info: &ConnectInfo,
) -> GrpcResult<InvokeResponse>
where
    B: StateBackend,
{
    let invoke_request = new_invoke_request(params, connect_info, env_info);
    let mut evm_impl = CallEvmImpl::new(state);
    evm_impl.create(
        connect_info.get_ip(),
        connect_info.get_port(),
        invoke_request,
    )
}

fn new_invoke_request(
    params: &ActionParams,
    connect_info: &ConnectInfo,
    env_info: &EnvInfo,
) -> InvokeRequest {
    let mut proto_env_info = ProtoEnvInfo::new();
    proto_env_info.set_number(format!("{}", env_info.number));
    proto_env_info.set_author(Address::default().lower_hex());
    let timestamp = env_info.timestamp;
    proto_env_info.set_timestamp(format!("{}", timestamp));
    let mut proto_params = ProtoActionParams::new();
    proto_params.set_code_address(connect_info.get_addr().to_string());
    proto_params.set_data(params.data.to_owned().unwrap());
    proto_params.set_sender(params.sender.lower_hex());
    let mut invoke_request = InvokeRequest::new();
    invoke_request.set_param(proto_params);
    invoke_request.set_env_info(proto_env_info.clone());
    invoke_request
}
