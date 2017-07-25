use util::trie::TrieFactory;
use account_db::Factory as AccountFactory;
use evm::Factory as EvmFactory;

/// Collection of factories.
#[derive(Default, Clone)]
pub struct Factories {
	/// factory for evm.
	pub vm: EvmFactory,
    /// factory for tries.
    pub trie: TrieFactory,
    /// factory for account databases.
    pub accountdb: AccountFactory,
}
