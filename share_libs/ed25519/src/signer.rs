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
    fn from(privkey: PrivKey) -> Self {
        let keypair = KeyPair::from_privkey(privkey).unwrap();
        Signer {
            address: keypair.address(),
            keypair: keypair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::crypto::CreateKey;

    #[test]
    fn test_signer() {
        let keypair = KeyPair::gen_keypair();
        let signer = Signer::from(keypair.privkey().clone());
        assert_eq!(signer.keypair.privkey(), keypair.privkey());
        assert_eq!(signer.keypair.pubkey(), keypair.pubkey());
        assert_eq!(signer.address, keypair.address());
    }
}
