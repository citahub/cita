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

pub mod handler;
pub mod authority_round;
pub mod spec;

pub use self::authority_round::AuthorityRound;
pub use self::spec::Spec;
use crypto::{PrivKey, Signature, sign, recover, pubkey_to_address, Error as CryptoError};
pub use engine::*;
pub use libproto::blockchain::{BlockHeader, Block, Transaction, BlockBody, Proof};
use util::Address;
use util::H256;

pub trait Signable {
    fn bare_hash(&self) -> H256;
    fn sign_with_privkey(&self, privkey: &PrivKey) -> Result<Signature, CryptoError> {
        sign(privkey, &self.bare_hash().into())
    }
    fn recover_address_with_signature(&self, signature: &Signature) -> Result<Address, CryptoError> {
        let pubkey = recover(signature, &self.bare_hash().into()).unwrap();
        Ok(pubkey_to_address(&pubkey).into())
    }
}
