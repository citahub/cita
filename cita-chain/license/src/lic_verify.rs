// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::lic_cfg::Config as LicConfig;
use crate::lic_info::{LicenseInfo, LicenseType};
use chrono::{DateTime, Utc};
use cita_crypto::{CreateKey, KeyPair, PrivKey};
use cita_types::{clean_0x, Address, H256};
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::channel::{after, select, unbounded, Receiver, Sender};
use std::fs::File;
use std::io::prelude::Read;
use std::io::ErrorKind;
use std::process::exit;
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};

// Check license validation in each 2 hour.
pub const DAILY_PATROL_PERIOD: Duration = Duration::from_secs(2 * 3600);

// Recheck license validation after 1s when system has not ready.
pub const DELAY_CHECK: Duration = Duration::from_secs(1);

#[derive(Debug, PartialEq)]
pub enum VerifyMode {
    FreeTrial,
    LicenseFile,
}

#[derive(Debug, PartialEq)]
pub enum LicVerifyError {
    NotReady,
    Interal(String),
}

pub struct LicVerify {
    verify_mode: VerifyMode,
    lic_info: Option<LicenseInfo>,
    // Issuer in CITA system.
    issuer: Address,
    genesis_block_hash: Option<H256>,
    node_addr: Address,
    free_trial_time: u64,
    current_hight: Option<u64>,
    msg_recieve: Receiver<VerifyMessage>,
    check_lic: Receiver<Instant>,
    lic_verify_client: LicVerifyClient,
    ctx_pub: Sender<(String, Vec<u8>)>,
}

impl LicVerify {
    pub fn new(lic_cfg: LicConfig, ctx_pub: Sender<(String, Vec<u8>)>) -> Result<Self, String> {
        // Get license info from license file.
        let lic_info = match File::open("cita.lic") {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .map_err(|e| format!("Read license file error: {:?}", e))?;

                let lic_info = LicenseInfo::from(buffer)?;
                Some(lic_info)
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => None,
                _ => return Err(format!("Open license file error: {:?}", e)),
            },
        };

        // Get node address from private key file.
        let node_addr = match File::open("privkey") {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .map_err(|e| format!("Read private key file error: {:?}", e))?;
                let priv_key = PrivKey::from_str(clean_0x(buffer.as_ref()))
                    .map_err(|e| format!("Parse private key error: {:?}", e))?;
                let key_pair = KeyPair::from_privkey(priv_key)
                    .map_err(|e| format!("Create key pair from private key error: {:?}", e))?;
                key_pair.address()
            }
            Err(e) => return Err(format!("Open private key file error: {:?}", e)),
        };

        let verify_mode = if lic_info.is_some() {
            let mut finger_print_str = format!("{:?}", lic_info.as_ref().unwrap().finger_print);
            finger_print_str.truncate(9);
            info!("CITA is using a license version ({})!", finger_print_str);
            VerifyMode::LicenseFile
        } else {
            info!("CITA is using a free trial version!");
            VerifyMode::FreeTrial
        };

        let (msg_sender, msg_recieve) = unbounded();
        let client = LicVerifyClient::new(msg_sender);

        // Check license afer 1s, waiting for system ready.
        let check_lic = after(DELAY_CHECK);
        Ok(LicVerify {
            verify_mode,
            lic_info,
            issuer: Address::from_str(clean_0x(lic_cfg.issuer_address.into_owned().as_ref()))
                .map_err(|e| format!("Parse address from config error: {:?}", e))?,
            genesis_block_hash: None,
            node_addr,
            free_trial_time: lic_cfg
                .free_trial_time
                .into_owned()
                .parse::<u64>()
                .map_err(|e| format!("Parse free trial time error: {:?}", e))?,
            current_hight: None,
            msg_recieve,
            check_lic,
            lic_verify_client: client,
            ctx_pub,
        })
    }

    pub fn verify(&self) -> Result<(bool, String), LicVerifyError> {
        // If free trial mode, check current hight.
        if self.verify_mode == VerifyMode::FreeTrial {
            if let Some(current_hight) = self.current_hight {
                info!(
                    "Current hight: {}, free trial time: {}",
                    current_hight, self.free_trial_time
                );
                if current_hight > self.free_trial_time {
                    return Ok((false, "Free trial version expired!".to_owned()));
                } else {
                    return Ok((true, "Free trial license OK!".to_owned()));
                }
            } else {
                return Err(LicVerifyError::NotReady);
            }
        }

        // If license mode.
        if let Some(ref info) = self.lic_info {
            // 1. Check issuer
            if info.issuer != self.issuer {
                return Ok((false, "Incorrect Issuer!".to_owned()));
            }

            // 2. Check license according license type
            match info.lic_type {
                LicenseType::Oem => (),
                LicenseType::Chain => {
                    let lic_chain_hash = H256::from_str(clean_0x(info.lic_value.as_ref()))
                        .map_err(|e| {
                            LicVerifyError::Interal(format!(
                                "Convert genesis_block_hash from license value error: {:?}",
                                e
                            ))
                        })?;
                    if let Some(ref genesis_block_hash) = self.genesis_block_hash {
                        if lic_chain_hash.ne(genesis_block_hash) {
                            return Ok((
                                false,
                                "Genesis block hash does not match to license!".to_owned(),
                            ));
                        }
                    } else {
                        return Err(LicVerifyError::NotReady);
                    }
                }
                LicenseType::Node => {
                    let lic_address = Address::from_str(clean_0x(info.lic_value.as_ref()))
                        .map_err(|e| {
                            LicVerifyError::Interal(format!(
                                "Convert Address from license value error: {:?}",
                                e
                            ))
                        })?;
                    // Read node address from
                    if lic_address.ne(&self.node_addr) {
                        return Ok((false, "Node address does not match to license!".to_owned()));
                    }
                }
            }
        }
        // 3. Check expiration date
        if !self.check_expiration_date() {
            return Ok((false, "CITA license expired!".to_owned()));
        }
        Ok((true, "CITA license check ok!".to_owned()))
    }

    // For clippy
    #[allow(clippy::drop_copy, clippy::zero_ptr)]
    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.msg_recieve) -> msg => {
                    match msg {
                        Ok(data) => {
                            data.handle(self);
                        },
                        Err(err) => error!("Receive data error {:?}", err),
                    }
                }
                recv(self.check_lic) -> _ => {
                    match self.verify() {
                        Ok(ret) => {
                            if ret.0 {
                                info!("{}", ret.1);
                                self.check_lic = after(DAILY_PATROL_PERIOD);
                            } else {
                                match self.verify_mode {
                                    VerifyMode::FreeTrial => error!("CITA license (Free trial) Invalid: {}", ret.1),
                                    VerifyMode::LicenseFile => {
                                        let mut finger_print_str = format!("{:?}", self.lic_info.as_ref().unwrap().finger_print);
                                        finger_print_str.truncate(9);
                                        error!("CITA license ({}) Invalid: {}", finger_print_str, ret.1)
                                    }
                                }

                                // Push exit massage to other service.
                                self.ctx_pub
                                .send((
                                    routing_key!(Chain >> InvalidLicense).into(),
                                    ret.1.as_bytes().to_vec(),
                                ))
                                .unwrap();
                                exit(1);
                            }
                        },
                        Err(e) => {
                            match e {
                                LicVerifyError::NotReady => self.check_lic = after(DELAY_CHECK),
                                LicVerifyError::Interal(err_msg) => {
                                    error!("Check CITA license fail: {:?}", err_msg);
                                    // Push exit massage to other service.
                                    self.ctx_pub
                                    .send((
                                        routing_key!(Chain >> InvalidLicense).into(),
                                        err_msg.as_bytes().to_vec(),
                                    ))
                                    .unwrap();
                                    exit(1);
                                }
                            }
                        },
                    }
                }
            }
        }
    }

    fn check_expiration_date(&self) -> bool {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();

        // Check now must between (begin_time ,end_time).
        if let Some(ref info) = self.lic_info {
            // If begin_time < now < end_time
            if now.gt(&info.begin_time) && now.lt(&info.end_time) {
                return true;
            }
        }
        false
    }

    pub fn client(&self) -> LicVerifyClient {
        self.lic_verify_client.clone()
    }

    fn set_current_hight(&mut self, hight: u64) {
        self.current_hight = Some(hight);
    }

    fn set_genesis_block_hash(&mut self, hash: H256) {
        self.genesis_block_hash = Some(hash);
    }
}

