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

extern crate cita_crypto as crypto;
#[macro_use]
extern crate log as rlog;
extern crate protobuf;
extern crate rlp;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate util;

pub mod blockchain;
pub mod communication;
pub mod request;
pub mod into;
pub mod auth;
pub mod response;
pub mod sync;
pub mod consensus;

pub use auth::*;
use blockchain::*;
use communication::*;
pub use consensus::*;
use crypto::{CreateKey, KeyPair, Message as SignMessage, PrivKey, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;
pub use request::*;
pub use response::*;
use rlp::*;
use rustc_serialize::hex::ToHex;
use std::ops::Deref;
use std::result::Result::Err;
pub use sync::{SyncRequest, SyncResponse};
use util::{merklehash, H256, Hashable};
use util::snappy;

//TODO respone contain error
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TxResponse {
    pub hash: H256,
    pub status: String,
}

impl TxResponse {
    pub fn new(hash: H256, status: String) -> Self {
        TxResponse {
            hash: hash,
            status: status,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct State(pub Vec<Vec<u8>>);

pub type TopicMessage = (String, communication::Message);

pub mod submodules {
    pub const JSON_RPC: u32 = 1;
    pub const NET: u32 = 2;
    pub const CHAIN: u32 = 3;
    pub const CONSENSUS: u32 = 4;
    pub const CONSENSUS_CMD: u32 = 5;
    pub const AUTH: u32 = 6;
}

// TODO: 这里要不要修改下，使topics和MsgClass对应起来
pub mod topics {
    pub const DEFAULT: u16 = 0;
    pub const REQUEST: u16 = 1;
    pub const NEW_BLK: u16 = 2;
    pub const NEW_STATUS: u16 = 3;
    pub const SYNC_BLK: u16 = 4;
    pub const RESPONSE: u16 = 5;
    pub const CONSENSUS_MSG: u16 = 6;
    pub const NEW_PROPOSAL: u16 = 7;
    pub const VERIFY_TX_REQ: u16 = 8;
    pub const VERIFY_TX_RESP: u16 = 9;
    pub const VERIFY_BLK_REQ: u16 = 10;
    pub const VERIFY_BLK_RESP: u16 = 11;
    pub const BLOCK_TXHASHES: u16 = 12;
    pub const BLOCK_TXHASHES_REQ: u16 = 13;
    pub const NEW_PROOF_BLOCK: u16 = 14;
    pub const BLOCK_TXS: u16 = 15;
    pub const RICH_STATUS: u16 = 16;
}

#[derive(Debug)]
pub enum MsgClass {
    REQUEST(Request),
    RESPONSE(Response),
    HEADER(BlockHeader),
    BLOCK(Block),
    STATUS(Status),
    VERIFYTXREQ(VerifyTxReq),
    VERIFYTXRESP(VerifyTxResp),
    VERIFYBLKREQ(VerifyBlockReq),
    VERIFYBLKRESP(VerifyBlockResp),
    BLOCKTXHASHES(BlockTxHashes),
    BLOCKTXHASHESREQ(BlockTxHashesReq),
    BLOCKWITHPROOF(BlockWithProof),
    BLOCKTXS(BlockTxs),
    MSG(Vec<u8>),
    RICHSTATUS(RichStatus),
    SYNCREQUEST(SyncRequest),
    SYNCRESPONSE(SyncResponse),
}

pub fn topic_to_string(top: u16) -> &'static str {
    match top {
        topics::DEFAULT => "default",
        topics::REQUEST => "request",
        topics::NEW_BLK => "new_blk",
        topics::NEW_STATUS => "new_status",
        topics::SYNC_BLK => "sync_blk",
        topics::RESPONSE => "response",
        topics::CONSENSUS_MSG => "consensus_msg",
        topics::NEW_PROPOSAL => "new_proposal",
        topics::VERIFY_TX_REQ => "verify_tx_req",
        topics::VERIFY_TX_RESP => "verify_tx_resp",
        topics::VERIFY_BLK_REQ => "verify_blk_req",
        topics::VERIFY_BLK_RESP => "verify_blk_resp",
        topics::BLOCK_TXHASHES => "block_txhashes",
        topics::BLOCK_TXHASHES_REQ => "block_txhashes_req",
        topics::NEW_PROOF_BLOCK => "new_proof_blk",
        topics::BLOCK_TXS => "block_txs",
        topics::RICH_STATUS => "rich_status",
        _ => "",
    }
}

pub fn id_to_key(id: u32) -> &'static str {
    match id {
        submodules::JSON_RPC => "json_rpc",
        submodules::NET => "net",
        submodules::CHAIN => "chain",
        submodules::CONSENSUS => "consensus",
        submodules::CONSENSUS_CMD => "consensus_cmd",
        submodules::AUTH => "auth",
        _ => "",
    }
}

pub fn key_to_id(key: &str) -> u32 {
    if key.starts_with("jsonrpc") {
        submodules::JSON_RPC
    } else if key.starts_with("net") {
        submodules::NET
    } else if key.starts_with("chain") {
        submodules::CHAIN
    } else if key.starts_with("consensus_cmd") {
        submodules::CONSENSUS_CMD
    } else if key.starts_with("consensus") {
        submodules::CONSENSUS
    } else if key.starts_with("auth") {
        submodules::AUTH
    } else {
        0
    }
}

pub fn de_cmd_id(cmd_id: u32) -> (u32, u16) {
    let mut submodule = cmd_id >> 16;
    let sub = submodule;
    submodule = submodule << 16;
    let topic = (cmd_id - submodule) as u16;
    (sub, topic)
}

pub fn display_cmd(cmd_id: u32) -> (&'static str, &'static str) {
    let cd = de_cmd_id(cmd_id);
    (id_to_key(cd.0), topic_to_string(cd.1))
}

pub fn cmd_id(submodule: u32, topic: u16) -> u32 {
    (submodule << 16) + topic as u32
}

pub mod factory {
    use super::*;
    pub const ZERO_ORIGIN: u32 = 99999;

    pub fn create_msg(sub: u32, top: u16, msg_type: MsgType, content: Vec<u8>) -> communication::Message {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(cmd_id(sub, top));
        msg.set_field_type(msg_type);
        msg.set_operate(communication::OperateType::BROADCAST);
        msg.set_origin(ZERO_ORIGIN);
        //compress data
        msg.set_content(snappy::cita_compresse(content));
        msg
    }

    ///for crate_msg extral version
    pub fn create_msg_ex(
        sub: u32,
        top: u16,
        msg_type: MsgType,
        operate: communication::OperateType,
        origin: u32,
        content: Vec<u8>,
    ) -> communication::Message {
        let mut msg = factory::create_msg(sub, top, msg_type, content);
        msg.set_origin(origin);
        msg.set_operate(operate);
        msg
    }
}

type CmdId = u32;
pub type Origin = u32;

pub fn parse_msg(msg: &[u8]) -> (CmdId, Origin, MsgClass) {
    let mut msg = parse_from_bytes::<communication::Message>(msg.as_ref()).unwrap();
    let content_msg = msg.take_content();
    let content_msg = snappy::cita_decompress(content_msg);
    let msg_class = match msg.get_field_type() {
        MsgType::REQUEST => MsgClass::REQUEST(parse_from_bytes::<Request>(&content_msg).unwrap()),
        MsgType::RESPONSE => MsgClass::RESPONSE(parse_from_bytes::<Response>(&content_msg).unwrap()),
        MsgType::HEADER => MsgClass::HEADER(parse_from_bytes::<BlockHeader>(&content_msg).unwrap()),
        MsgType::BLOCK => MsgClass::BLOCK(parse_from_bytes::<Block>(&content_msg).unwrap()),
        MsgType::STATUS => MsgClass::STATUS(parse_from_bytes::<Status>(&content_msg).unwrap()),
        MsgType::VERIFY_TX_REQ => MsgClass::VERIFYTXREQ(parse_from_bytes::<VerifyTxReq>(&content_msg).unwrap()),
        MsgType::VERIFY_TX_RESP => MsgClass::VERIFYTXRESP(parse_from_bytes::<VerifyTxResp>(&content_msg).unwrap()),
        MsgType::VERIFY_BLK_REQ => MsgClass::VERIFYBLKREQ(parse_from_bytes::<VerifyBlockReq>(&content_msg).unwrap()),
        MsgType::VERIFY_BLK_RESP => MsgClass::VERIFYBLKRESP(parse_from_bytes::<VerifyBlockResp>(&content_msg).unwrap()),
        MsgType::BLOCK_TXHASHES => MsgClass::BLOCKTXHASHES(parse_from_bytes::<BlockTxHashes>(&content_msg).unwrap()),
        MsgType::BLOCK_TXHASHES_REQ => {
            MsgClass::BLOCKTXHASHESREQ(parse_from_bytes::<BlockTxHashesReq>(&content_msg).unwrap())
        }
        MsgType::BLOCK_WITH_PROOF => {
            MsgClass::BLOCKWITHPROOF(parse_from_bytes::<BlockWithProof>(&content_msg).unwrap())
        }
        MsgType::BLOCK_TXS => MsgClass::BLOCKTXS(parse_from_bytes::<BlockTxs>(&content_msg).unwrap()),
        MsgType::MSG => {
            let mut content = Vec::new();
            content.extend_from_slice(&content_msg);
            MsgClass::MSG(content)
        }
        MsgType::RICH_STATUS => MsgClass::RICHSTATUS(parse_from_bytes::<RichStatus>(&content_msg).unwrap()),
        MsgType::SYNC_REQ => MsgClass::SYNCREQUEST(parse_from_bytes::<SyncRequest>(&content_msg).unwrap()),
        MsgType::SYNC_RES => MsgClass::SYNCRESPONSE(parse_from_bytes::<SyncResponse>(&content_msg).unwrap()),
    };

    (msg.get_cmd_id(), msg.get_origin(), msg_class)
}

impl blockchain::Transaction {
    /// Signs the transaction by PrivKey.
    pub fn sign(&self, sk: PrivKey) -> SignedTransaction {
        let keypair = KeyPair::from_privkey(sk).unwrap();
        let pubkey = keypair.pubkey();
        let unverified_tx = self.build_unverified(sk);

        // Build SignedTransaction
        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_signer(pubkey.to_vec());
        let bytes = unverified_tx.write_to_bytes().unwrap();
        signed_tx.set_tx_hash(bytes.crypt_hash().to_vec());
        signed_tx.set_transaction_with_sig(unverified_tx);
        signed_tx
    }

    /// Build UnverifiedTransaction
    pub fn build_unverified(&self, sk: PrivKey) -> UnverifiedTransaction {
        let mut unverified_tx = UnverifiedTransaction::new();
        let bytes = self.write_to_bytes().unwrap();
        let hash = bytes.crypt_hash();
        unverified_tx.set_transaction(self.clone());
        let signature = Signature::sign(&sk, &SignMessage::from(hash)).unwrap();
        unverified_tx.set_signature(signature.to_vec());
        unverified_tx.set_crypto(Crypto::SECP);
        unverified_tx
    }
}

impl blockchain::UnverifiedTransaction {
    /// Try to recover the public key.
    pub fn recover_public(&self) -> Result<(PubKey, H256), (H256, String)> {
        let bytes = self.get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash();
        let tx_hash = self.crypt_hash();
        if self.get_signature().len() != SIGNATURE_BYTES_LEN {
            trace!("Invalid signature length {}", hash);
            Err((tx_hash, String::from("Invalid signature length")))
        } else {
            match self.get_crypto() {
                Crypto::SECP => {
                    let signature = Signature::from(self.get_signature());
                    match signature.recover(&hash) {
                        Ok(pubkey) => Ok((pubkey, tx_hash)),
                        _ => {
                            trace!("Recover error {}", tx_hash);
                            Err((tx_hash, String::from("Recover error")))
                        }
                    }
                }
                _ => {
                    trace!("Unexpected crypto {}", tx_hash);
                    Err((tx_hash, String::from("Unexpected crypto")))
                }
            }
        }
    }

    pub fn crypt_hash(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.crypt_hash()
    }

    pub fn tx_verify_req_msg(&self) -> VerifyTxReq {
        let bytes = self.get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash();
        let mut verify_tx_req = VerifyTxReq::new();
        verify_tx_req.set_valid_until_block(self.get_transaction().get_valid_until_block());
        // tx hash
        verify_tx_req.set_hash(hash.to_vec());
        verify_tx_req.set_crypto(self.get_crypto());
        verify_tx_req.set_signature(self.get_signature().to_vec());
        verify_tx_req.set_nonce(self.get_transaction().get_nonce().to_string());
        // unverified tx hash
        let tx_hash = self.crypt_hash();
        verify_tx_req.set_tx_hash(tx_hash.to_vec());
        verify_tx_req
    }
}

impl Deref for SignedTransaction {
    type Target = UnverifiedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.get_transaction_with_sig()
    }
}

impl blockchain::SignedTransaction {
    /// Try to verify transaction and recover sender.
    pub fn verify_transaction(transaction: UnverifiedTransaction) -> Result<Self, H256> {
        let (public, tx_hash) = transaction.recover_public().map_err(|(hash, _)| hash)?;
        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_signer(public.to_vec());
        signed_tx.set_tx_hash(tx_hash.to_vec());
        signed_tx.set_transaction_with_sig(transaction);
        Ok(signed_tx)
    }

    pub fn crypt_hash(&self) -> H256 {
        H256::from(self.tx_hash.as_slice())
    }
}

impl Eq for Proof {}

impl Decodable for Proof {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| Ok(parse_from_bytes::<Proof>(&bytes).unwrap()))
    }
}

