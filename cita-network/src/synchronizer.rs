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

use crate::mq_agent::{MqAgentClient, PubMessage};
use crate::node_manager::{BroadcastReq, NodesManagerClient, SingleTxReq};
use libproto::blockchain::{Block, Status};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::{Message, OperateType, SyncRequest, SyncResponse};
use libproto::{TryFrom, TryInto};
use pubsub::channel::{unbounded, Receiver, Sender};
use rand::{thread_rng, Rng, ThreadRng};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::convert::Into;
use std::time::{Duration, Instant};
use std::u8;
use tentacle::SessionId;

const SYNC_STEP: u64 = 20;
const SYNC_TIME_OUT: u64 = 9;

/// Get messages and determine if need to synchronize or broadcast the current node status
pub struct Synchronizer {
    mq_client: MqAgentClient,
    nodes_mgr_client: NodesManagerClient,
    current_status: Status,
    global_status: Status,
    sync_end_height: u64, //current_status <= sync_end_status
    is_synchronizing: bool,
    latest_status_lists: BTreeMap<u64, VecDeque<u32>>,
    block_lists: BTreeMap<u64, Block>,
    rand: ThreadRng,
    // Timer for each height processing
    remote_sync_time_out: Instant,
    /// local sync error
    local_sync_count: u8,
    sync_client: SynchronizerClient,
    msg_receiver: Receiver<SynchronizerMessage>,
}

unsafe impl Sync for Synchronizer {}
unsafe impl Send for Synchronizer {}

impl Synchronizer {
    pub fn new(mq_client: MqAgentClient, nodes_mgr_client: NodesManagerClient) -> Self {
        let (tx, rx) = unbounded();
        let client = SynchronizerClient::new(tx);
        Synchronizer {
            mq_client,
            nodes_mgr_client,
            current_status: Status::new(),
            global_status: Status::new(),
            latest_status_lists: BTreeMap::new(),
            sync_end_height: 0,
            is_synchronizing: false,
            block_lists: BTreeMap::new(),
            rand: thread_rng(),
            remote_sync_time_out: (Instant::now() - Duration::from_secs(SYNC_TIME_OUT)),
            local_sync_count: 0,
            sync_client: client,
            msg_receiver: rx,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(msg) = self.msg_receiver.recv() {
                msg.handle(self);
            }
        }
    }

    pub fn client(&self) -> SynchronizerClient {
        self.sync_client.clone()
    }

