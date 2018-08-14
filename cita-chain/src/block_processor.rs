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
use libproto::executor::ExecutedResult;
use std::sync::mpsc::Sender;
use std::sync::Arc;

/// Processing blocks and transaction storage
#[derive(Clone)]
pub struct BlockProcessor {
    pub chain: Arc<Chain>,
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
        self.chain.broadcast_current_status(&self.ctx_pub);
    }

    pub fn set_executed_result(&self, ret: ExecutedResult) {
        self.chain.set_executed_result(&ret, &self.ctx_pub);
    }

    pub fn reset_max_store_height(&self) {
        self.chain
            .set_max_store_height(self.chain.get_current_height());
    }

    pub fn signal_to_executor(&self) {
        self.chain.signal_to_executor(&self.ctx_pub)
    }
}
