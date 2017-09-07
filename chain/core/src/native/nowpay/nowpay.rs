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

/// NowPay native contract implettion
///
///
use super::*;

pub struct NowPay {
    functions: HashMap<Signature, Box<Function>>,
}

impl Contract for NowPay {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        self.functions.get(hash)
    }
}

impl NowPay {
    pub fn new() -> Self {
        let mut contract = NowPay { functions: HashMap::<Signature, Box<Function>>::new() };
        contract.functions.insert(0, Box::new(NowPay::set_value));
        contract
    }

    pub fn set_value(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..32) {
                let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }
}