    /// After receiving the `Chain >> Status`, it is processed as follows:
    /// 1. The chain height suddenly becomes lower than the original,
    ///    which means that the library is deleted, that is,
    ///    the synchronization is restarted from the received height.
    /// 2. The height of the chain is greater than or equal to the original height,
    ///    less than or equal to the height that the network has synchronized
    ///        - It is equal to the original height more than 2 times, indicating that
    ///          the data in the chain or executor is lost, and the block information is
    ///          sent again from the buffer. If no buffer exists, the data is requested again from other nodes.
    /// 3. The height is greater than or equal to the global height, indicating that the synchronization
    ///     has been completed and the synchronization status is exited.
    /// 4. The height is less than the global height, indicating that synchronization needs to be continued
    /// 5. Other unknown state
    pub fn update_current_status(&mut self, latest_status: Status) {
        debug!(
            "sync: update_current_status: current height = {}, \
             before height = {}, \
             sync_end_height = {}",
            latest_status.get_height(),
            self.current_status.get_height(),
            self.sync_end_height
        );
        let old_height = self.current_status.get_height();
        let new_height = latest_status.get_height();

        if new_height == old_height {
            if self.local_sync_count < u8::MAX {
                // Chain height does not increase
                self.local_sync_count += 1;
            }
        } else {
            self.local_sync_count = 0;
            self.remote_sync_time_out = Instant::now();
        }

        self.latest_status_lists = self
            .latest_status_lists
            .split_off(&(latest_status.get_height() + 1));
        self.current_status = latest_status;
        self.broadcast_status();
        self.prune_block_list_cache(new_height + 1);

        info!(
            "current: {}, sync_end: {}, global: {}, sync: {}",
            new_height,
            self.sync_end_height,
            self.global_status.get_height(),
            self.is_synchronizing
        );

        if new_height < old_height {
            // Chain error, may be a problem with the database, such as the database was deleted
            let start_height = new_height + 1;

            if self.block_lists.contains_key(&start_height) && !self.block_lists.is_empty() {
                self.submit_blocks();
            } else {
                self.start_sync_req(start_height);
            }
        } else if new_height < self.sync_end_height {
            // In synchronization, or loss of sync data, need to resend
            debug!(
                "Syncing: update_current_status: height = {}, block_lists len = {}",
                new_height,
                self.block_lists.len()
            );

            if !self.block_lists.is_empty()
                && new_height < self.sync_end_height
                && self.local_sync_count >= 3
            {
                // Chain height does not increase, loss data or data is invalid,
                // send cache to executor and chain, and clear cache
                self.local_sync_count = 0;
                self.block_lists.clear();
                self.start_sync_req(new_height + 1);
                info!("More than 3 times, clear the cache");
            }

            self.is_synchronizing = true;
        } else if new_height >= self.global_status.get_height() {
            if self.is_synchronizing {
                self.is_synchronizing = false;
                self.sync_end_height = 0;
                self.block_lists.clear();
            }
        } else if new_height < self.global_status.get_height() {
            // If the block height is equal to the maximum height that has already been synchronized,
            // perform the synchronization operation first to see if it is the latest in the chain
            if self.is_synchronizing {
                let start = match self.block_lists.iter().last() {
                    Some((height, _)) => *height + 1,
                    None => new_height + 1,
                };
                self.start_sync_req(start);
            }
        } else {
            info!("...Can't reach this");
        }
    }

    /// 1. Global height is less than current height + 1, no action
    /// 2. Global height is equal to current height + 1
    ///     - Start syncing when it is not in sync and timeout
    /// 3. Global height is greater than current height + 1, Timeout or not in sync, initiate synchronization
    pub fn update_global_status(&mut self, status: &Status, origin: u32) {
        debug!(
            "sync: update_global_status: current height = {}, from node = {}, height = {}",
            self.current_status.get_height(),
            origin,
            status.get_height()
        );
        let current_height = self.current_status.get_height();
        if self.global_status.get_height() < status.get_height() {
            self.global_status = status.clone();
        }

        match status.get_height() {
            status_height if status_height == current_height + 1 => {
                // A node on the chain blocks out, synchronizing the latest block
                self.add_latest_sync_lists(status_height, origin);

                if self.remote_sync_time_out.elapsed().as_secs() > SYNC_TIME_OUT
                    && !self.is_synchronizing
                {
                    self.start_sync_req(status_height);
                }
            }
            status_height if status_height > current_height + 1 => {
                // The node is far behind the data on the chain and initiates a synchronization request
                self.add_latest_sync_lists(status.get_height(), origin);

                if self.remote_sync_time_out.elapsed().as_secs() > SYNC_TIME_OUT
                    || !self.is_synchronizing
                {
                    self.start_sync_req(current_height + 1);
                }
            }
            _ => {
                // status_height < current_height + 1
                // The current node is the latest height and does not need to be synchronized
            }
        }
    }

    pub fn is_synchronizing(&self) -> bool {
        self.is_synchronizing
    }

    pub fn process_sync(&mut self, mut blocks: SyncResponse) {
        let blocks = blocks.take_blocks();
        debug!("sync: process_sync: blocks len = {}", blocks.len());

        let mut heights = vec![];
        for block in blocks.into_iter() {
            heights.push(block.get_header().get_height());
            self.block_lists
                .insert(block.get_header().get_height(), block);
        }

        debug!("sync: process_sync: heights = {:?}", heights);
        self.submit_blocks();
    }

