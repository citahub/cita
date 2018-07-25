use cita_types::traits::LowerHex;
use cita_types::{Address, H160, H256, U256};
use error::{Error, ExecutionError};
use evm;
use evm::action_params::{ActionParams, ActionValue};
use evm::env_info::{EnvInfo, LastHashes};
use evm::{FinalizationResult, Finalize};
use grpc::Result as GrpcResult;
use grpc_contracts::contract_state::ConnectInfo;
use libexecutor::CallEvmImpl;
use libproto::citacode::{
    ActionParams as ProtoActionParams, EnvInfo as ProtoEnvInfo, InvokeRequest, InvokeResponse,
};
use receipt::{Receipt, ReceiptError};
use state::backend::Backend as StateBackend;
use state::State;
use state_db::StateDB;
use std::str::FromStr;
use types::reserved_addresses;
use util::Bytes;

lazy_static! {
    static ref LOW_CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::GO_CONTRACT_MIN).unwrap();
    static ref HIGH_CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::GO_CONTRACT_MAX).unwrap();
}

pub fn is_grpc_contract(caddr: Address) -> bool {
    caddr > *LOW_CONTRACT_ADDRESS && caddr < *HIGH_CONTRACT_ADDRESS
}

pub fn invoke_grpc_contract<B>(
    env_info: &EnvInfo,
    params: ActionParams,
    state: &mut State<B>,
    //    executor: &Executor,
    check_permission: bool,
    check_quota: bool,
    connect_info: ConnectInfo,
) -> GrpcResult<InvokeResponse>
where
    B: StateBackend,
{
    let mut proto_env_info = ProtoEnvInfo::new();
    proto_env_info.set_number(format!("{}", env_info.number));
    proto_env_info.set_author(Address::default().lower_hex());
    let timestamp = env_info.timestamp;
    proto_env_info.set_timestamp(format!("{}", timestamp));

    let mut proto_params = ProtoActionParams::new();
    proto_params.set_code_address(connect_info.get_addr().to_string());
    proto_params.set_data(params.data.unwrap());
    proto_params.set_sender(params.sender.lower_hex());
    //to be discussed
    //action_params.set_gas("1000".to_string());
    let mut invoke_request = InvokeRequest::new();
    invoke_request.set_param(proto_params);
    invoke_request.set_env_info(proto_env_info.clone());
    let mut evm_impl = CallEvmImpl::new(state, check_permission);
    evm_impl.call(
        connect_info.get_ip(),
        connect_info.get_port(),
        invoke_request,
    )
}
