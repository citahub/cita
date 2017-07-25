use crypto::*;
use util::hash::H256 as Hash256;
use serde_types::hash::H256;
use hyper::Client;
use hyper::client::Response;
use hyper::status::StatusCode;
use std::io::Read;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{self};
use std::thread;
use serde_json;
use jsonrpc_types::rpc_response::*;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use util::FixedHash;

use core::param::Param;
use core::trans::*;
use std::fmt;

static mut EXIT: bool = false;
#[allow(dead_code,unused_assignments)]
#[derive(Clone,Debug,PartialEq)]
pub enum Action{
    Create,
    Call,
}

#[allow(dead_code,unused_assignments)]
#[derive(Clone,Debug)]
pub struct Sendtx{
    txnum: i32,
    threads: i32,
    pvfile: String,
    create: i32,
    ipandport: Vec<String>,
    code: String,
    first: Arc<Mutex<i32>>,
    totaltx: u64,
}

#[allow(dead_code,unused_variables,unused_assignments,non_snake_case,unused_mut)]
impl Sendtx{

    pub fn new(param: &Param) -> Self {
       
        //let (sync_send, sync_recv) = mpsc::channel();
        let totaltx = param.txnum * param.threads;
        let trans = Sendtx {
            txnum: param.txnum,
            threads: param.threads,
            pvfile: "".to_string(),
            create: 1,
            ipandport: param.ipandport.clone(),
            code: param.code.clone(),
            first: Arc::new(Mutex::new(0)),
            totaltx: totaltx as u64,
        };
        trans
    }
    
    pub fn generate_primacron(&self) -> Result<KeyPair, Error> {
        
        let path = Path::new(&self.pvfile);
         let mut file = match File::open(&path){
            Ok(file) => file,
            Err(_) => panic!("open {:?} fail", path),
        };
        
        let mut contents = String::new();
        match file.read_to_string(&mut contents){
                Err(_) => panic!("read fail "),
                Ok(_) => println!("read successfully.[{}]",contents),                
        }
        let privkey = Hash256::from_str(contents.as_str()).unwrap();
        KeyPair::from_privkey(H256::from(privkey))
    }
    
    pub fn random_generation(&self) ->Result<KeyPair, Error> {
        let test1_privkey = Hash256::random();
        KeyPair::from_privkey(H256::from(test1_privkey))
    }  

    pub fn send_data(&self, url: String, method: Methods) -> Response {
  
        let client = Client::new();
        let data = Trans::generate_tx_data(method);
        client.post(&url).body(&data).send().unwrap()
         
    }

    pub fn parse_response(res: &mut Response) -> (String ,bool) {
        let mut buf = String::new();
        let mut ret = (String::new(),false);
        if let Ok(len) = (*res).read_to_string(&mut buf) {
            buf.truncate(len);
            if let Ok(deserialized) = serde_json::from_str(&buf) {

                let deserialized: RpcResponse = deserialized;
                ret = match deserialized.result {
                    
                    ResponseBody::BlockNumber(hei) => (format!("{}",hei),true),
                    ResponseBody::Transaction(RpcTransaction) => {
                        //let transaction = RpcTransaction.transaction;
                        let content = RpcTransaction.content;
                        if !content.to_vec().is_empty() {
                            (String::new(),true)
                        }
                        else{
                            (String::new(),false)
                        }
                    },
                    
                    ResponseBody::FullBlock(full_block) => {
                        //let block = full_block.block;
                        let body = full_block.body;
                        let transactions = body.transactions;
                        (format!("{}",transactions.len()),true)
                    },
                    
                    ResponseBody::TxResponse(TxResponse) => {
                        if TxResponse.status == "4:OK" {
                            let hash = TxResponse.hash;
                            (format!("{:?}",hash.0),true)                                            
                        }
                        else{
                          println!("cita_sendTransaction : {:?}", buf);   
                          (String::new(),false)     
                        }
                    },
                    
                    ResponseBody::Receipt(Receipt) => {

                        match Receipt.contract_address{
                            Some(contract_address) =>(format!("{:?}",contract_address.0),true),
                            None => (String::new(),false),
                        }
                    },

                    _ => (String::new(),false),
                }
                
            }
            else{
                println!("jsonrpc response: {:?}",buf);
                ret = (String::new(),false);
            }
        }
        ret
    }


