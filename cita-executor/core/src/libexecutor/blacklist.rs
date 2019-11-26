// Copyright Cryptape Technologies LLC.
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

use cita_types::Address;
use libproto::BlackList as ProtoBlackList;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct BlackList {
    black_list: Vec<Address>,
    clear_list: Vec<Address>,
}

impl BlackList {
    pub fn new() -> Self {
        BlackList {
            black_list: Vec::new(),
            clear_list: Vec::new(),
        }
    }

    pub fn black_list(&self) -> &Vec<Address> {
        &self.black_list
    }

    pub fn clear_list(&self) -> &Vec<Address> {
        &self.clear_list
    }

    pub fn set_black_list(mut self, black_list: Vec<Address>) -> Self {
        self.black_list = black_list;
        self
    }

    pub fn set_clear_list(mut self, clear_list: Vec<Address>) -> Self {
        self.clear_list = clear_list;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.black_list.len() + self.clear_list.len()
    }

    pub fn protobuf(&self) -> ProtoBlackList {
        let mut bl = ProtoBlackList::new();
        bl.set_black_list(
            self.black_list
                .clone()
                .into_iter()
                .map(|address| address.to_vec())
                .collect(),
        );
        bl.set_clear_list(
            self.clear_list
                .clone()
                .into_iter()
                .map(|address| address.to_vec())
                .collect(),
        );
        bl
    }
}
