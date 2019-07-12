use cita_trie::DB;
use cita_vm::{state::State, BlockDataProvider, Config};
use std::cell::RefCell;
use std::sync::Arc;

// FIXME: CITAExecutive need rename to Executive after all works ready.
pub struct CitaExecutive<B> {
    pub block_provider: Arc<BlockDataProvider>,
    pub state_provider: Arc<RefCell<State<B>>>,
    pub config: Config,
}

impl<B: DB + 'static> CitaExecutive<B> {
    pub fn new(
        block_provider: Arc<BlockDataProvider>,
        state_provider: State<B>,
        config: Config,
    ) -> Self {
        Self {
            block_provider,
            state_provider: Arc::new(RefCell::new(state_provider)),
            config,
        }
    }

    pub fn exec() {}
}
