//use contracts::permission_management::contains_resource;
use cita_types::clean_0x;
use cita_types::traits::LowerHex;
use cita_types::{Address, H160, H256, U256};
use db::{self as db, Key, Readable, Writable};
use error::{Error, ExecutionError};
use executive::check_permission;
use grpc::Result as GrpcResult;
use grpc::Server;
use libexecutor::executor::Executor;
use libproto::citacode::{ActionParams, EnvInfo, InvokeRequest, InvokeResponse};
use libproto::citacode_grpc::{CitacodeService, CitacodeServiceClient};
use libproto::executor::{LoadRequest, LoadResponse, RegisterRequest, RegisterResponse};
use libproto::executor_grpc::{ExecutorService, ExecutorServiceServer};
use log_entry::LogEntry;
use receipt::Receipt;
use rlp::*;
use state::backend::Backend as StateBackend;
use state::State;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::str::FromStr;
use std::sync::Arc;
use types::ids::BlockId;
use types::transaction::{Action, SignedTransaction};
use util::RwLock;
use util::*;

#[derive(Clone)]
pub struct ConnectInfo {
    ip: String,
    port: u16,
    address: String,
}

impl ConnectInfo {
    pub fn new(ip: String, port: u16, addr: String) -> Self {
        ConnectInfo {
            ip: ip,
            port: port,
            address: addr,
        }
    }

    pub fn get_ip(&self) -> &str {
        self.ip.as_ref()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_addr(&self) -> &str {
        self.address.as_ref()
    }

    pub fn stream_rlp(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.ip);
        s.append(&self.port);
        s.append(&self.address);
    }

    /// Get the RLP of this header.
    pub fn rlp(&self) -> Bytes {
        let mut s = RlpStream::new();
        self.stream_rlp(&mut s);
        s.out()
    }
}

impl Encodable for ConnectInfo {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s);
    }
}

impl Decodable for ConnectInfo {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let conn_info = ConnectInfo {
            ip: r.val_at(0)?,
            port: r.val_at(1)?,
            address: r.val_at(2)?,
        };

        Ok(conn_info)
    }
}

#[derive(Clone)]
pub struct ContractState {
    pub conn_info: ConnectInfo,
    pub height: u64,
}

impl ContractState {
    // add code here
    pub fn new(ip: String, port: u16, address: String, h: u64) -> Self {
        ContractState {
            conn_info: ConnectInfo::new(ip, port, address),
            height: h,
        }
    }

    pub fn stream_rlp(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.conn_info);
        s.append(&self.height);
    }

    /// Get the RLP of this header.
    pub fn rlp(&self) -> Bytes {
        let mut s = RlpStream::new();
        self.stream_rlp(&mut s);
        s.out()
    }
}

impl Encodable for ContractState {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s);
    }
}

impl Decodable for ContractState {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let contract_state = ContractState {
            conn_info: r.val_at(0)?,
            height: r.val_at(1)?,
        };

        Ok(contract_state)
    }
}

impl Key<ContractState> for H160 {
    type Target = H160;

    fn key(&self) -> H160 {
        *self
    }
}

pub struct ServiceMap {
    disable: RwLock<HashMap<String, ContractState>>,
    enable: RwLock<HashMap<String, ContractState>>,
}