    pub fn send_tx(&self, action: Action, sync_send: mpsc::Sender<(u64,u64)>, send_h: mpsc::Sender<u64>, sender: String){

        let mut sucess = 0;
        let mut fail = 0;
        let v_url = self.get_url();
        let num = v_url.len();
        let mut pos = 0;
        for index in 0..self.txnum {
            pos = (index as usize)%num; 
            let url = v_url[pos].clone();
            //let mut frompv = H256::from(Hash256::new());
            let keypair = self.random_generation().unwrap();
            let frompv = keypair.privkey();
            let frompk = keypair.pubkey();
            let tx = match action {
                Action::Create => {
                    Trans::generate_tx(&self.code,sender.clone(),frompv, frompk)
                },
                Action::Call => {
                    //读取合约地址
                    Trans::generate_tx(&self.code,sender.clone(),&frompv, frompk)
                },
            };
            {
                let mut firsttx = self.first.lock().unwrap();
                if *firsttx == 0 {
                    let curh = self.get_height(url.clone());
                    send_h.send(curh).unwrap();
                    *firsttx = 1;
                }
            }
            let mut res = self.send_data(url.clone(),Methods::Sendtx(tx));
            match res.status{
                StatusCode::Ok => {
                    let parse_response = Self::parse_response(&mut res);
                    if parse_response.1 {
                        if action == Action::Create {
                            //存储返回hash
                            let path = Path::new("hash.txt");
                            let mut file = match File::create(&path) {
                                Err(_) => panic!("create fail"),
                                Ok(file) => file,
                            };
                            
                            match file.write_all(parse_response.0.as_bytes()){
                                Err(_) => println!("write fail"),
                                Ok(_) => (),
                            }                          
                        }
                        sucess = sucess + 1;    
                    }
                    else{
                        fail = fail + 1;  
                    }
                },
                _ => println!("jsonrpc connect [{}] fail!", url),
            }
            
        }
        
        println!("sucess {}, fail {}", sucess, fail);
        //channel 发送sucess, fail
        let _ = sync_send.send((sucess, fail));
    }

    pub fn get_height(&self, url: String) -> u64{
        let mut h = 0;
        let mut res = self.send_data(url.clone(),Methods::Height);
        match res.status{
            StatusCode::Ok => {
                let parse_response = Self::parse_response(&mut res);
                if parse_response.1 {
                    h = u64::from_str(&(parse_response.0)).unwrap();  
                }
            },
            _ => panic!("jsonrpc connect fail!"),
        }       
        h
    }

    pub fn get_txnum_by_height(&self, url: String, h: u64) -> i32{

        let mut num = -1;

        let mut res = self.send_data(url.clone(),Methods::Blockbyheiht(h));
        match res.status{
            StatusCode::Ok => {
                let parse_response = Self::parse_response(&mut res);
                if parse_response.1 {
                    num = i32::from_str(&(parse_response.0)).unwrap();    
                }
            },
            _ => num = -2,
        }
        num
    }

    pub fn get_contract_address(&self) -> String{

        let mut address = "".to_string();
        let v_url = self.get_url();
        
        let mut file = match File::open("hash.txt"){
            Ok(file) => file,
            Err(_) => panic!("open [{}] fail", "has.txt"),
        };
        
        let mut contents = String::new();
        match file.read_to_string(&mut contents){
                Err(_) => panic!("read fail"),
                Ok(_) => println!("read hash.[{}]",contents),                
        }

        for url in &v_url {
            let mut res = self.send_data(url.clone(),Methods::Receipt(contents.clone()));
            match res.status{
                StatusCode::Ok => {
                    let parse_response = Self::parse_response(&mut res);
                    if parse_response.1 {
                        address = parse_response.0;
                        break;
                    }
                },
                _ => (),
            }
        }
        address
    }    

    //创建合约线程

    pub fn dispatch_create_contracts_thd(&self, sync_send: mpsc::Sender<(u64,u64)>, send_h: mpsc::Sender<u64>){

        for index in 0..self.threads {
            let threadname = format!("create_contracts_thd #{}", index);
            let t = Arc::new(self.clone());
            let sync_send = sync_send.clone();
            let send_h = send_h.clone();
            let _ = thread::Builder::new().name(threadname).spawn(move || {
                t.send_tx( Action::Create, sync_send, send_h, "".to_string());
            });
        }
    }


