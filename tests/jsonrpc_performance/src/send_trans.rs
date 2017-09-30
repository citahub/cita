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

use crypto::*;
use hyper::Client;
use hyper::client::Response;
use hyper::status::StatusCode;
use jsonrpc_types::response::*;
use libproto::blockchain::UnverifiedTransaction;
use param::Param;
use serde_json;
use std::fmt;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time;
use trans::*;

static mut START_H: u64 = 1;

#[derive(Clone, Debug, PartialEq)]
pub enum TxCtx {
    Dup,
    SignErr,
    Correct,
    GetHeight,
}

#[derive(Clone, Debug)]
pub struct Sendtx {
    txnum: i32,
    threads: i32,
    ipandport: Vec<String>,
    totaltx: u64,
    quota: u64,
    code: String,
    contract_address: String,
    tx_type: TxCtx,
    analysis: bool,
    start_h: u64,
    tx_format_err: bool,
    sys_time: Arc<Mutex<time::SystemTime>>,
    curr_height: u64,
}

#[allow(non_snake_case)]
impl Sendtx {
    pub fn new(param: &Param, start_h: u64, analysis: bool) -> Self {
        let totaltx = param.txnum * param.threads;

        let tx_type = match param.tx_type.as_ref() {
            "Dup" => TxCtx::Dup,
            "SignErr" => TxCtx::SignErr,
            "GetHeight" => TxCtx::GetHeight,
            "Correct" | _ => TxCtx::Correct,
        };

        let trans = Sendtx {
            txnum: param.txnum,
            threads: param.threads,
            ipandport: param.ipandport.clone(),
            totaltx: totaltx as u64,
            quota: param.quota,
            code: param.code.clone(),
            contract_address: param.contract_address.clone(),
            tx_type: tx_type,
            analysis: analysis,
            start_h: start_h,
            tx_format_err: param.tx_format_err,
            sys_time: Arc::new(Mutex::new(time::SystemTime::now())),
            curr_height: 0,
        };
        trans
    }

    pub fn random_generation(&self) -> Result<KeyPair, Error> {
        Ok(KeyPair::gen_keypair())
    }

    pub fn send_data(url: String, method: Methods) -> Result<Response, i32> {

        let client = Client::new();
        let data = Trans::generate_tx_data(method);
        match client.post(&url).body(&data).send() {
            Ok(res) => Ok(res),
            Err(_) => Err(-1),
        }
    }

    pub fn parse_data(data: String) -> (String, bool) {
        let mut _ret = (String::new(), false);
        if let Ok(deserialized) = serde_json::from_str(&data) {

            let deserialized: RpcSuccess = deserialized;
            _ret = match deserialized.result {

                ResultBody::BlockNumber(hei) => (format!("{}", hei), true),
                ResultBody::Transaction(RpcTransaction) => {
                    let content = RpcTransaction.content;
                    if !content.to_vec().is_empty() { (String::new(), true) } else { (String::new(), false) }
                }

                ResultBody::FullBlock(full_block) => {
                    let body = full_block.body;
                    let transactions = body.transactions;
                    let time_stamp = full_block.header.timestamp;
                    (format!("{}|{}", transactions.len(), time_stamp), true)
                }

                ResultBody::TxResponse(TxResponse) => {
                    if TxResponse.status.to_uppercase().contains("OK") {
                        let hash = TxResponse.hash;
                        (format!("{:?}", hash), true)
                    } else {
                        (String::new(), false)
                    }
                }
                _ => (String::new(), false),
            }

        } else {
            _ret = (String::new(), false);
        }
        _ret
    }

    pub fn read_response(res: &mut Response) -> (String, bool) {
        let mut buf = String::new();
        let mut ret = (String::new(), false);
        if let Ok(len) = (*res).read_to_string(&mut buf) {
            buf.truncate(len);
            trace!("response = {}", buf);
            ret = Self::parse_data(buf);
        }
        ret
    }

    pub fn http_send_tx(&self, url: String, method: Methods, sync_send: mpsc::Sender<(u64, u64)>) {
        if let Ok(mut res) = Self::send_data(url.clone(), method) {
            match res.status {
                StatusCode::Ok => {
                    let parse_response = Self::read_response(&mut res);

                    if parse_response.1 {
                        let _ = sync_send.send((1, 0));
                    } else {
                        let _ = sync_send.send((0, 1));
                    }
                }
                _ => {
                    let _ = sync_send.send((0, 1));
                }
            }
        } else {
            let _ = sync_send.send((0, 1));
        }

    }


