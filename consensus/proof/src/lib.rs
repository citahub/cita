extern crate libproto;
extern crate util;
extern crate bincode;
extern crate cita_crypto as crypto;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate serde_types;

mod authority_round_proof;
mod tendermint_proof;


use libproto::blockchain::{Proof, ProofType};
pub use authority_round_proof::AuthorityRoundProof;
pub use tendermint_proof::TendermintProof;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CitaProof {
    AuthorityRound(AuthorityRoundProof),
    Raft,
    Tendermint(TendermintProof),
}


impl From<Proof> for CitaProof {
    fn from(p: Proof) -> Self {
        match p.get_field_type() {
            ProofType::AuthorityRound => CitaProof::AuthorityRound(AuthorityRoundProof::from(p)),
            ProofType::Raft => CitaProof::Raft,
            ProofType::Tendermint => CitaProof::Tendermint(TendermintProof::from(p)),
        }

    }
}

impl Into<Proof> for CitaProof {
    fn into(self) -> Proof {
        match self {
            CitaProof::AuthorityRound(proof) => proof.into(),
            CitaProof::Raft => Proof::new(),
            CitaProof::Tendermint(proof) => proof.into(),
        }
    }
}



#[cfg(test)]
mod tests {
    use serde_types::hash::*;
    use super::authority_round_proof::AuthorityRoundProof;
    use super::tendermint_proof::TendermintProof;
    use crypto::Signature;
    use super::CitaProof;
    use libproto::blockchain::Proof;
    use std::collections::HashMap;

    #[test]
    fn poa_proof_convert() {
        let o_proof = CitaProof::AuthorityRound(AuthorityRoundProof::new(0, Signature::default()));
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: CitaProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }

    #[test]
    fn tendermint_proof_convert() {
        let o_proof =
            CitaProof::Tendermint(TendermintProof::new(0, 1, H256::default(), HashMap::new()));
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: CitaProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