    //执行合约的交易线程
    pub fn dispatch_send_thd(&self, sync_send: mpsc::Sender<(u64,u64)>, send_h: mpsc::Sender<u64>){

        //获取合约地址
        let sender = self.get_contract_address();

        for index in 0..self.threads{
            //发送交易
            //获取高度
            let threadname = format!("send_tx_thd #{}", index);
            let t = Arc::new(self.clone());
            let sync_send = sync_send.clone();
            let send_h = send_h.clone();
            let sender = sender.clone();
            let _ = thread::Builder::new().name(threadname).spawn(move || {
                t.send_tx(Action::Call, sync_send, send_h, sender);
            });
        }            
    }

    pub fn analysitxinfo(&self, recv: mpsc::Receiver<(u64,u64)>, recvh: mpsc::Receiver<u64>){
        let mut sucess = 0;
        let mut fail = 0;
        let s = Arc::new(self.clone());
        let v_url = self.get_url();
        let url_num = v_url.len();
        let mut pos = 0;
        let mut url = v_url[pos].clone();
        let _ = thread::Builder::new().name("analysistransinfo".into()).spawn(move || {
                let mut flag = 0;
                let mut txnum = 0;
                let mut starth = 0;
                let mut endh = 0;
                let mut sys_time = time::SystemTime::now();
                let cl = s.clone();
                loop{
                    
                    match endh{
                        0 => {
                            //endh = cl.get_height(url.clone());
                            let h = recvh.recv_timeout(time::Duration::new(0, 0));
                            
                            if h.is_ok() {
                               endh = h.unwrap();
                               println!("================== {}", endh);
                            }
                            sys_time = time::SystemTime::now();
                        },
                        _ => {
                            let blocknum = cl.get_txnum_by_height(url.clone(), endh);
                            if blocknum >= 0 {
                                if blocknum > 0 && starth == 0 {
                                    starth = endh;
                                }
                                endh = endh + 1;
                                txnum = txnum + blocknum;
                                println!("current tx num: {}, use time:{}s", txnum, sys_time.elapsed().unwrap().as_secs());
                            }
                            else if blocknum == -2{
                                pos += 1;
                            }
                        },
                    }

                    match pos{
                        x if x < url_num => url = v_url[pos].clone(),
                        _ => panic!("connect jsonrpc fail"),
                    }

                    let notify = recv.recv_timeout(time::Duration::new(0, 0));
                    if notify.is_ok() {
                        sucess = notify.unwrap().0;
                        fail = notify.unwrap().1;
                    }
                    
                    if (sucess + fail) >= s.totaltx as u64 && txnum as u64  >= sucess {
                        unsafe{EXIT = true;}
                        break;
                    }
                    
                }
                println!("send tx num:{}, start h:{}, end h:{}, Total time:{}s",txnum, starth, endh, sys_time.elapsed().unwrap().as_secs());
        });

    }

    pub fn get_url(&self) -> Vec<String>{
        let mut vurl = Vec::new();
        for ipandport in &self.ipandport {
            let v: Vec<&str> = ipandport.split(":").collect();
            let url = fmt::format(format_args!("http://{}:{}",v[0],v[1]));  
            vurl.push(url);      
        }
        vurl
    }

    pub fn start(&self, category: u8){

        let (sync_send, sync_recv) = mpsc::channel();
        let (send_h,recv_h) = mpsc::channel();
        let (send, recv) = mpsc::channel();

        match category {
            1 =>self.dispatch_create_contracts_thd(sync_send,send_h),
            2|_ =>{
                self.dispatch_send_thd(sync_send,send_h);  
            },
        } 

        //通过高度获取交易是块交易的数量
        self.analysitxinfo(recv, recv_h);

        //发送完成输出完成的总数
        //jsonrpc返回成功的数量==入块的成功数退出循环
        Self::wait(self.totaltx, sync_recv,send);
        
    }

    
    fn wait(totaltx: u64, sync_recv: mpsc::Receiver<(u64,u64)>, send: mpsc::Sender<(u64,u64)>){

        let mut sucess = 0;
        let mut fail   = 0;
        let sys_time = time::SystemTime::now();

        loop{
            let notify = sync_recv.recv_timeout(time::Duration::new(1, 0));
            if notify.is_ok() {
                sucess += notify.unwrap().0;
                fail += notify.unwrap().1;
                if (sucess + fail) >= totaltx {
                    send.send((sucess,fail)).unwrap();
                    println!(" jsonrpc use time:{}s",  sys_time.elapsed().unwrap().as_secs());
                    println!("write successfully.[{}]",sucess);
                }
            }
            unsafe{
                if EXIT {
                    break;
                }  
            }
        }
    }


}