    pub fn send_tx(&self, thd_index: i32, sync_send: mpsc::Sender<(u64, u64)>, first_tx: Arc<Mutex<bool>>, sys_time: Arc<Mutex<time::SystemTime>>) {

        let v_url = self.get_url();
        let num = v_url.len();
        let mut _pos = 0;
        let mut txs = Vec::new();
        self.generation_txs(&mut txs);
        {
            let mut first_tx_lock = first_tx.lock().unwrap();
            if *first_tx_lock {
                let mut sys_time_lock = sys_time.lock().unwrap();
                *sys_time_lock = time::SystemTime::now();
                //获取第一次交易的高度
                unsafe {
                    START_H = Self::get_height(v_url[0].clone());
                }
                *first_tx_lock = false;
            }
        }

        for index in (0 + thd_index)..(self.txnum + thd_index) {
            _pos = (index as usize) % num;
            let url = v_url[_pos].clone();
            let tx = txs[(index - thd_index) as usize].clone();
            let method = match self.tx_format_err {
                false => Methods::Sendtx(tx).clone(),
                true => Methods::Formaterr(tx).clone(),
            };
            let sync_send_clone = sync_send.clone();
            self.http_send_tx(url, method, sync_send_clone);
        }
    }

    pub fn send_height_tx(&self, thd_index: i32, sync_send: mpsc::Sender<(u64, u64)>, first_tx: Arc<Mutex<bool>>, sys_time: Arc<Mutex<time::SystemTime>>) {

        let v_url = self.get_url();
        let num = v_url.len();
        let mut _pos = 0;
        {
            let mut first_tx_lock = first_tx.lock().unwrap();
            if *first_tx_lock {
                let mut sys_time_lock = sys_time.lock().unwrap();
                *sys_time_lock = time::SystemTime::now();
                //获取第一次交易的高度
                unsafe {
                    START_H = Self::get_height(v_url[0].clone());
                }
                *first_tx_lock = false;
            }
        }

        for index in (0 + thd_index)..(self.txnum + thd_index) {
            _pos = (index as usize) % num;
            let url = v_url[_pos].clone();
            let method = Methods::Height;
            let sync_send_clone = sync_send.clone();
            self.http_send_tx(url, method, sync_send_clone);
        }
    }

    //分配线程
    pub fn dispatch_thd(&mut self, sync_send: mpsc::Sender<(u64, u64)>) {

        let first_tx = Arc::new(Mutex::new(true));
        let v_url = self.get_url();
        self.curr_height = Self::get_height(v_url[0].clone());
        for index in 0..self.threads {
            let threadname = format!("dispatch_thd #{}", index);
            let t = Arc::new(self.clone());
            let sync_send = sync_send.clone();
            let first_tx_clone = first_tx.clone();
            let sys_time_clone = self.sys_time.clone();
            let ret = thread::Builder::new().name(threadname).spawn(move || if t.tx_type == TxCtx::GetHeight {
                                                                        t.send_height_tx(index, sync_send, first_tx_clone, sys_time_clone);
                                                                    } else {
                                                                        t.send_tx(index, sync_send, first_tx_clone, sys_time_clone);
                                                                    });
            if ret.is_err() {
                info!("thread create fail: {:?}", ret.unwrap_err());
            }
        }
    }

    pub fn get_url(&self) -> Vec<String> {
        let mut vurl = Vec::new();
        for ipandport in &self.ipandport {
            let v: Vec<&str> = ipandport.split(":").collect();
            let url = fmt::format(format_args!("http://{}:{}", v[0], v[1]));
            vurl.push(url);
        }
        vurl
    }

    pub fn get_first_tx_time(&self) -> time::SystemTime {
        *self.sys_time.lock().unwrap()
    }

    pub fn generation_txs(&self, txs: &mut Vec<UnverifiedTransaction>) {
        let mut pv_change = true; //pv是否改变
        let mut keypair = self.random_generation().unwrap();

        for _ in 0..self.txnum {
            if pv_change {
                keypair = self.random_generation().unwrap();
            }
            let frompv = keypair.privkey();
            let tx = match self.tx_type {
                TxCtx::Dup => {
                    //重复交易
                    pv_change = false;
                    Trans::generate_tx(&self.code, self.contract_address.clone(), frompv, self.curr_height + 88, self.quota, false)
                }
                TxCtx::SignErr => {
                    //交易签名错误
                    Trans::generate_tx(&self.code, self.contract_address.clone(), frompv, self.curr_height + 88, self.quota, true)
                }
                TxCtx::Correct => {
                    //正确交易
                    Trans::generate_tx(&self.code, self.contract_address.clone(), frompv, self.curr_height + 88, self.quota, false)
                }
                TxCtx::GetHeight => {
                    continue;
                }
            };
            txs.push(tx);
        }
    }