impl Encodable for Proof {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.write_to_bytes().unwrap());
    }
}

impl Block {
    pub fn crypt_hash(&self) -> H256 {
        self.get_header().crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        self.get_header().crypt_hash_hex()
    }

    pub fn check_hash(&self) -> bool {
        self.get_body().transactions_root().0 == *self.get_header().get_transactions_root()
    }

    pub fn block_verify_req(&self, request_id: u64) -> VerifyBlockReq {
        let mut reqs: Vec<VerifyTxReq> = Vec::new();
        let signed_txs = self.get_body().get_transactions();
        for signed_tx in signed_txs {
            let signer = signed_tx.get_signer();
            let unverified_tx = signed_tx.get_transaction_with_sig();
            let mut verify_tx_req = unverified_tx.tx_verify_req_msg();
            verify_tx_req.set_signer(signer.to_vec());
            reqs.push(verify_tx_req);
        }
        let mut verify_blk_req = VerifyBlockReq::new();
        verify_blk_req.set_id(request_id);
        verify_blk_req.set_reqs(RepeatedField::from_vec(reqs));
        verify_blk_req
    }
}

impl BlockHeader {
    pub fn crypt_hash(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        let bytes = self.write_to_bytes().unwrap();
        bytes.crypt_hash().to_hex()
    }
}

impl BlockBody {
    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.get_transactions()
            .iter()
            .map(|ts| H256::from_slice(ts.get_tx_hash()))
            .collect()
    }

    pub fn transactions_root(&self) -> H256 {
        merklehash::complete_merkle_root_raw(self.transaction_hashes().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_id_works() {
        assert_eq!(cmd_id(submodules::JSON_RPC, topics::REQUEST), 0x10001);
        assert_eq!(cmd_id(submodules::CHAIN, topics::RESPONSE), 0x30005);
    }

    #[test]
    fn create_tx() {
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to("123".to_string());
        tx.set_valid_until_block(99999);
        tx.set_quota(999999999);

        let signed_tx = tx.sign(*pv);
        assert_eq!(
            signed_tx.crypt_hash(),
            signed_tx.get_transaction_with_sig().crypt_hash()
        );
    }
}
