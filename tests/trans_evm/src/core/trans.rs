use crypto::*;
use libproto::blockchain::{Content, Crypto, SignedTransaction, Transaction};
use protobuf::core::Message;
use util::*;
use uuid::Uuid;
use serde_types::hash::H256;

#[allow(dead_code,unused_variables)]
#[derive(Clone,Debug)]
pub enum Methods{
    Sendtx(Transaction),
    Height,
    Blockbyheiht(u64),
    Trans(String),
    Receipt(String),
}


#[allow(dead_code,unused_variables)]
#[derive(Debug,Clone)]
pub struct Trans{
    tx: Transaction,
}

#[allow(dead_code,unused_variables)]
impl Trans{

    pub fn new() -> Self{
        Trans{
            tx: Transaction::new(),
        }
    }

    pub fn generate_tx(code: &str, address: String,pv: &PrivKey, pk: &PubKey) ->  Transaction {

        //let code = "601080600c6000396000f3006000355415600957005b60203560003555".from_hex().unwrap();
        let data = code.from_hex().unwrap();
        let random_string = Uuid::new_v4().to_string();

        let mut content = Content::new();
        content.set_nonce("0".to_string());
        content.set_data(data);
        content.set_gas(5000);

        let mut tx = Transaction::new();
        tx.set_content(content.clone().write_to_bytes().unwrap());
        tx.set_valid_until_block(99999);
        let sender = pubkey_to_address(pk);
        tx.set_from(format!("{:?}",sender.0));
        //设置空，则创建合约
        tx.set_to(address);

        let message = tx.sha3();
        let signature = sign(pv, &H256::from(message)).unwrap();

        let mut signed_content = SignedTransaction::new();
        signed_content.set_transaction(tx.write_to_bytes().unwrap());
        signed_content.set_signature(signature.to_vec());
        signed_content.set_crypto(Crypto::SECP);

        tx.clear_content();
        tx.set_content(signed_content.write_to_bytes().unwrap());

        tx
    }

    pub fn generate_tx_data(method: Methods) -> String{

        let txdata = match method{
            Methods::Sendtx(tx) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}",tx.write_to_bytes().unwrap().to_hex())
            },
            Methods::Height => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_blockNumber\",\"params\":[],\"id\":2}}")
            },
            Methods::Blockbyheiht(h) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getBlockByNumber\",\"params\":[{},false],\"id\":2}}", h)
            },
            Methods::Trans(hash) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getTransaction\",\"params\":[\"{}\"],\"id\":2}}",hash)
            },
            Methods::Receipt(hash) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionReceipt\",\"params\":[\"{}\"],\"id\":2}}",hash)
            },
        };
        txdata
        //Self::new(txdata)
    }

}