    pub fn get_txinfo_by_height(&self, url: String, h: u64) -> (i32, u64) {

        let mut num = -1;
        let mut time_stamp = 0;
        if let Ok(mut res) = Self::send_data(url.clone(), Methods::Blockbyheiht(h)) {
            match res.status {
                StatusCode::Ok => {
                    let parse_response = Self::read_response(&mut res);
                    if parse_response.1 {
                        let v: Vec<&str> = parse_response.0.split("|").collect();
                        num = i32::from_str(&(v[0])).unwrap();
                        time_stamp = u64::from_str(&(v[1])).unwrap();
                    }
                }
                _ => num = -2,
            }
        }
        (num, time_stamp)
    }

    pub fn get_height(url: String) -> u64 {
        let mut h = 0;

        if let Ok(mut res) = Self::send_data(url.clone(), Methods::Height) {
            match res.status {
                StatusCode::Ok => {
                    let parse_response = Self::read_response(&mut res);
                    if parse_response.1 {
                        h = u64::from_str(&(parse_response.0)).unwrap();
                    }
                }
                _ => panic!("jsonrpc connect fail!"),
            }
        }

        h
    }

    pub fn analysitxinfo(&mut self) {
        let v_url = self.get_url();
        let mut _url = v_url[0].clone();


        let mut tx_num = 0;
        let mut start_time_stamp = 0;
        let mut _end_time_stamp = 0;
        let mut h = self.start_h;
        let mut start_h = h;
        loop {
            let (blocknum, time_stamp) = self.get_txinfo_by_height(_url.clone(), h);
            tx_num += blocknum as u64;
            if tx_num == 0 {
                start_time_stamp = time_stamp;
                start_h = h;
                h += 1;
                continue;
            }
            _end_time_stamp = time_stamp;
            info!("height:{}, blocknum: {},  time stamp :{}", h, blocknum, time_stamp);
            if tx_num >= self.totaltx || (tx_num > 0 && blocknum == 0){
                break;
            }
            h += 1;
        }
        let secs = _end_time_stamp - start_time_stamp;
        let tps = if secs > 0 { (tx_num * 1000) as u64 / secs } else { tx_num as u64 };
        info!("tx_num: {}, start_h: {}, end_h: {}, use time: {} ms, tps: {}", tx_num, start_h, h, secs, tps);
    }


    pub fn start(&mut self) {
        //发送重复交易
        //发送签名错误交易
        //发送正常交易
        //发送获取block交易,统计正常交易的开始到结束的开始高度的每个block中交易数量、时间戳，总的汇总交易数量、time、tps
        if self.analysis {
            self.analysitxinfo();
        } else {
            let (sync_send, sync_recv) = mpsc::channel();
            self.dispatch_thd(sync_send);
            let v_url = self.get_url();
            let _url = v_url[0].clone();
            //jsonrpc返回成功的数量==入块的成功数退出循环
            self.wait(self.totaltx, sync_recv, _url);
        }


    }


    fn wait(&self, totaltx: u64, sync_recv: mpsc::Receiver<(u64, u64)>, url: String) {

        let mut sucess = 0;
        let mut fail = 0;
        let mut _end_h = 0;
        loop {
            let notify = sync_recv.recv_timeout(time::Duration::new(1, 0));
            //let notify = sync_recv.recv();
            if notify.is_ok() {
                sucess += notify.unwrap().0;
                fail += notify.unwrap().1;
                if (sucess + fail) >= totaltx {
                    //获取一次高度
                    let start = self.get_first_tx_time();
                    let sys_time = time::SystemTime::now();
                    let diff = sys_time.duration_since(start).expect("SystemTime::duration_since failed");
                    let mut secs = diff.as_secs();
                    let nanos = diff.subsec_nanos();
                    secs = secs * 1000 + (nanos / 1000000) as u64;
                    let tps = if secs > 0 { totaltx * 1000 / secs } else { totaltx };
                    _end_h = Self::get_height(url.clone());
                    let buf = if self.tx_format_err {
                        "jsonrpc(err format)"
                    } else {
                        match self.tx_type {
                            TxCtx::Dup => "jsonprc + consensus(dup tx)",
                            TxCtx::SignErr => "jsonrpc + auth + consensus(signerr)",
                            TxCtx::Correct => "jsonrpc + auth + consensus(corrent)",
                            TxCtx::GetHeight => "jsonrpc + chain(get height)",
                        }
                    };
                    unsafe {
                        info!("test type: {}, tx_num:{}, start_h: {}, end_h: {}, jsonrpc use time:{} ms, tps: {}", buf, totaltx, START_H, _end_h, secs, tps);
                    }
                    break;
                }
            }
        }
    }
}
