use Source;
use connection::Connection;
use libproto::{communication, factory, topics, submodules, SyncRequest, cmd_id, SyncResponse};
use libproto::blockchain::{Status, Block};
use libproto::communication::MsgType;
use protobuf::{parse_from_bytes, Message, RepeatedField};
use rand::{thread_rng, ThreadRng, Rng};
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, mpsc};
//use time::now;
use std::time::{Duration, Instant};
use util::snappy;

const SYNC_STEP: u64 = 20;
const SYNC_TIME_OUT: u64 = 60;

pub struct Synchronizer {
    tx_pub: mpsc::Sender<(String, Vec<u8>)>,
    con: Arc<Connection>,
    current_status: Status,
    global_status: Status,
    sync_end_height: u64, //current_status <= sync_end_status
    is_synchronizing: bool,
    latest_status_lists: BTreeMap<u64, VecDeque<u32>>,
    block_lists: BTreeMap<u64, Block>,
    rand: ThreadRng,
    sync_time_out: Instant,
}

unsafe impl Sync for Synchronizer {}
unsafe impl Send for Synchronizer {}

impl Synchronizer {
    pub fn new(tx_pub: mpsc::Sender<(String, Vec<u8>)>, con: Arc<Connection>) -> Self {
        Synchronizer {
            tx_pub: tx_pub,
            con: con,
            current_status: Status::new(),
            global_status: Status::new(),
            latest_status_lists: BTreeMap::new(),
            sync_end_height: 0,
            is_synchronizing: true,
            block_lists: BTreeMap::new(),
            rand: thread_rng(),
            sync_time_out: (Instant::now() - Duration::from_secs(SYNC_TIME_OUT)),
        }
    }

    pub fn update_current_status(&mut self, latest_status: Status) {
        debug!("sync: update_current_status: current height = {}, before height = {}, sync_end_height = {}", latest_status.get_height(), self.current_status.get_height(), self.sync_end_height);
        let old_height = self.current_status.get_height();
        self.latest_status_lists = self.latest_status_lists.split_off(&(latest_status.get_height() + 1));
        self.current_status = latest_status;
        self.broadcast_status();

        if self.current_status.get_height() <= old_height {
            //chain error
            self.is_synchronizing = true;
            let start_height = self.current_status.get_height() + 1;
            self.start_sync_req(start_height, 0);

        } else if self.current_status.get_height() < self.sync_end_height {
            //chain error.需要同步
            let start_height = self.current_status.get_height() + 1;
            let end_height = self.sync_end_height;
            self.start_sync_req(start_height, end_height + 1);
            self.sync_time_out = Instant::now();
            self.is_synchronizing = true;

        } else if self.current_status.get_height() == self.sync_end_height {
            let start_height = self.current_status.get_height() + 1;
            let second = self.current_status.get_height() + 2;
            debug!("sync: update_current_status: start_height = {}, second = {}, block_lists len = {}", start_height, second, self.block_lists.len());
            if self.block_lists.contains_key(&start_height) && self.block_lists.contains_key(&second) {
                self.submit_blocks();

            } else {
                self.start_sync_req(start_height, 0);
            }

            self.sync_time_out = Instant::now();
            self.is_synchronizing = true;
        } else {
            //大于时,表示最新.
            self.sync_end_height = self.current_status.get_height();
            self.is_synchronizing = false;
        }
    }

    pub fn update_global_status(&mut self, status: &Status, origin: u32) {
        debug!("sync: update_global_status: current height = {}, from node = {}, height = {}", self.current_status.get_height(), origin, status.get_height());
        let current_height = self.current_status.get_height();
        let old_global_status = self.global_status.clone();
        if self.global_status.get_height() < status.get_height() {
            self.global_status = status.clone();
        }

        if status.get_height() < current_height + 1 {
            //不同步
            self.is_synchronizing = false;

        } else if status.get_height() == current_height + 1 {
            //即将同步,保存
            self.add_latest_sync_lists(status.get_height(), origin);

            if self.global_status.get_height() > old_global_status.get_height() && self.is_synchronizing {
                self.start_sync_req(status.get_height(), status.get_height());

            } else {
                self.is_synchronizing = false;
            }

        } else {
            //发起同步
            self.add_latest_sync_lists(status.get_height(), origin);

            if self.sync_time_out.elapsed().as_secs() > SYNC_TIME_OUT || !self.is_synchronizing {
                self.sync_time_out = Instant::now();
                self.start_sync_req(current_height + 1, status.get_height());

            } else if self.global_status.get_height() > old_global_status.get_height() && self.is_synchronizing {
                self.start_sync_req(status.get_height(), status.get_height());
            }

            self.is_synchronizing = true;
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
            self.block_lists.insert(block.get_header().get_height(), block);
        }

        debug!("sync: process_sync: heights = {:?}", heights);
        self.submit_blocks();
    }


