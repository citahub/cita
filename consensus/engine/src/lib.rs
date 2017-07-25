extern crate libproto;
extern crate util;
extern crate sha3;
extern crate protobuf;
extern crate cita_crypto as crypto;
extern crate proof;
extern crate amqp;
extern crate pubsub;
extern crate engine_json;
extern crate serde_types;

mod error;
mod instrument;

pub use error::*;
pub use instrument::*;

use libproto::blockchain::{Block, Transaction, Status};
use util::SemanticVersion;
use std::time::Duration;
use pubsub::Pub;
use serde_types::hash::{H256};

pub trait Engine: Sync + Send {
    fn name(&self) -> &str;

    fn version(&self) -> SemanticVersion {
        SemanticVersion::new(1, 2, 3)
    }

    fn duration(&self) -> Duration;

    fn verify_block(&self, block: &Block) -> Result<(), EngineError>;

    fn receive_new_transaction(&self, tx: &Transaction, _pub: &mut Pub, _origin: u32, from_broadcast: bool);

    fn receive_new_block(&self, block: &Block, _pub: &mut Pub);

    fn receive_new_status(&self, status: Status);

    fn new_block(&self, _pub: &mut Pub);

    fn set_new_status(&self, height: usize, pre_hash: H256);

    fn new_messages(&self, _pub: &mut Pub) {}

    fn handle_message(&self, _message: Vec<u8>, _pub: &mut Pub) -> Result<(), EngineError> {
        Ok(())
    }

    fn handle_proposal(&self, _message: Vec<u8>, _pub: &mut Pub) -> Result<(), EngineError> {
        Ok(())
    }
}

#[test]
fn it_works() {}
