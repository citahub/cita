use cita_types::{Address, H160};
use db::Key;
use rlp::*;
use std::str::FromStr;
use util::*;

#[derive(Clone, Debug)]
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

    pub fn get_address(&self) -> Address {
        Address::from_str(&self.conn_info.address).unwrap()
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
