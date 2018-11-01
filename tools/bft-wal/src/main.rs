// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

extern crate authority_manage;
extern crate bincode;
extern crate cita_types;

use authority_manage::wal::Wal;
use authority_manage::{AuthorityManage, DATA_PATH};
use bincode::deserialize;
use cita_types::Address;

#[derive(Debug)]
pub struct OldAuthorityManage {
    pub authorities: Vec<Address>,
    pub authority_n: usize,
    pub authorities_log: Wal,
    pub authorities_old: Vec<Address>,
    pub authority_n_old: usize,
    pub authority_h_old: usize,
}

impl OldAuthorityManage {
    pub fn init() -> Self {
        let logpath =
            ::std::env::var(DATA_PATH).unwrap_or_else(|_| panic!("{} must be set", DATA_PATH));

        let mut authority_manage = Self {
            authorities: Vec::new(),
            authority_n: 0,
            authorities_log: Wal::new(&*logpath).unwrap(),
            authorities_old: Vec::new(),
            authority_n_old: 0,
            authority_h_old: 0,
        };

        let vec_out = authority_manage.authorities_log.load();
        if !vec_out.is_empty() {
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
}

impl Into<AuthorityManage> for OldAuthorityManage {
    fn into(self) -> AuthorityManage {
        let mut validators = self.authorities.clone();
        validators.dedup();

        let mut validators_old = self.authorities_old.clone();
        validators_old.dedup();

        AuthorityManage {
            authorities: self.authorities,
            validators,
            authorities_log: self.authorities_log,
            authorities_old: self.authorities_old,
            validators_old,
            authority_h_old: self.authority_h_old,
        }
    }
}

fn main() {
    let old = OldAuthorityManage::init();
    let mut auth: AuthorityManage = old.into();
    auth.save();
}
