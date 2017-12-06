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

extern crate cita_crypto as crypto;
extern crate libproto;
extern crate util;

mod error;
mod instrument;

pub use error::*;
pub use instrument::*;
use libproto::Request;

use libproto::blockchain::{Block, RichStatus};
use std::sync::mpsc::Sender;
use std::time::Duration;
use util::H256;
use util::SemanticVersion;

pub trait Engine: Sync + Send {
    fn name(&self) -> &str;

    fn version(&self) -> SemanticVersion {
        SemanticVersion::new(1, 2, 3)
    }

    fn duration(&self) -> Duration;

    fn verify_block(&self, block: &Block) -> Result<(), EngineError>;

    fn receive_new_transaction(
        &self,
        tx_req: &Request,
        tx_pub: Sender<(String, Vec<u8>)>,
        _origin: u32,
        from_broadcast: bool,
    );

    fn receive_new_block(&self, block: &Block, tx_pub: Sender<(String, Vec<u8>)>);

    fn receive_new_status(&self, status: &RichStatus);

    fn new_block(&self, tx_pub: Sender<(String, Vec<u8>)>);

    fn set_new_status(&self, height: usize, pre_hash: H256);

    fn new_messages(&self, tx_pub: Sender<(String, Vec<u8>)>);

    fn handle_message(&self, _message: Vec<u8>, tx_pub: Sender<(String, Vec<u8>)>) -> Result<(), EngineError>;

    fn handle_proposal(&self, _message: Vec<u8>, tx_pub: Sender<(String, Vec<u8>)>) -> Result<(), EngineError>;
}

#[test]
fn it_works() {}
