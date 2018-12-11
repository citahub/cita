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

extern crate bincode;
extern crate cita_crypto;
extern crate cita_types;
extern crate crossbeam_channel;
extern crate libproto;
extern crate util;

use self::cita_crypto::{CreateKey, PrivKey, Sign, Signature, Signer};
use cita_types::{traits::LowerHex, Address, H256, U256};
use postman::Postman;
use std::str::FromStr;
use util::Hashable;

#[cfg(feature = "secp256k1")]
const PRIVATE_KEY: &str = "ef98e68db428906d626cd37782cdfb052ac282132beee53a99948738ea553b4a";
#[cfg(feature = "ed25519")]
const PRIVATE_KEY: &str = "7c34da71aa179b9906dd5ab6713715aa86c10b8e0486168c336b20cae70d8ec07c34da71aa179b9906dd5ab6713715aa86c10b8e0486168c336b20cae70d8ec0";
#[cfg(feature = "sm2")]
const PRIVATE_KEY: &str = "ef98e68db428906d626cd37782cdfb052ac282132beee53a99948738ea553b4a";
const PROOF_ROUND: usize = 0;
const PROOF_STEP: usize = 0;

pub fn generate_postman(current_height: u64, current_hash: H256) -> Postman {
    let (_mq_req_sender, mq_req_receiver) = crossbeam_channel::unbounded();
    let (mq_resp_sender, _mq_resp_receiver) = crossbeam_channel::unbounded();
    let (fsm_req_sender, _fsm_req_receiver) = crossbeam_channel::unbounded();
    let (_fsm_resp_sender, fsm_resp_receiver) = crossbeam_channel::unbounded();
    let (command_req_sender, _command_req_receiver) = crossbeam_channel::bounded(0);
    let (_command_resp_sender, command_resp_receiver) = crossbeam_channel::bounded(0);
    Postman::new(
        current_height,
        current_hash,
        mq_req_receiver,
        mq_resp_sender,
        fsm_req_sender,
        fsm_resp_receiver,
        command_req_sender,
        command_resp_receiver,
    )
}

pub fn generate_signer() -> Signer {
    let private_key: PrivKey = PrivKey::from_str(PRIVATE_KEY).unwrap();
    let signer: Signer = Signer::from(private_key);
    signer
}

pub fn generate_executed_result(height: u64) -> libproto::ExecutedResult {
    let mut executed_result = libproto::ExecutedResult::new();
    executed_result
        .mut_executed_info()
        .mut_header()
        .set_height(height);
    executed_result
}

pub fn generate_proof(signer: Signer, height: u64, proposal: H256) -> libproto::Proof {
    let serialized = bincode::serialize(
        &(
            height as usize,
            PROOF_ROUND,
            PROOF_STEP,
            signer.address,
            proposal,
        ),
        bincode::Infinite,
    )
    .expect("serialize into bytes");
    let signature = Signature::sign(signer.keypair.privkey(), &serialized.crypt_hash())
        .expect("signature message");

    let mut commits = ::std::collections::HashMap::new();
    commits.insert(signer.address, signature);
    let bft_proof = proof::BftProof::new(height as usize, PROOF_ROUND, proposal, commits);
    bft_proof.into()
}

pub fn generate_signed_transaction(
    to: Address,
    data: Vec<u8>,
    nonce: u32,
    privkey: PrivKey,
) -> libproto::SignedTransaction {
    let mut transaction = libproto::Transaction::new();
    transaction.set_nonce(U256::from(nonce).lower_hex());
    transaction.set_data(data);
    transaction.set_valid_until_block(100);
    transaction.set_quota(1844674);
    if to == Address::from(0) {
        // create or call contract
        transaction.set_to(String::from(""));
    } else {
        // transfer to someone
        transaction.set_to(to.lower_hex());
    }

    let proto_signed_transaction: libproto::SignedTransaction = transaction.sign(privkey);
    proto_signed_transaction
}

