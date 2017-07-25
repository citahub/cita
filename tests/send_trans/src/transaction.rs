use crypto::{KeyPair, sign};
use libproto::blockchain::{Content, Crypto, SignedTransaction, Transaction};
use now_proto::{AccountInvoke, AccountInvokeMethod, CreateAccountParams, Crypto as TxCrypto, Role};
use protobuf::core::Message;
use rustc_serialize::hex::ToHex;
use std::str::FromStr;
use serde_types::hash::H256;
use util::hash::H256 as Hash256;
use uuid::Uuid;

pub fn make_tx_msg(category: &str, key: &str) -> String {
    let tx = match category {
        "create-account" | _ => get_tx_for_create_account(key),
    };
    format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}",
            tx.write_to_bytes().unwrap().to_hex())
}

fn get_tx_for_create_account(key: &str) -> Transaction {
    let random_string = Uuid::new_v4().to_string();

    let mut params = CreateAccountParams::new();
    params.set_crypto(TxCrypto::SECP);
    params.set_identifier(random_string.clone());
    params.set_pubkey(random_string.clone());
    params.set_info(random_string.clone());
    params.set_role(Role::ADMIN);

    let mut invoke = AccountInvoke::new();
    invoke.set_method(AccountInvokeMethod::CREATE_ACCOUNT);
    invoke.set_params(params.write_to_bytes().unwrap());

    let mut content = Content::new();
    content.set_nonce(random_string.clone());
    content.set_data(invoke.write_to_bytes().unwrap());

    let mut tx = Transaction::new();
    tx.set_to(String::from("2"));
    tx.set_valid_until_block(4294967296u64); // 2^32
    tx.set_content(content.write_to_bytes().unwrap());

    let privkey = H256::from(Hash256::from_str(key).unwrap());
    let keypair = KeyPair::from_privkey(privkey).unwrap();
    let message = tx.sha3();
    let signature = sign(keypair.privkey(), &message.into()).unwrap();

    let mut stx = SignedTransaction::new();
    stx.set_transaction(tx.write_to_bytes().unwrap());
    stx.set_crypto(Crypto::SECP);
    stx.set_signature(signature.0.to_vec());

    tx.clear_content();
    tx.set_content(stx.write_to_bytes().unwrap());

    tx
}
