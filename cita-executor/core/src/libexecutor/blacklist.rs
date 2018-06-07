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

use libproto::BlackList as ProtoBlackList;
use protobuf::RepeatedField;
use types::Public;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct BlackList {
    black_list: Vec<Public>,
}

impl BlackList {
    pub fn new() -> Self {
        BlackList {
            black_list: Vec::new(),
        }
    }

    pub fn black_list(&self) -> Vec<Public> {
        self.black_list.clone()
    }

    pub fn set_black_list(mut self, black_list: Vec<Vec<u8>>) -> Self {
        self.black_list = black_list
            .iter()
            .map(|public| Public::from_slice(public.as_slice()))
            .collect();
        self
    }

    pub fn len(&self) -> usize {
        self.black_list.len()
    }

    pub fn protobuf(&self) -> ProtoBlackList {
        let mut bl = ProtoBlackList::new();
        bl.set_signer(RepeatedField::from_vec(
            self.black_list
                .clone()
                .into_iter()
                .map(|public| public.to_vec())
                .collect(),
        ));
        bl
    }
}
