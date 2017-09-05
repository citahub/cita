extern crate util;
extern crate bincode;
extern crate rustc_serialize;

mod wal;

use util::Address;
use wal::Wal;
use bincode::{serialize, deserialize, Infinite};
use std::sync::atomic::{AtomicBool, Ordering};

const DATA_PATH: &'static str = "DATA_PATH";
const LOG_TYPE_AUTHORITIES: u8 = 1;

#[derive(Debug)]
pub struct AuthorityManage {
    pub authorities: Vec<Address>,
    pub authority_n: usize,
    authorities_log: Wal,
    pub authorities_old: Vec<Address>,
    pub authority_n_old: usize,
    pub authority_h_old: usize,
    is_load: AtomicBool,
}

#[allow(unused_imports,dead_code)]
impl AuthorityManage {
    pub fn new() -> Self {
        //判断authorities有没存储，有就读取
        let logpath = ::std::env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/authorities";

        let mut authority_manage = AuthorityManage{
            authorities: Vec::new(),
            authority_n: 0,
            authorities_log: Wal::new(&*logpath).unwrap(),
            authorities_old: Vec::new(),
            authority_n_old: 0,
            authority_h_old: 0,
            is_load: AtomicBool::new(true),
        };

        let vec_out = authority_manage.authorities_log.load();
        if !vec_out.is_empty(){
            //out 转换成authorities;
            if let Ok((h,authorities)) = deserialize(&(vec_out[0].1)) {
                let auth:Vec<Address> = authorities;
                authority_manage.authorities_old.extend_from_slice(&auth);
                authority_manage.authority_n_old = authority_manage.authorities_old.len();
                authority_manage.authority_h_old = h;
                println!("{}, {:?}", h, auth);
            }
        }

        authority_manage
    }

    pub fn receive_authorities_list(&mut self, height: usize,authorities: Vec<Address>) {
        if !self.is_load.load(Ordering::SeqCst) {
            self.authorities_old = self.authorities.clone();
            self.authority_n_old = self.authority_n;
            let bmsg = serialize(&(height,&self.authorities), Infinite).unwrap();
            let _ = self.authorities_log.save(LOG_TYPE_AUTHORITIES, &bmsg);
        }
        else{
            self.is_load.store(false, Ordering::SeqCst);
        }
        self.authorities.clear();
        self.authorities.extend_from_slice(&authorities);
        self.authority_n = self.authorities.len();
    }
}
