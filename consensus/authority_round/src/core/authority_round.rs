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

use super::{Engine, EngineError, Signable, unix_now, AsMillis};
use authority_manage::AuthorityManage;
use crypto::{Signature, Signer, CreateKey};
use engine_json;
use libproto::*;
use libproto::blockchain::{BlockBody, Proof, Block, SignedTransaction, RichStatus};
use parking_lot::RwLock;
use proof::AuthorityRoundProof;
use protobuf::{Message, RepeatedField};
use rustc_serialize::hex::ToHex;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::mpsc::Sender;
use std::time::Duration;
use tx_pool::Pool;
use util::Address;

use util::H256;
const INIT_HEIGHT: usize = 1;
const INIT_STEP: usize = 0;

impl Signable for BlockBody {
    fn bare_hash(&self) -> H256 {
        self.transactions_root()
    }
}

pub struct AuthorityRoundParams {
    pub duration: Duration,
    /// Valid authorities
    pub authorities: Vec<Address>,
    pub authority_n: u64,
    pub signer: Signer,
}

pub struct AuthorityRound {
    params: AuthorityRoundParams,
    position: u64,
    tx_pool: Arc<RwLock<Pool>>,
    height: AtomicUsize,
    pre_hash: RwLock<Option<H256>>,
    sealing: AtomicBool,
    step: AtomicUsize,
    ready: Mutex<Sender<usize>>,
    auth_manage: RwLock<AuthorityManage>,
}

impl AuthorityRound {
    /// Create a new instance of POA engine
    pub fn new(params: AuthorityRoundParams, ready: Sender<usize>) -> Result<Arc<Self>, EngineError> {
        let position = params.authorities.iter().position(|&a| a == params.signer.address.clone().into()).unwrap() as u64;

        let engine = Arc::new(AuthorityRound {
                                  params: params,
                                  position: position,
                                  tx_pool: Arc::new(RwLock::new(Pool::new(10000, 3000))),
                                  height: AtomicUsize::new(INIT_HEIGHT),
                                  pre_hash: RwLock::new(None),
                                  sealing: AtomicBool::new(false),
                                  step: AtomicUsize::new(INIT_STEP),
                                  ready: Mutex::new(ready),
                                  auth_manage: RwLock::new(AuthorityManage::new()),
                              });
        Ok(engine)
    }

    pub fn update_height(&self) {
        // self.height.fetch_add(1, Ordering::SeqCst);
        self.step.fetch_add(1, Ordering::SeqCst);
    }

    pub fn is_sealer(&self, nonce: u64) -> bool {
        //let authority = nonce % self.params.authority_n;
        let authority = nonce % self.auth_manage.read().authority_n as u64;
        authority == self.position
    }

    pub fn generate_proof(&self, body: &mut BlockBody, step: u64) -> Proof {
        let signature = body.sign_with_privkey(self.params.signer.keypair.privkey()).unwrap();
        let proof: Proof = AuthorityRoundProof::new(step, signature).into();
        proof
    }

    pub fn generate_block(&self) -> Option<Block> {
        let block_time = unix_now();
        let height = self.height.load(Ordering::SeqCst) as u64;
        let step = self.step.load(Ordering::SeqCst) as u64;
        if self.is_sealer(step) {
            let mut block = Block::new();
            block.mut_header().set_height(height);
            let pre_hash = *self.pre_hash.read();
            block.mut_header().set_prevhash(pre_hash.unwrap().to_vec());
            {
                let mut tx_pool = self.tx_pool.write();
                let txs: Vec<SignedTransaction> = tx_pool.package(height);
                block.mut_body().set_transactions(RepeatedField::from_slice(&txs[..]));
                let proof = self.generate_proof(block.mut_body(), step);
                block.mut_header().set_timestamp(block_time.as_millis());
                let transactions_root = block.get_body().transactions_root();
                block.mut_header().set_transactions_root(transactions_root.to_vec());
                block.mut_header().set_proof(proof);
            }
            trace!("generate_block {:?}", block.crypt_hash());
            Some(block)
        } else {
            None
        }
    }

