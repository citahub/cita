use jsonrpc_types::rpc_types::Filter;
use std::collections::HashMap;
use std::time::SystemTime;

// TODO Refactor:
// * use generic data type
// * use one hashmap: use tuple type
//
// ```rust
// struct FilterData<T> {
//     data: HashMap<usize, T>,
// }
//      ```
#[derive(Default)]
pub struct BlockFilter {
    /// To save the filter for filter_blocks
    data: HashMap<usize, u64>,
}

impl BlockFilter {
    fn is_filter(&self, id: usize) -> bool {
        self.data.contains_key(&id)
    }

    fn insert(&mut self, id: usize, filter: u64) {
        self.data.insert(id, filter);
    }

    fn remove(&mut self, id: usize) {
        self.data.remove(&id);
    }

    fn get(&self, id: usize) -> Option<&u64> {
        self.data.get(&id)
    }
}

#[derive(Default)]
pub struct LogsFilter {
    /// To save the filter for filter_logs
    data: HashMap<usize, Filter>,
}

impl LogsFilter {
    fn is_filter(&self, id: usize) -> bool {
        self.data.contains_key(&id)
    }

    fn insert(&mut self, id: usize, filter: Filter) {
        self.data.insert(id, filter);
    }

    fn remove(&mut self, id: usize) {
        self.data.remove(&id);
    }

    fn get(&self, id: usize) -> Option<&Filter> {
        self.data.get(&id)
    }
}

#[derive(Default)]
pub struct FilterDB {
    /// Self-increase ID.
    next_available_id: usize,
    /// To save the last update timestamp
    last_update: HashMap<usize, u64>,
    /// Refactor: Note: logs filter includes block filter.
    block_filter: BlockFilter,
    logs_filter: LogsFilter,
    /// lifetime of fileter id
    lifetime: u32,
}

impl FilterDB {
    pub fn new() -> Self {
        FilterDB {
            // Set the lifetime: 60s
            lifetime: 60,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn set_lifetime(&mut self, lifetime: u32) {
        self.lifetime = lifetime;
    }

    /// Generate a new fresh id
    /// Prune the hashmap first.
    pub fn gen_id(&mut self) -> usize {
        self.prune();
        let id = self.next_available_id;
        self.next_available_id = self.next_available_id.wrapping_add(1);
        id
    }

    /// Generate a new normal filter
    pub fn gen_logs_filter(&mut self, id: usize, filter: Filter) {
        let now = now();
        self.last_update.insert(id, now);
        self.logs_filter.insert(id, filter);
    }

    /// Generate a new filter for block
    pub fn gen_block_filter(&mut self, id: usize, filter: u64) {
        let now = now();
        self.last_update.insert(id, now);
        self.block_filter.insert(id, filter);
    }

    /// Uninstall the filter id
    pub fn uninstall(&mut self, id: usize) -> bool {
        // Remove logs first. casue logs filter includes the block filter.
        if self.is_logs_filter(id) {
            self.logs_filter.remove(id);
            true
        } else if self.is_block_filter(id) {
            self.block_filter.remove(id);
            true
        } else {
            false
        }
    }

    /// Prune the overdue id manually
    /// Remove all the ids that: (now-lastupdate) > self.lifetime
    pub fn prune(&mut self) {
        let now = now();
        for (id, time) in self.last_update.clone().iter() {
            if (now - *time) >= self.lifetime.into() {
                trace!("Prune filter, time: {:?}", (now - *time));
                self.block_filter.remove(*id);
                self.logs_filter.remove(*id);
            }
        }
    }

    /// Get the filter for logs.
    /// Prune the hashmap first.
    pub fn get_logs_filter(&mut self, id: usize) -> Option<&Filter> {
        let now = now();
        trace!("Get logs filter: {:?}", self.logs_filter.get(id));
        self.prune();
        self.last_update.insert(id, now);
        self.logs_filter.get(id)
    }

    /// Get the filter for block
    /// Prune the hashmap first.
    pub fn get_block_filter(&mut self, id: usize) -> Option<&u64> {
        let now = now();
        trace!("Get block filter: {:?}", self.block_filter.get(id));
        self.prune();
        self.last_update.insert(id, now);
        self.block_filter.get(id)
    }

    /// Check the id is for block filter:
    /// Check the log, too. cause the logs filter also insert block filter.
    pub fn is_block_filter(&self, id: usize) -> bool {
        self.block_filter.is_filter(id) && !self.logs_filter.is_filter(id)
    }

    /// Check the id is for logs
    pub fn is_logs_filter(&self, id: usize) -> bool {
        self.logs_filter.is_filter(id)
    }

    /// Check the id is block filter or logs filter
    pub fn is_filter(&self, id: usize) -> bool {
        self.logs_filter.is_filter(id) || self.block_filter.is_filter(id)
    }
}

/// Generate the now time
fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::FilterDB;
    use jsonrpc_types::rpc_types::BlockNumber;
    use jsonrpc_types::rpc_types::Filter;

    #[test]
    fn test_gen_id() {
        let mut filterdb = FilterDB::new();
        let id = filterdb.next_available_id;
        filterdb.gen_id();
        assert_eq!(filterdb.next_available_id, id + 1);
    }

    #[test]
    fn test_gen_logs_filter_and_uninstall() {
        let mut filterdb = FilterDB::new();
        let id = filterdb.gen_id();
        let filter = Filter::new(BlockNumber::earliest(), BlockNumber::earliest(), None, None);
        assert_eq!(filterdb.is_logs_filter(id), false);
        assert_eq!(filterdb.get_logs_filter(id), None);
        // Gen logs filter
        filterdb.gen_logs_filter(id, filter.clone());
        assert_eq!(filterdb.is_logs_filter(id), true);
        assert_eq!(*filterdb.get_logs_filter(id).unwrap(), filter);
        // Uninstall
        filterdb.uninstall(id);
        assert_eq!(filterdb.is_block_filter(id), false);
        assert_eq!(filterdb.get_block_filter(id), None);
    }

    #[test]
    fn test_gen_block_filter_and_uninstall() {
        let mut filterdb = FilterDB::new();
        let id = filterdb.gen_id();
        let filter = 0;
        assert_eq!(filterdb.is_block_filter(id), false);
        assert_eq!(filterdb.get_block_filter(id), None);
        // Gen block filter
        filterdb.gen_block_filter(id, filter.clone());
        assert_eq!(filterdb.is_block_filter(id), true);
        assert_eq!(*filterdb.get_block_filter(id).unwrap(), filter);
        // Uninstall
        filterdb.uninstall(id);
        assert_eq!(filterdb.is_block_filter(id), false);
        assert_eq!(filterdb.get_block_filter(id), None);
    }

    #[test]
    fn test_prune() {
        let mut filterdb = FilterDB::new();
        let id = filterdb.gen_id();
        let filter = 0;
        filterdb.gen_block_filter(id, filter.clone());
        assert_eq!(filterdb.is_filter(id), true);
        assert_eq!(*filterdb.get_block_filter(id).unwrap(), filter);
        filterdb.set_lifetime(0);
        assert_eq!(filterdb.lifetime, 0);
        filterdb.prune();
        assert_eq!(filterdb.get_block_filter(id), None);
        assert_eq!(filterdb.is_filter(id), false);
    }
}