    // Initiate a sync request
    fn start_sync_req(&mut self, start_height: u64) {
        debug!(
            "sync: start_sync_req: start_height = {}, current height = {}",
            start_height,
            self.current_status.get_height()
        );
        let mut origin = 0;
        let mut end_height = start_height;
        let mut is_send = false;
        let current_height = self.current_status.get_height();

        if let Some((height, origins)) = self
            .latest_status_lists
            .iter()
            .rfind(|&(_, origins)| !origins.is_empty())
        {
            debug!(
                "sync: start_sync_req: height = {}, origins = {:?}",
                height, origins
            );
            if let Some(origins) = self.latest_status_lists.get(height) {
                if *height > current_height {
                    origin = origins[self.rand.gen_range(0, origins.len())];
                    end_height = current_height + SYNC_STEP;
                    is_send = true;
                }
            }
        }

        if is_send {
            self.sync_strategy(start_height, end_height, origin);
        }
    }

    fn sync_strategy(&self, start_height: u64, end_height: u64, origin: u32) {
        //current height = 155,start_height = 156, end height = 160, to origin = 1
        debug!(
            "sync: sync_strategy: current height = {}, \
             start_height = {}, \
             end height = {}, \
             to origin = {}",
            self.current_status.get_height(),
            start_height,
            end_height,
            origin
        );
        if start_height >= self.current_status.get_height() && start_height <= end_height {
            let mut start_height = start_height;
            let mut step_sum = SYNC_STEP;
            let mut heights = vec![];

            while start_height <= end_height {
                if step_sum == 0 {
                    step_sum = SYNC_STEP;
                    self.send_sync_req(heights, origin);
                    heights = vec![];
                }
                heights.push(start_height);

                step_sum -= 1;
                start_height += 1;
            }
            self.send_sync_req(heights, origin);
        }
    }

    fn send_sync_req(&self, heights: Vec<u64>, origin: u32) {
        if !heights.is_empty() {
            debug!(
                "sync: send_sync_req:current height = {}, \
                 heights = {:?}, \
                 origin {:?}, \
                 chain.sync: OperateType {:?}",
                self.current_status.get_height(),
                heights,
                origin,
                OperateType::Single
            );
            let mut sync_req = SyncRequest::new();
            sync_req.set_heights(heights);
            let msg = Message::init(OperateType::Single, origin, sync_req.into());

            self.nodes_mgr_client.send_message(SingleTxReq::new(
                SessionId::from(origin as usize),
                routing_key!(Synchronizer >> SyncRequest).into(),
                msg,
            ));
        }
    }

    fn broadcast_status(&mut self) {
        debug!(
            "sync: broadcast status {:?}, {:?} to other nodes",
            self.current_status.get_height(),
            self.current_status.get_hash()
        );
        let msg: Message = self.current_status.clone().into();

        self.nodes_mgr_client.broadcast(BroadcastReq::new(
            routing_key!(Synchronizer >> Status).into(),
            msg,
        ));
    }

    // Submit synchronization information
    fn submit_blocks(&mut self) {
        let mut height = self.current_status.get_height() + 1;
        debug!(
            "sync: submit_blocks:submit_height = {},\
             current height = {},\
             sync_end_height = {},\
             block_lists = {}",
            height,
            self.current_status.get_height(),
            self.sync_end_height,
            self.block_lists.len()
        );
        let mut blocks = vec![];
        let end_height = height + SYNC_STEP;

        loop {
            if height <= end_height {
                if let Some(block) = self.block_lists.get(&height) {
                    blocks.push(block.to_owned());
                } else {
                    break;
                }
            } else {
                break;
            }
            height += 1;
        }

        if let Some(block) = blocks.last() {
            if let Some(header) = block.header.as_ref() {
                let height = header.get_height() - 1;

                if height > self.sync_end_height {
                    self.sync_end_height = height;
                }
            }
        }

        if self.block_lists.contains_key(&::std::u64::MAX) {
            blocks.push(self.block_lists.remove(&::std::u64::MAX).unwrap());
        }

        self.pub_blocks(blocks);
        self.is_synchronizing = true;
        self.remote_sync_time_out = Instant::now();
    }

