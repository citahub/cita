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

use super::{Address, KeyPair, PrivKey};
use util::crypto::CreateKey;

#[derive(Default)]
pub struct Signer {
    pub keypair: KeyPair,
    pub address: Address,
}

impl From<PrivKey> for Signer {
    fn from(k: PrivKey) -> Self {
        let keypair = KeyPair::from_privkey(k).unwrap();
        Signer {
            address: keypair.address().clone(),
            keypair: keypair,
        }
    }
}
