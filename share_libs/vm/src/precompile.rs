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
