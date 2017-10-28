// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use rlp::*;
use util::{Address, U256};

// Account, complement
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Account {
    balance: u64,
    addr: Address,
}



impl Encodable for Account {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.balance);
        s.append(&self.addr);
    }
}

impl Decodable for Account {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        if r.item_count()? != 2 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Account {
            balance: r.val_at(0)?,
            addr: r.val_at(1)?,
        })
    }
}

impl Account {
    // Get the RLP of this commitment.
    pub fn rlp(&self) -> Bytes {
        let mut s = RlpStream::new();
        self.rlp_append(&mut s);
        s.out()
    }

    // Get the crypt_hash (Keccak or blake2b) of this account.
    pub fn rlp_hash(&self) -> H256 {
        self.rlp().crypt_hash()
    }


}

