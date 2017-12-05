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

use bincode::{deserialize, serialize, Infinite};
use crypto::Signature;
use libproto::blockchain::{Proof, ProofType};
use rustc_serialize::hex::ToHex;
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AuthorityRoundProof {
    pub signature: Signature,
    pub step: u64,
}

impl AuthorityRoundProof {
    pub fn new(step: u64, signature: Signature) -> AuthorityRoundProof {
        AuthorityRoundProof {
            step: step,
            signature: signature,
        }
    }
}

impl From<Proof> for AuthorityRoundProof {
    fn from(p: Proof) -> Self {
        let decoded: AuthorityRoundProof = deserialize(&p.get_content()[..]).unwrap();
        decoded
    }
}

impl Into<Proof> for AuthorityRoundProof {
    fn into(self) -> Proof {
        let mut proof = Proof::new();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        proof.set_content(encoded_proof);
        proof.set_field_type(ProofType::AuthorityRound);
        proof
    }
}

impl fmt::Display for AuthorityRoundProof {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "step: {}, signature: {}",
            self.step,
            self.signature.to_hex()
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_crypto as crypto;

    use super::{AuthorityRoundProof, Signature};
    use crypto::SIGNATURE_NAME;
    use libproto::blockchain::Proof;

    #[test]
    fn proof_display() {
        let proof = AuthorityRoundProof::new(0, Signature::default());
        let string = format!("{}", proof);
        if SIGNATURE_NAME == "ed25519" {
            assert_eq!(
                string,
                "step: 0, \
                 signature: 0000000000000000000\
                 000000000000000000000000000000\
                 000000000000000000000000000000\
                 000000000000000000000000000000\
                 000000000000000000000000000000\
                 000000000000000000000000000000\
                 00000000000000000000000"
            );
        } else if SIGNATURE_NAME == "secp256k1" {
            assert_eq!(
                string,
                "step: 0, \
                 signature: 00000000000000000000000\
                 0000000000000000000000000000000000\
                 0000000000000000000000000000000000\
                 0000000000000000000000000000000000\
                 00000"
            );
        }
    }

    #[test]
    fn proof_convert() {
        let o_proof = AuthorityRoundProof::new(0, Signature::default());
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: AuthorityRoundProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
