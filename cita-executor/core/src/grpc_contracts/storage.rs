use cita_types::{Address, H160, H256, U256, U512};
use state::backend::Backend;
use state::State;
use util::trie;

pub fn set_storage<B>(
    state: &mut State<B>,
    address: Address,
    key: Vec<u8>,
    value: Vec<u8>,
) -> trie::Result<()>
where
    B: Backend,
{
    let mut v = Vec::new();
    let k = H256::from_slice(&key);
    v.extend_from_slice(&value);

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