#[derive(Clone)]
pub struct LicVerifyClient {
    msg_sender: Sender<VerifyMessage>,
}

impl LicVerifyClient {
    pub fn new(sender: Sender<VerifyMessage>) -> Self {
        LicVerifyClient { msg_sender: sender }
    }

    pub fn set_current_hight(&self, req: CurrentHightReq) {
        self.send_req(VerifyMessage::CurrentHight(req));
    }

    pub fn set_genesis_block_hash(&self, req: GenesisBlockHashReq) {
        self.send_req(VerifyMessage::GenesisBlockHash(req));
    }

    fn send_req(&self, req: VerifyMessage) {
        if let Err(e) = self.msg_sender.try_send(req) {
            warn!(
                "[LicVerifyClient] Send message to node manager failed : {:?}",
                e
            );
        }
    }
}

pub enum VerifyMessage {
    CurrentHight(CurrentHightReq),
    GenesisBlockHash(GenesisBlockHashReq),
}

impl VerifyMessage {
    pub fn handle(self, inst: &mut LicVerify) {
        match self {
            VerifyMessage::CurrentHight(req) => req.handle(inst),
            VerifyMessage::GenesisBlockHash(req) => req.handle(inst),
        }
    }
}

pub struct CurrentHightReq {
    hight: u64,
}

impl CurrentHightReq {
    pub fn new(hight: u64) -> Self {
        CurrentHightReq { hight }
    }

    pub fn handle(self, inst: &mut LicVerify) {
        debug!("Set current block hight to {}.", self.hight);
        inst.set_current_hight(self.hight);
    }
}

pub struct GenesisBlockHashReq {
    genesis_block_hash: H256,
}

impl GenesisBlockHashReq {
    pub fn new(block_hash: H256) -> Self {
        GenesisBlockHashReq {
            genesis_block_hash: block_hash,
        }
    }

    pub fn handle(self, inst: &mut LicVerify) {
        debug!("Set genesis block hash to {:?}.", self.genesis_block_hash);
        inst.set_genesis_block_hash(self.genesis_block_hash);
    }
}
