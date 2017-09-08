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

extern crate util;
extern crate bincode;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate libproto;
mod wal;

use bincode::{serialize, deserialize, Infinite};
use libproto::blockchain::RichStatus;
use std::collections::HashMap;
use std::convert::From;
use util::Address;
use wal::Wal;

const DATA_PATH: &'static str = "DATA_PATH";
const LOG_TYPE_AUTHORITIES: u8 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthManageInfo {
    pub nodes: Vec<Address>,
    pub roles: HashMap<Address, Vec<String>>,
}

impl AuthManageInfo {
    pub fn new() -> AuthManageInfo {
        AuthManageInfo {
            nodes: vec![],
            roles: HashMap::new(),
        }

    }

    pub fn into(self, height: u64, hash: Vec<u8>) -> RichStatus {
        let mut status = RichStatus::new();
        status.set_data(serde_json::to_string(&self).expect("rich status serde error!"));
        status.set_hash(hash);
        status.set_height(height);
        status
    }

    pub fn into_string(self) -> String {
        serde_json::to_string(&self).expect("rich status serde error!")
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.roles.clear();
    }
}

impl From<RichStatus> for AuthManageInfo {
    fn from(auth: RichStatus) -> AuthManageInfo {
        serde_json::from_str(auth.get_data()).expect("rich status serde error!")
    }
}


#[derive(Debug)]
pub struct AuthorityManage {
    authorities_log: Wal,
    pub authorities: AuthManageInfo,
    pub authority_n: usize,
    pub authorities_old: AuthManageInfo,
    pub authority_n_old: usize,
    pub authority_h_old: usize,
}

impl AuthorityManage {
    pub fn new() -> Self {
        let logpath = ::std::env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/authorities";

        let mut authority_manage = AuthorityManage {
            authorities_log: Wal::new(&*logpath).unwrap(),
            authorities: AuthManageInfo::new(),
            authority_n: 0,
            authorities_old: AuthManageInfo::new(),
            authority_n_old: 0,
            authority_h_old: 0,
        };

        let vec_out = authority_manage.authorities_log.load();
        if !vec_out.is_empty() {
            //out 转换成authorities;
            if let Ok((h, authorities_old, authorities)) = deserialize(&(vec_out[0].1)) {
                let auth_old: AuthManageInfo = authorities_old;
                let auth: AuthManageInfo = authorities;

                authority_manage.authorities = auth;
                authority_manage.authority_n = authority_manage.authorities.nodes.len();

                authority_manage.authorities_old = auth_old;
                authority_manage.authority_n_old = authority_manage.authorities_old.nodes.len();
                authority_manage.authority_h_old = h;
            }
        }

        authority_manage
    }

    pub fn receive_authorities_list(&mut self, height: usize, authorities: AuthManageInfo) {

        if self.authorities != authorities {
            let mut authorities = authorities;
            std::mem::swap(&mut self.authorities, &mut authorities);
            self.authorities_old = authorities;
            self.authority_n_old = self.authority_n;
            self.authority_h_old = height;
            self.authority_n = self.authorities.nodes.len();

            let bmsg = serialize(&(height, self.authority_n_old.clone(), self.authorities.clone()), Infinite).unwrap();
            let _ = self.authorities_log.save(LOG_TYPE_AUTHORITIES, &bmsg);
        }
    }
}