    pub fn pub_transaction(&self, tx: &SignedTransaction, tx_pub: Sender<(String, Vec<u8>)>) {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_TX, communication::MsgType::TX, tx.write_to_bytes().unwrap());
        trace!("broadcast new tx {:?}", tx);
        tx_pub.send(("consensus.tx".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }


    //call by seal_block and update_head, broadcast block to other node and also pass to chain
    pub fn pub_block(&self, block: &Block, tx_pub: Sender<(String, Vec<u8>)>) {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_BLK, communication::MsgType::BLOCK, block.write_to_bytes().unwrap());
        trace!("publish block {:?}", block.crypt_hash());
        tx_pub.send(("consensus.blk".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }
}

impl From<engine_json::AuthorityRoundParams> for AuthorityRoundParams {
    fn from(p: engine_json::AuthorityRoundParams) -> Self {
        AuthorityRoundParams {
            duration: Duration::from_millis(p.duration.into()),
            authority_n: p.authorities.len() as u64,
            authorities: p.authorities.into_iter().map(Into::into).collect::<Vec<_>>(),
            signer: Signer::from(p.signer),
        }
    }
}

impl Engine for AuthorityRound {
    fn name(&self) -> &str {
        "AuthorityRound"
    }

    fn duration(&self) -> Duration {
        self.params.duration.clone()
    }

    fn verify_block(&self, block: &Block) -> Result<(), EngineError> {
        let block_time = block.get_header().get_timestamp();
        let proof = AuthorityRoundProof::from(block.get_header().get_proof().clone());
        let signature = Signature::from(proof.signature);
        let author = block.get_body().recover_address_with_signature(&signature).unwrap();
        //if !self.params.authorities.contains(&author) {
        if !self.auth_manage.read().authorities.contains(&author) {
            trace!("verify_block author {:?}", author.to_hex());
            return Err(EngineError::NotAuthorized(author))?;
        }
        if block_time > unix_now().as_millis() {
            trace!("verify_block time {:?}", block_time);
            return Err(EngineError::FutureBlock(block_time))?;
        }
        Ok(())
    }

    fn receive_new_status(&self, status: RichStatus) {
        self.step.store(status.height as usize, Ordering::SeqCst);
        let new_height = (status.height + 1) as usize;
        let height = self.height.load(Ordering::SeqCst);
        trace!("new_status status {:?} height {:?}", status, height);

        let authorities: Vec<Address> = status.get_nodes().into_iter().map(|node| Address::from_slice(node)).collect();
        self.auth_manage.write().receive_authorities_list(height, authorities);

        if new_height == INIT_HEIGHT {
            self.height.store(new_height, Ordering::SeqCst);
            self.sealing.store(false, Ordering::SeqCst);
            {
                let _ = self.ready.lock().unwrap().send(new_height);
            }
            trace!("init height {:?} new height {:?}", height, new_height);
        }
        if new_height >= height {
            self.height.store(new_height, Ordering::SeqCst);
            let pre_hash = H256::from_slice(&status.hash);
            {
                trace!("new_status hash is {:?}", pre_hash);
                *self.pre_hash.write() = Some(pre_hash);
            }
            self.sealing.store(false, Ordering::SeqCst);
            {
                let _ = self.ready.lock().unwrap().send(new_height);
            }
            trace!("new_status height {:?} new height {:?}", height, new_height);
        }
    }

    fn receive_new_block(&self, block: &Block, tx_pub: Sender<(String, Vec<u8>)>) {
        if self.sealing.load(Ordering::SeqCst) {
            return ();
        }
        let height = self.height.load(Ordering::SeqCst) as u64;
        let block_number = block.get_header().get_height() as u64;
        if block_number == height {
            if self.verify_block(block).is_ok() {
                self.sealing.store(true, Ordering::SeqCst);
                trace!("update_head height {:?}", height);
                self.update_height();
                self.pub_block(block, tx_pub);
                let txs = block.get_body().get_transactions();
                self.tx_pool.write().update(txs);
            }
        }
    }

    fn receive_new_transaction(&self, tx: &SignedTransaction, tx_pub: Sender<(String, Vec<u8>)>, _origin: u32, from_broadcast: bool) {
        let mut content = blockchain::TxResponse::new();
        let hash: H256 = tx.crypt_hash();
        {
            let mut tx_pool = self.tx_pool.write();
            content.set_hash(hash.to_vec());
            let success = tx_pool.enqueue(tx.clone());
            if success {
                content.set_result(String::from("4:OK").into_bytes());
                self.pub_transaction(tx, tx_pub.clone());
            } else {
                content.set_result(String::from("4:DUP").into_bytes());
            }
            if !from_broadcast {
                let msg = factory::create_msg(submodules::CONSENSUS, topics::TX_RESPONSE, communication::MsgType::TX_RESPONSE, content.write_to_bytes().unwrap());
                trace!("response new tx {:?}", tx);
                tx_pub.send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
            }
        }
    }

    // call when time to seal block
    fn new_block(&self, tx_pub: Sender<(String, Vec<u8>)>) {
        if self.sealing.load(Ordering::SeqCst) {
            return ();
        }
        if let Some(block) = self.generate_block() {
            self.sealing.store(true, Ordering::SeqCst);
            self.update_height();
            self.pub_block(&block, tx_pub);
        }
    }

    #[allow(unused_variables)]
    fn set_new_status(&self, height: usize, pre_hash: H256) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn new_messages(&self, tx_pub: Sender<(String, Vec<u8>)>) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn handle_message(&self, _message: Vec<u8>, tx_pub: Sender<(String, Vec<u8>)>) -> Result<(), EngineError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn handle_proposal(&self, _message: Vec<u8>, tx_pub: Sender<(String, Vec<u8>)>) -> Result<(), EngineError> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    extern crate cita_crypto as crypto;

    use super::super::Spec;
    use crypto::SIGNATURE_NAME;

    #[test]
    fn has_valid_metadata() {
        ::std::env::set_var("DATA_PATH", "./data");
        let config_path = if SIGNATURE_NAME == "ed25519" {
            "../res/authority_round.json".to_string()
        } else if SIGNATURE_NAME == "secp256k1" {
            "../res/authority_round_secp256k1.json".to_string()
        } else {
            "not exist".to_string()
        };
        let test_spec = ::std::env::current_dir().unwrap().join(&config_path);
        println!("{}", test_spec.display());
        let engine = Spec::new_test_round(test_spec.to_str().unwrap()).engine;
        assert!(!engine.name().is_empty());
        assert!(engine.version().major >= 1);
    }
}
