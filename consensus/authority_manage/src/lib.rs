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

extern crate bincode;
extern crate util;

mod wal;

use bincode::{deserialize, serialize, Infinite};
use util::Address;
use wal::Wal;

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
}

impl AuthorityManage {
    pub fn new() -> Self {
        let logpath = ::std::env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/authorities";

        let mut authority_manage = AuthorityManage {
            authorities: Vec::new(),
            authority_n: 0,
            authorities_log: Wal::new(&*logpath).unwrap(),
            authorities_old: Vec::new(),
            authority_n_old: 0,
            authority_h_old: 0,
        };

        let vec_out = authority_manage.authorities_log.load();
        if !vec_out.is_empty() {
            //out 转换成authorities;
            if let Ok((h, authorities_old, authorities)) = deserialize(&(vec_out[0].1)) {
                let auth_old: Vec<Address> = authorities_old;
                let auth: Vec<Address> = authorities;

                authority_manage.authorities.extend_from_slice(&auth);
                authority_manage.authority_n = authority_manage.authorities.len();

                authority_manage
                    .authorities_old
                    .extend_from_slice(&auth_old);
                authority_manage.authority_n_old = authority_manage.authorities_old.len();
                authority_manage.authority_h_old = h;
            }
        }

        authority_manage
    }

    pub fn receive_authorities_list(&mut self, height: usize, authorities: Vec<Address>) {
        if self.authorities != authorities {
            self.authorities_old.clear();
            self.authorities_old.extend_from_slice(&self.authorities);
            self.authority_n_old = self.authority_n;
            self.authority_h_old = height;

            self.authorities.clear();
            self.authorities.extend_from_slice(&authorities);
            self.authority_n = self.authorities.len();

            let bmsg = serialize(
                &(
                    height,
                    self.authority_n_old.clone(),
                    self.authorities.clone(),
                ),
                Infinite,
            ).unwrap();
            let _ = self.authorities_log.save(LOG_TYPE_AUTHORITIES, &bmsg);
        }
    }
}
