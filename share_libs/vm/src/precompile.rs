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

use libproto::blockchain::Transaction;
use libproto::Call as CallTransaction;
use util::hash::H256;

pub struct Precompile<T> {
    f: Box<Fn(Transaction, u64, &T) -> H256 + Send>,
    call_f: Box<Fn(&CallTransaction, &T) -> Vec<u8> + Send>,
}

impl<T> Precompile<T>
    where T: Send + 'static
{
    pub fn new(f: Box<Fn(Transaction, u64, &T) -> H256 + Send>,
               call_f: Box<Fn(&CallTransaction, &T) -> Vec<u8> + Send>)
               -> Self {
        Precompile {
            f: f,
            call_f: call_f,
        }
    }

    pub fn call(&mut self, tx: Transaction, height: u64, data: &T) -> H256 {
        (self.f)(tx, height, data)
    }

    pub fn query(&mut self, tx: &CallTransaction, data: &T) -> Vec<u8> {
        (self.call_f)(tx, data)
    }
}