    pub fn receive(&mut self, from: Source, mut msg: communication::Message) {
        let t = msg.get_field_type();
        let cid = msg.get_cmd_id();
        let data = snappy::cita_decompress(msg.take_content());

        if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) && t == MsgType::STATUS {
            if let Ok(status) = parse_from_bytes::<Status>(&data) {
                match from {
                    Source::LOCAL => {
                        self.update_current_status(status);
                    }
                    Source::REMOTE => {
                        self.update_global_status(&status, msg.get_origin());
                    }
                }
            }
        } else if cid == cmd_id(submodules::CHAIN, topics::NEW_BLK) && t == MsgType::SYNC_RES {
            if let Ok(blocks) = parse_from_bytes::<SyncResponse>(&data) {
                match from {
                    Source::LOCAL => {
                        error!("sync: msg not parse!");
                    }
                    Source::REMOTE => {
                        self.process_sync(blocks);
                    }
                }
            }
        }
    }

    //发起同步请求
    fn start_sync_req(&mut self, start_height: u64, end_height: u64) {
        debug!("sync: start_sync_req: start_height = {}, end_height = {},current height = {}", start_height, end_height, self.current_status.get_height());
        let mut origin = 0;
        let mut end_height = end_height;
        let mut is_send = false;

        if let Some((height, origins)) =
            self.latest_status_lists
                .iter()
                .rfind(|&(_, origins)| origins.len() >= (2 / (3 * self.con.peers_pair.read().len())))
        {
            debug!("sync: start_sync_req: height = {}, origins = {:?}", height, origins);
            if let Some(origins) = self.latest_status_lists.get(height) {
                if *height > self.current_status.get_height() {
                    origin = origins[self.rand.gen_range(0, origins.len())];
                    if end_height == 0 {
                        end_height = *height + 1;
                    }
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
        debug!("sync: sync_strategy: current height = {},start_height = {}, end height = {}, to origin = {}", self.current_status.get_height(), start_height, end_height, origin);
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
            debug!("sync: send_sync_req:current height = {},  heights = {:?} ,origin {:?}, chain.sync: OperateType {:?}", self.current_status.get_height(), heights, origin, communication::OperateType::SINGLE);
            let mut sync_req = SyncRequest::new();
            sync_req.set_heights(heights);
            let msg = factory::create_msg_ex(submodules::CHAIN, topics::SYNC_BLK, communication::MsgType::SYNC_REQ, communication::OperateType::SINGLE, origin, sync_req.write_to_bytes().unwrap());
            self.con.broadcast(msg);
        }
    }


    fn broadcast_status(&mut self) {
        debug!("sync: broadcast status {:?}, {:?} to other nodes", self.current_status.get_height(), self.current_status.get_hash());
        let msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, self.current_status.clone().write_to_bytes().unwrap());
        self.con.broadcast(msg);
    }


    //提交连续的块
    fn submit_blocks(&mut self) {
        let mut height = self.current_status.get_height() + 1;
        debug!("sync: submit_blocks:submit_height = {}, current height = {}, sync_end_height = {}, block_lists = {}", height, self.current_status.get_height(), self.sync_end_height, self.block_lists.len());
        let mut blocks = vec![];
        self.block_lists = self.block_lists.split_off(&height);
        let end_height = height + SYNC_STEP;

        loop {
            if height <= end_height {
                if let Some(block) = self.block_lists.remove(&height) {
                    blocks.push(block);
                } else {
                    break;
                }
            } else {
                break;
            }
            height += 1;
        }

        if self.block_lists.contains_key(&::std::u64::MAX) && self.block_lists.len() == 1 {
            blocks.push(self.block_lists.remove(&::std::u64::MAX).unwrap());

        } else if let Some(block) = blocks.last() {
            self.block_lists.insert(block.get_header().get_height(), block.clone());
        }

        self.pub_blocks(blocks);
    }


    fn pub_blocks(&mut self, blocks: Vec<Block>) {
        if !blocks.is_empty() {
            let height = self.current_status.get_height();
            self.sync_end_height = height + (blocks.len() - 1) as u64;
            debug!("sync: pub_blocks: current height = {}, sync_end_height = {}, len = {}, ", self.current_status.get_height(), self.sync_end_height, blocks.len());
            let mut sync_res = SyncResponse::new();
            sync_res.set_blocks(RepeatedField::from_vec(blocks));
            let msg = factory::create_msg(submodules::CHAIN, topics::NEW_BLK, communication::MsgType::SYNC_RES, sync_res.write_to_bytes().unwrap());
            self.tx_pub.send(("net.blk".to_string(), msg.write_to_bytes().unwrap()));
        }
    }


    fn add_latest_sync_lists(&mut self, height: u64, origin: u32) {
        debug!("sync: add_sync_lists: current height = {}, from node = {}, height = {}", self.current_status.get_height(), origin, height);
        self.latest_status_lists.entry(height).or_insert(VecDeque::new()).push_back(origin);
    }
}