    fn pub_blocks(&mut self, blocks: Vec<Block>) {
        if !blocks.is_empty() {
            debug!(
                "sync: pub_blocks: current height = {}, \
                 sync_end_height = {}, \
                 len = {}, ",
                self.current_status.get_height(),
                self.sync_end_height,
                blocks.len()
            );
            let mut sync_res = SyncResponse::new();
            sync_res.set_blocks(blocks.into());
            let msg: Message = sync_res.into();
            self.mq_client.pub_sync_blocks(PubMessage::new(
                routing_key!(Net >> SyncResponse).into(),
                msg.try_into().unwrap(),
            ));
        }
    }

    fn add_latest_sync_lists(&mut self, height: u64, origin: u32) {
        debug!(
            "sync: add_sync_lists: current height = {}, \
             from node = {}, \
             height = {}",
            self.current_status.get_height(),
            origin,
            height
        );
        let insert_is_ok = self
            .latest_status_lists
            .entry(height)
            .or_insert_with(VecDeque::new)
            .iter()
            .fold(HashSet::new(), |mut set, item| {
                set.insert(item);
                set
            })
            .insert(&origin);
        if insert_is_ok {
            self.latest_status_lists
                .entry(height)
                .or_insert_with(VecDeque::new)
                .push_back(origin);
        }
    }

    /// Prune block on btreemap
    fn prune_block_list_cache(&mut self, height: u64) {
        self.block_lists = self.block_lists.split_off(&height);
    }
}

#[derive(Clone)]
pub struct SynchronizerClient {
    sender: Sender<SynchronizerMessage>,
}

impl SynchronizerClient {
    pub fn new(sender: Sender<SynchronizerMessage>) -> Self {
        SynchronizerClient { sender }
    }

    pub fn handle_local_status(&self, msg: SynchronizerMessage) {
        self.send_msg(msg);
    }

    pub fn handle_remote_status(&self, msg: SynchronizerMessage) {
        self.send_msg(msg);
    }

    pub fn handle_remote_response(&self, msg: SynchronizerMessage) {
        self.send_msg(msg);
    }

    fn send_msg(&self, msg: SynchronizerMessage) {
        match self.sender.try_send(msg) {
            Ok(_) => {
                debug!("Send message to Synchronizer Success");
            }
            Err(err) => {
                warn!("Send message to Synchronizer failed : {:?}", err);
            }
        }
    }
}

pub struct SynchronizerMessage {
    key: String,
    data: Vec<u8>,
}

impl SynchronizerMessage {
    pub fn new(key: String, data: Vec<u8>) -> Self {
        SynchronizerMessage { key, data }
    }

    pub fn handle(self, service: &mut Synchronizer) {
        let mut msg = Message::try_from(&self.data).unwrap();
        let origin = msg.get_origin();
        let rt_key = RoutingKey::from(&self.key);
        match rt_key {
            routing_key!(Chain >> Status) => {
                if let Some(status) = msg.take_status() {
                    service.update_current_status(status);
                };
            }
            routing_key!(Synchronizer >> Status) => {
                if let Some(status) = msg.take_status() {
                    service.update_global_status(&status, origin);
                };
            }
            routing_key!(Synchronizer >> SyncResponse) => {
                if let Some(blocks) = msg.take_sync_response() {
                    service.process_sync(blocks);
                };
            }
            _ => {
                error!("receive: unexpected data key = {:?}", self.key);
            }
        }
    }
}
