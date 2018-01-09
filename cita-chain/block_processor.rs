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

use core::libchain::chain::Chain;
use libproto::executer::ExecutedResult;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct BlockProcessor {
    chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
}

impl BlockProcessor {
    pub fn new(chain: Arc<Chain>, ctx_pub: Sender<(String, Vec<u8>)>) -> Self {
        BlockProcessor {
            chain: chain,
            ctx_pub: ctx_pub,
        }
    }

    pub fn broadcast_current_status(&self) {
        self.chain.delivery_current_rich_status(&self.ctx_pub);
        if !self.chain.is_sync.load(Ordering::SeqCst) {
            self.chain.broadcast_status(&self.ctx_pub);
        }
    }

    pub fn broadcast_current_block(&self) {
        self.chain.broadcast_current_block(&self.ctx_pub);
    }

    pub fn set_excuted_result(&self, ret: ExecutedResult) {
        self.chain.set_excuted_result(&ret, &self.ctx_pub);
    }
}