impl ServiceMap {
    pub fn new() -> Self {
        ServiceMap {
            disable: RwLock::new(HashMap::new()),
            enable: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_enable(&self, contract_address: String) {
        match self.disable.write().remove(&contract_address) {
            Some(value) => {
                self.enable.write().insert(contract_address, value);
            }
            None => {
                warn!(
                    "can't enable go contract address [{}] because it have not registed!",
                    contract_address
                );
            }
        }
    }

    pub fn set_enable_height(&self, contract_address: String, height: u64) {
        if let Some(value) = self.enable.write().get_mut(&contract_address) {
            value.height = height;
        }
    }

    pub fn insert_disable(&self, key: String, ip: String, port: u16, height: u64) {
        self.disable
            .write()
            .insert(key, ContractState::new(ip, port, "".to_string(), height));
    }

    pub fn contains_key(&self, key: String) -> bool {
        self.enable.write().contains_key(&key)
    }

    pub fn get(&self, key: String, enable: bool) -> Option<ContractState> {
        if enable {
            if let Some(value) = self.enable.write().get(&key) {
                Some(value.clone())
            } else {
                None
            }
        } else {
            if let Some(value) = self.disable.write().get(&key) {
                Some(value.clone())
            } else {
                None
            }
        }
    }
}

pub struct ExecutorServiceImpl {
    service_map: Arc<ServiceMap>,
    ext: Arc<Executor>,
}

impl ExecutorService for ExecutorServiceImpl {
    // add code here
    fn register(
        &self,
        _o: ::grpc::RequestOptions,
        req: RegisterRequest,
    ) -> ::grpc::SingleResponse<RegisterResponse> {
        let mut r = RegisterResponse::new();
        let caddr = req.get_contract_address();
        let ip = req.get_ip();
        let port = req.get_port();

        if let Ok(iport) = port.parse::<u16>() {
            self.service_map
                .insert_disable(clean_0x(caddr).to_string(), ip.to_string(), iport, 0);
            r.set_state(format!("OK {}---{}:{}", caddr, ip, port));
        } else {
            r.set_state(format!("Error Register {}---{}:{}", caddr, ip, port));
        }
        ::grpc::SingleResponse::completed(r)
    }

    fn load(
        &self,
        _o: ::grpc::RequestOptions,
        req: LoadRequest,
    ) -> ::grpc::SingleResponse<LoadResponse> {
        let mut r = LoadResponse::new();

        let caddr = req.get_contract_address();
        let req_key = req.get_key();
        let key = H256::from_slice(String::from(req_key).as_bytes());

        let address = Address::from_str(caddr.as_ref()).unwrap();
        let mut hi: u64 = 0;
        if let Some(info) = self.service_map.get(clean_0x(caddr).to_string(), true) {
            hi = info.height;
        }

        if hi == 0 {
            if let Some(value) = self.ext.db.read().read(db::COL_EXTRA, &address) {
                hi = value.height
            }
        }
        if hi == 0 {
            error!("contract address {} have not created", caddr);
            r.set_value("".to_string());
        } else {
            let out = self.get_bytes(BlockId::Number(hi), &address, key);
            let value = String::from_utf8(out).unwrap();
            trace!("load find value: {}", value);
            r.set_value(value);
        }
        ::grpc::SingleResponse::completed(r)
    }
}

impl ExecutorServiceImpl {
    pub fn new(service_map: Arc<ServiceMap>, ext: Arc<Executor>) -> Self {
        ExecutorServiceImpl {
            service_map: service_map,
            ext: ext,
        }
    }

    //  get vec
    fn get_bytes(&self, block_id: BlockId, address: &Address, key: H256) -> Vec<u8> {
        let mut out = Vec::new();
        match self.ext.state_at(block_id) {
            Some(state) => {
                if let Ok(len) = state.storage_at(&address, &key) {
                    let len = len.low_u64();
                    let hlen = len / 32;
                    let modnum = len & 0x1f;
                    let mut pos = U256::from(key);

                    for _ in 0..hlen {
                        pos = pos + U256::one();
                        if let Ok(val) = state.storage_at(&address, &H256::from(pos)) {
                            out.extend_from_slice(val.as_ref());
                        } else {
                            return Vec::new();
                        }
                    }

                    if modnum > 0 {
                        pos = pos + U256::one();
                        if let Ok(val) = state.storage_at(&address, &H256::from(pos)) {
                            out.extend_from_slice(&(val.as_ref() as &[u8])[0..modnum as usize])
                        }
                    }
                }
                out
            }
            None => {
                error!("state do not find by height");
                out
            }
        }
    }
}

pub fn vm_grpc_server(
    port: u16,
    service_map: Arc<ServiceMap>,
    ext: Arc<Executor>,
) -> Option<Server> {
    let mut server = ::grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(ExecutorServiceServer::new_service_def(
        ExecutorServiceImpl::new(service_map, ext),
    ));
    server.http.set_cpu_pool_threads(4);
    match server.build() {
        Ok(server) => Some(server),
        Err(_) => None,
    }
}

pub struct CallEvmImpl<'a, B: 'a + StateBackend> {
    state: &'a mut State<B>,
    gas_used: U256,
    check_permission: bool,
}

impl<'a, B: 'a + StateBackend> CallEvmImpl<'a, B> {
    pub fn new(state: &'a mut State<B>, check_permission: bool) -> Self {
        CallEvmImpl {
            state: state,
            gas_used: 0.into(),
            check_permission: check_permission,
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

    pub fn transact(
        &mut self,
        executor: &Executor,
        t: &SignedTransaction,
        env_info: EnvInfo,
        action_params: ActionParams,
        connect_info: ConnectInfo,
    ) -> Result<Receipt, Error> {
        let mut invoke_request = InvokeRequest::new();
        invoke_request.set_param(action_params);
        invoke_request.set_env_info(env_info.clone());

        let sender = *t.sender();
        let nonce = self.state.nonce(&sender)?;
        self.state.inc_nonce(&sender)?;

        trace!("permission should be check: {}", self.check_permission);
        if self.check_permission {
            check_permission(
                &self.state.group_accounts,
                &self.state.account_permissions,
                t,
            )?;
        }

        let base_gas_required = U256::from(100); // `CREATE` transaction cost

        if sender != Address::zero() && t.action != Action::Store && t.gas < base_gas_required {
            return Err(From::from(ExecutionError::NotEnoughBaseGas {
                required: base_gas_required,
                got: t.gas,
            }));
        }

        let ip = connect_info.get_ip();
        let port = connect_info.get_port();
        let addr = connect_info.get_addr();

        let (resp, contract_address) = {
            match t.action {
                Action::Call(addr) => {
                    let resp = self.call(ip, port, invoke_request);
                    (resp, Address::from_slice(&addr))
                }
                _ => {
                    let resp = self.create(ip, port, invoke_request);
                    // set enable
                    let contract_address = Address::from_slice(&t.data);
                    executor
                        .service_map
                        .set_enable(contract_address.lower_hex());
                    info!(
                        "enable go contract {} at {}:{}",
                        contract_address.lower_hex(),
                        ip,
                        port
                    );
                    (resp, contract_address)
                }
            }
        };

        if let Ok(resp) = resp {
            let mut batch = executor.db.read().transaction();
            let h = env_info.get_number().parse::<u64>().unwrap();
            let value = ContractState::new(ip.to_string(), port, addr.to_string(), h);
            batch.write(db::COL_EXTRA, &contract_address, &value);
            executor.db.read().write(batch).unwrap();
            executor
                .service_map
                .set_enable_height(contract_address.lower_hex(), h);

            for storage in resp.get_storages().into_iter() {
                let mut value = Vec::new();
                let key = H256::from_slice(storage.get_key());
                value.extend_from_slice(storage.get_value());

                trace!("recv resp: {:?}", storage);
                trace!("key: {:?}, value: {:?}", key, value);
                self.set_bytes(contract_address, key, &value)
            }
            // todo cumulative gas
            let gas_left = U256::from_str(resp.get_gas_left()).unwrap();
            let gas_used = t.gas - gas_left;
            let cumulative_gas_used = self.gas_used + gas_used;

            let logs = resp
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
                        topics: topics,
                        data: data.to_vec(),
                    }
                })
                .collect();

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

    pub fn set_bytes(&mut self, address: Address, key: H256, info: &Vec<u8>) {
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
