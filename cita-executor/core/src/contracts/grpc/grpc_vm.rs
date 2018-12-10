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

use authentication::check_permission;
use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use db::{self as db, Writable};
use error::{Error, ExecutionError};
use grpc::Result as GrpcResult;
use libexecutor::sys_config::BlockSysConfig;

use contracts::grpc::{
    contract_state::{ConnectInfo, ContractState},
    service_registry,
};
use libexecutor::executor::Executor;
use libproto::citacode::{ActionParams, EnvInfo, InvokeRequest, InvokeResponse};
use libproto::citacode_grpc::{CitacodeService, CitacodeServiceClient};
use log_entry::LogEntry;
use receipt::Receipt;
use state::backend::Backend as StateBackend;
use state::State;
use std::error::Error as StdError;
use std::str::FromStr;
use types::transaction::{Action, SignedTransaction};
use util::Bytes;

pub fn extract_logs_from_response(sender: Address, response: &InvokeResponse) -> Vec<LogEntry> {
    response
        .get_logs()
        .into_iter()
        .map(|log| {
            let mut topics = Vec::new();
            let tdata = log.get_topic();

            for chunk in tdata.chunks(32) {
                let value = H256::from(chunk);
                topics.push(value);
            }

            let data = Bytes::from(log.get_data());
            LogEntry {
                address: sender,
                topics,
                data: data.to_vec(),
            }
        })
        .collect()
}

pub struct CallEvmImpl<'a, B: 'a + StateBackend> {
    state: &'a mut State<B>,
    gas_used: U256,
}

impl<'a, B: 'a + StateBackend> CallEvmImpl<'a, B> {
    pub fn new(state: &'a mut State<B>) -> Self {
        CallEvmImpl {
            state,
            gas_used: 0.into(),
        }
    }

    pub fn call(
        &mut self,
        host: &str,
        port: u16,
        invoke_request: InvokeRequest,
    ) -> GrpcResult<InvokeResponse> {
        let client = CitacodeServiceClient::new_plain(host, port, Default::default()).unwrap();
        let resp = client.invoke(::grpc::RequestOptions::new(), invoke_request);
        resp.wait_drop_metadata()
    }

    pub fn create(
        &mut self,
        host: &str,
        port: u16,
        invoke_request: InvokeRequest,
    ) -> GrpcResult<InvokeResponse> {
        let client = CitacodeServiceClient::new_plain(host, port, Default::default()).unwrap();
        let resp = client.init(::grpc::RequestOptions::new(), invoke_request);
        resp.wait_drop_metadata()
    }

    pub fn save_contract_state(&mut self, executor: &Executor, contract_state: &ContractState) {
        let addr = contract_state.get_address();
        let mut batch = executor.db.read().transaction();
        batch.write(db::COL_EXTRA, &addr, contract_state);
        executor.db.read().write(batch).unwrap();
        service_registry::set_enable_contract_height(addr, contract_state.height);
    }

    pub fn save_response_state(&mut self, contract_address: Address, response: &InvokeResponse) {
        for storage in &response.get_storages()[..] {
            let mut value = Vec::new();
            let key = H256::from_slice(storage.get_key());
            value.extend_from_slice(storage.get_value());

            trace!("recv resp: {:?}", storage);
            trace!("key: {:?}, value: {:?}", key, value);
            self.set_bytes(contract_address, key, &value[..])
        }
    }

    pub fn transact(
        &mut self,
        executor: &Executor,
        t: &SignedTransaction,
        env_info: &EnvInfo,
        action_params: &ActionParams,
        connect_info: &ConnectInfo,
        conf: &BlockSysConfig,
    ) -> Result<Receipt, Error> {
        let mut invoke_request = InvokeRequest::new();
        invoke_request.set_param(action_params.to_owned());
        invoke_request.set_env_info(env_info.to_owned());

        let sender = *t.sender();
        let nonce = self.state.nonce(&sender).map_err(|err| *err)?;
        self.state.inc_nonce(&sender).map_err(|err| *err)?;
        // TODO There are three option about permission
        if (*conf).check_options.call_permission {
            check_permission(
                &conf.group_accounts,
                &conf.account_permissions,
                t,
                (*conf).check_options,
            )?;
        }
        // FIXME: Need to check the gas required for go vm.
        let base_gas_required = U256::from(21_000);

        if sender != Address::zero() && t.action != Action::Store && t.gas < base_gas_required {
            return Err(From::from(ExecutionError::NotEnoughBaseGas {
                required: base_gas_required,
                got: t.gas,
            }));
        }

        let ip = connect_info.get_ip();
        let port = connect_info.get_port();
        let _addr = connect_info.get_addr();
        let _height: u64 = env_info.number.parse().unwrap();

        let (resp, contract_address) = {
            match t.action {
                Action::Call(addr) => {
                    let resp = self.call(ip, port, invoke_request);
                    (resp, Address::from_slice(&addr))
                }
                Action::GoCreate => {
                    let resp = self.create(ip, port, invoke_request);
                    // set enable
                    let contract_address = Address::from_slice(&t.data);
                    service_registry::enable_contract(contract_address);
                    info!(
                        "enable go contract {} at {}:{}",
                        contract_address.lower_hex(),
                        ip,
                        port
                    );
                    (resp, contract_address)
                }
                _ => panic!("unknown action {:?}", t.action),
            }
        };

        if let Ok(resp) = resp {
            let height = env_info.get_number().parse::<u64>().unwrap();
            let contract_state =
                ContractState::new(ip.to_string(), port, contract_address.to_string(), height);
            self.save_contract_state(executor, &contract_state);
            self.save_response_state(contract_address, &resp);
            // todo cumulative gas
            let gas_left = U256::from_str(resp.get_gas_left()).unwrap();
            let gas_used = t.gas - gas_left;
            let cumulative_gas_used = self.gas_used + gas_used;

            let logs = extract_logs_from_response(sender, &resp);
            let receipt = Receipt::new(
                None,
                cumulative_gas_used,
                logs,
                None,
                nonce,
                t.get_transaction_hash(),
            );
            Ok(receipt)
        } else {
            error!("go contract execute failed {:?}", resp);
            Err(Error::StdIo(::std::io::Error::new(
                ::std::io::ErrorKind::Other,
                resp.err().unwrap().description(),
            )))
        }
    }

    pub fn set_bytes(&mut self, address: Address, key: H256, info: &[u8]) {
        let len = info.len();
        if len == 0 {
            return;
        }
        self.state
            .set_storage(&address, key, H256::from(len as u64))
            .expect("grpc set_storage fail");
        let mut pos = U256::from(key) + U256::one();
        for chunk in info.chunks(32) {
            let value = H256::from(chunk);
            self.state
                .set_storage(&address, H256::from(pos), value)
                .expect("grpc set_storage fail");
            pos = pos + U256::one();
        }
    }
}
