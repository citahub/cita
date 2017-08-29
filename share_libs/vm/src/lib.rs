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

extern crate libproto;
extern crate util;

pub mod precompile;

use libproto::Call as CallTransaction;
use libproto::blockchain::Transaction;
use precompile::Precompile;
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use util::H256;

pub struct VM<T> {
    buildin: HashMap<&'static str, Arc<Mutex<Precompile<T>>>>,
}

impl<T> VM<T>
where
    T: Send + 'static,
{
    pub fn new() -> Self {
        VM { buildin: HashMap::new() }
    }

    pub fn add(&mut self, addr: &'static str, precompile: Precompile<T>) {
        self.buildin.insert(addr, Arc::new(Mutex::new(precompile)));
    }

    pub fn execute(&self, tx: Transaction, height: u64, data: &T) -> H256 {
        let addr = tx.get_to();
        if let Some(p) = self.buildin.get::<str>(addr) {
            return p.lock().unwrap().call(tx.clone(), height, data);
        }
        return H256::zero();
    }

    pub fn query(&self, tx: &CallTransaction, data: &T) -> Vec<u8> {
        let addr = tx.get_to();
        if let Some(p) = self.buildin.get::<str>(str::from_utf8(addr).unwrap()) {
            return p.lock().unwrap().query(tx, data);
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::VM;
    use libproto::Call as CallTransaction;
    use libproto::blockchain::Transaction;
    use precompile::Precompile;
    use std::thread;
    use std::time::Duration;
    use util::H256;

    fn func1(tx: Transaction, height: u64, data: &Vec<u8>) -> H256 {
        thread::sleep(Duration::new(3, 0));
        println!("tx {:?} height {} data {:?}", tx, height, data);
        H256::random()
    }

    fn query1(tx: &CallTransaction, data: &Vec<u8>) -> Vec<u8> {
        thread::sleep(Duration::new(3, 0));
        println!("tx {:?} data {:?}", tx, data);
        H256::random().to_vec()
    }

    fn func2(tx: Transaction, height: u64, data: &Vec<u8>) -> H256 {
        thread::sleep(Duration::new(3, 0));
        println!("tx {:?} height {} data {:?}", tx, height, data);
        H256::random()
    }

    fn query2(tx: &CallTransaction, data: &Vec<u8>) -> Vec<u8> {
        thread::sleep(Duration::new(3, 0));
        println!("tx {:?} data {:?}", tx, data);
        H256::random().to_vec()
    }

    #[test]
    fn basic() {
        let mut vm = VM::<Vec<u8>>::new();

        let data1 = vec![0];
        let p1 = Precompile::<Vec<u8>>::new(Box::new(func1), Box::new(query1));
        vm.add("1", p1);

        let data2 = vec![0];
        let p2 = Precompile::<Vec<u8>>::new(Box::new(func2), Box::new(query2));
        vm.add("2", p2);

        let mut tx1 = Transaction::new();
        tx1.set_to("1".to_string());
        let mut tx2 = Transaction::new();
        tx2.set_to("2".to_string());
        let mut tx3 = Transaction::new();
        tx3.set_to("3".to_string());
        println!("state hash of execute tx1 {:?}", vm.execute(tx1, 1, &data1));
        println!("state hash of execute tx2 {:?}", vm.execute(tx2, 1, &data2));
        println!("state hash of execute tx3 {:?}", vm.execute(tx3, 1, &data1));

        let call_tx1 = CallTransaction::default();
        let call_tx2 = CallTransaction::default();

        println!("query1 {:?}", vm.query(&call_tx1, &data1));
        println!("query2 {:?}", vm.query(&call_tx2, &data2));

        thread::sleep(Duration::new(4, 0));
    }
}
