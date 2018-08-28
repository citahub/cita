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

use cita_types::{Address, H256, U256};
use state::backend::Backend;
use state::State;
use util::trie;

pub fn set_storage<B>(
    state: &mut State<B>,
    address: Address,
    key: &[u8],
    value: &[u8],
) -> trie::Result<()>
where
    B: Backend,
{
    let mut v = Vec::new();
    let k = H256::from_slice(key);
    v.extend_from_slice(value);

    let len = v.len();
    if len == 0 {
        return Ok(());
    }
    state.set_storage(&address, k, H256::from(len as u64))?;
    let mut pos = U256::from(k) + U256::one();
    for chunk in v.chunks(32) {
        let chunk_value = H256::from(chunk);
        state.set_storage(&address, H256::from(pos), chunk_value)?;
        pos = pos + U256::one();
    }
    Ok(())
}