// generate OpenBlock without proof
pub fn generate_block(
    height: u64,
    parent_hash: H256,
    to: Address,
    data: Vec<u8>,
    nonce: (u32, u32),
    privkey: PrivKey,
) -> libproto::Block {
    let body = {
        let mut body = libproto::BlockBody::default();
        // fn generate_signed_transaction(to: Address, data: Vec<u8>, nonce: u32, privkey: PrivKey) -> SignedTransaction {
        let mut transactions = Vec::new();
        for i in nonce.0..nonce.1 {
            let transaction = generate_signed_transaction(to, data.clone(), i, privkey);
            transactions.push(transaction);
        }
        body.set_transactions(transactions.into());
        body
    };
    let mut proto_block = libproto::Block::new();
    proto_block.set_body(body);
    let transactions_root = proto_block.get_body().transactions_root().to_vec();
    proto_block
        .mut_header()
        .set_transactions_root(transactions_root);
    proto_block.mut_header().set_prevhash(parent_hash.to_vec());
    proto_block.mut_header().set_height(height);
    proto_block.mut_header().set_timestamp(1543976147000);
    proto_block
}

pub fn generate_contract() -> Vec<u8> {
    //    let source = r#"
    //            pragma solidity ^0.4.8;
    //            contract ConstructSol {
    //                uint a;
    //                event LogCreate(address contractAddr);
    //                event A(uint);
    //                function ConstructSol(){
    //                    LogCreate(this);
    //                }
    //                function set(uint _a) {
    //                    a = _a;
    //                    A(a);
    //                }
    //                function get() returns (uint) {
    //                    return a;
    //                }
    //            }
    //        "#;
    let bin_code = "\
608060405234801561001057600080fd5b507fb8f132fb6526e0405f3ce4f3bab301f1d44\
09b1e7f2c01c2037d6cf845c831cb30604051808273ffffffffffffffffffffffffffffffffffffffff1673fffffff\
fffffffffffffffffffffffffffffffff16815260200191505060405180910390a1610118806100836000396000f30\
06080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900\
463ffffffff16806360fe47b114604e5780636d4ce63c146078575b600080fd5b348015605957600080fd5b5060766\
004803603810190808035906020019092919050505060a0565b005b348015608357600080fd5b50608a60e3565b604\
0518082815260200191505060405180910390f35b806000819055507fa17a9e66f0c355e3aa3b9ea969991204d6b1d\
2e62a47877f612cb2371d79e06a6000546040518082815260200191505060405180910390a150565b6000805490509\
05600a165627a7a7230582099e8d1cb1b7a1d19a5c72911caec1b01c02b80276e2b46d7c5239dea4c42d9f10029";
    bin_code.as_bytes().to_owned()
}

pub fn generate_block_with_proof(height: u64, parent_hash: H256) -> libproto::BlockWithProof {
    // - Block
    let to_address = Address::from(0);
    let data = generate_contract();
    let mut proto_block = generate_block(
        height,
        parent_hash,
        to_address,
        data,
        (0, 10),
        generate_signer().keypair.privkey().clone(),
    );

    // - Proof
    let present_proof = generate_proof(generate_signer(), height, H256::from(0));
    let previous_proof = generate_proof(generate_signer(), height - 1, H256::from(0));
    proto_block.mut_header().set_proof(previous_proof.into());

    // - BlockWithProof
    let mut block_with_proof = libproto::BlockWithProof::new();
    block_with_proof.set_proof(present_proof.into());
    block_with_proof.set_blk(proto_block);
    block_with_proof
}

pub fn generate_signed_proposal(height: u64, parent_hash: H256) -> libproto::SignedProposal {
    // - Block
    let to_address = Address::from(0);
    let data = generate_contract();
    let proto_block = generate_block(
        height,
        parent_hash,
        to_address,
        data,
        (0, 10),
        generate_signer().keypair.privkey().clone(),
    );

    // - Proof
    let signer = generate_signer();
    let signer_address = signer.address.clone();
    let proof = generate_proof(signer, height, H256::from(0));
    let bft_proof = proof::BftProof::from(proof.clone());
    let signature = bft_proof
        .commits
        .get(&signer_address)
        .expect("signer's signature should be contains within commits");

    // - Proposal
    let mut proposal = libproto::Proposal::new();
    proposal.set_block(proto_block);
    proposal.set_round(bft_proof.round as u64);
    proposal.set_height(height);

    // - SignedProposal
    let mut signed_proposal = libproto::SignedProposal::new();
    signed_proposal.set_proposal(proposal);
    signed_proposal.set_signature(signature.to_vec());
    signed_proposal
}
