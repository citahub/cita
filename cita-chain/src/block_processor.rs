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

use core::libchain::chain::Chain;
use libproto::executor::ExecutedResult;
use license::lic_verify::{CurrentHightReq, GenesisBlockHashReq, LicVerifyClient};
use pubsub::channel::Sender;
use std::sync::Arc;

/// Processing blocks and transaction storage
#[derive(Clone)]
pub struct BlockProcessor {
    pub chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
    lic_verify_client: LicVerifyClient,
}

impl BlockProcessor {
    pub fn new(
        chain: Arc<Chain>,
        ctx_pub: Sender<(String, Vec<u8>)>,
        lic_verify_client: LicVerifyClient,
    ) -> Self {
        // Set genesis block hash for license verifty
        if let Some(genesis_hash) = chain.genesis_block_hash() {
            let req = GenesisBlockHashReq::new(genesis_hash);
            lic_verify_client.set_genesis_block_hash(req);
            info!("Set genesis block hash!");
        } else {
            info!("This is a new chain, cannot get genesis block hash at the begining!");
        }
        BlockProcessor {
            chain,
            ctx_pub,
            lic_verify_client,
        }
    }

    pub fn broadcast_current_status(&self) {
        self.chain.broadcast_current_status(&self.ctx_pub);
    }

    pub fn set_executed_result(&self, ret: &ExecutedResult) {
        // Send the block hash to license verify when the genesis block (0) has stored
        if self.chain.get_current_height() == 1 {
            let genesis_hash = self
                .chain
                .genesis_block_hash()
                .expect("Can not　get genesis hash from this chain!");
            let req = GenesisBlockHashReq::new(genesis_hash);
            self.lic_verify_client.set_genesis_block_hash(req);
            info!("This is new chain, Set genesis block hash!");
        }

        // Set current hight to license verify for each block
        let req = CurrentHightReq::new(self.chain.get_current_height());
        self.lic_verify_client.set_current_hight(req);
        self.chain.set_executed_result(ret, &self.ctx_pub);
    }

    pub fn reset_max_store_height(&self) {
        self.chain
            .set_max_store_height(self.chain.get_current_height());
    }

    pub fn signal_to_executor(&self) {
        self.chain.signal_to_executor(&self.ctx_pub)
    }
}
