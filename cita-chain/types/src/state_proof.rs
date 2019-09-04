use super::Bytes;
use cita_types::{Address, H256};
use rlp::{self, Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Default, Debug, Clone)]
pub struct StateProof {
    address: Address,
    account_proof: Vec<Bytes>,
    key: H256,
    value_proof: Vec<Bytes>,
}

impl Encodable for StateProof {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4);
        s.append(&self.address);
        s.append_list::<Bytes, Bytes>(&self.account_proof);
        s.append(&self.key);
        s.append_list::<Bytes, Bytes>(&self.value_proof);
    }
}

impl Decodable for StateProof {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(StateProof {
            address: rlp.val_at(0)?,
            account_proof: rlp.list_at(1)?,
            key: rlp.val_at(2)?,
            value_proof: rlp.list_at(3)?,
        })
    }
}

impl StateProof {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        rlp::decode(bytes)
    }

    // FixMe: The implementation should be finished while fixing cross chain
    pub fn verify(&self, _state_root: H256) -> Option<H256> {
        // let state = State();
        // state.verify_proof();

        // let trie = PatriciaTrie::new(db, codec);
        // trie.verify_proof();
        // trie.verify_proof();

        // trie::triedb::verify_value_proof(
        //     &self.address,
        //     state_root,
        //     &self.account_proof,
        //     Account::from_rlp,
        // )
        // .and_then(|a| a.verify_value_proof(&self.key, &self.value_proof))
        None
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn account_proof(&self) -> &Vec<Bytes> {
        &self.account_proof
    }

    pub fn key(&self) -> &H256 {
        &self.key
    }

    #[cfg(test)]
    pub fn set_address(&mut self, new_address: Address) {
        self.address = new_address;
    }
}

#[cfg(test)]
mod test {
    use super::StateProof;
    use rlp;

    #[test]
    fn test_encode_and_decode_state_proof() {
        let state_proof = StateProof::default();

        let proof_rlp = rlp::encode(&state_proof).into_vec();
        let decoded_res: StateProof = rlp::decode(&proof_rlp);
        let encoded_rlp = rlp::encode(&decoded_res).into_vec();
        assert_eq!(proof_rlp, encoded_rlp);
    }
}
