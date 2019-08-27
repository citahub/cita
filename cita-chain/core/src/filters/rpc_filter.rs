use crate::libchain::chain::Chain;
use crate::types::block_number::{BlockNumber, BlockTag, Tag};
use crate::types::filter::Filter as FilterType;
use cita_types::H256;
use jsonrpc_types::rpc_types::{Filter, FilterChanges, Log};

/// The RPC interfaces about filter.
///     * newFilter
///     * newBlockFilter
///     * getFilterChanges
///     * getFilterLogs
///     * uninstallFilter
/// *Not include `getLogs`*.
pub trait RpcFilter {
    // Create a new filter and return the filter id
    // https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#newfilter
    fn new_filter(&self, filter: Filter) -> usize;
    // Create a new filter that can listen the new block.
    // https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#newblockfilter
    fn new_block_filter(&self) -> usize;
    // Get the logs for the filter with the given id since last time it was called.
    // https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#getfilterchanges
    fn get_filter_changes(&self, id: usize) -> Option<FilterChanges>;
    // Get the logs for the filter with the given id.
    // https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#getfilterlogs
    fn get_filter_logs(&self, id: usize) -> Option<Vec<Log>>;
    // Remove the filter with the given id.
    // https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#uninstallfilter
    fn uninstall_filter(&self, id: usize) -> bool;
}

/// Helper for RpcFilter
trait FilterHelper {
    // Get the block filter with the given id
    fn get_block_filter(&self, id: usize) -> BlockNumber;
    // Get logs with given filter
    fn get_logs_with_filter(&self, filter: Filter, block_filter: BlockNumber) -> Vec<Log>;
}

impl FilterHelper for Chain {
    fn get_block_filter(&self, id: usize) -> BlockNumber {
        let filterdb = self.filter_db();
        let mut block_filter = BlockNumber::min_value();

        if let Some(block_number) = filterdb.try_lock().unwrap().get_block_filter(id) {
            block_filter = *block_number;
        }

        drop(filterdb);
        block_filter
    }

    fn get_logs_with_filter(&self, filter: Filter, block_filter: BlockNumber) -> Vec<Log> {
        let mut filter: FilterType = filter.into();
        filter.from_block = BlockTag::Height(block_filter);
        filter.to_block = BlockTag::Tag(Tag::Latest);
        let limit = filter.limit;
        split_logs(
            self.get_logs(&filter).into_iter().map(Into::into).collect(),
            limit,
        )
    }
}

impl RpcFilter for Chain {
    fn new_filter(&self, filter: Filter) -> usize {
        let filterdb = self.filter_db();
        let id = filterdb.try_lock().unwrap().gen_id();
        let block_number = self.get_current_height();
        filterdb.try_lock().unwrap().gen_logs_filter(id, filter);
        filterdb
            .try_lock()
            .unwrap()
            .gen_block_filter(id, block_number);
        drop(filterdb);
        id
    }

    fn new_block_filter(&self) -> usize {
        let filterdb = self.filter_db();
        let block_number = self.get_current_height();
        let id = filterdb.try_lock().unwrap().gen_id();
        filterdb
            .try_lock()
            .unwrap()
            .gen_block_filter(id, block_number);
        drop(filterdb);
        id
    }

    fn get_filter_changes(&self, id: usize) -> Option<FilterChanges> {
        let filterdb = self.filter_db();
        let mut changes = Some(FilterChanges::Empty);
        let current_number = self.get_current_height();
        let block_filter = self.get_block_filter(id);

        if !filterdb.try_lock().unwrap().is_filter(id) {
            drop(filterdb);
            return changes;
        }

        // Check the logs
        if let Some(filter) = filterdb.try_lock().unwrap().get_logs_filter(id) {
            trace!("Into filter changes: logs");
            changes = Some(FilterChanges::Logs(
                self.get_logs_with_filter(filter.clone(), block_filter),
            ));
        };

        // Check the block
        if filterdb.try_lock().unwrap().is_block_filter(id) {
            trace!("Into filter changes: block");
            // Return hashes from next block after the filter was created to the current block.
            let hashes = ((block_filter + 1)..=current_number)
                .filter_map(|_id| self.block_hash_by_height(_id))
                .collect::<Vec<H256>>();
            trace!("Block filter changes: {:?}", hashes);
            changes = Some(FilterChanges::Hashes(
                hashes.into_iter().map(Into::into).collect(),
            ));
        };

        // Update the block filter: use the current number
        filterdb
            .try_lock()
            .unwrap()
            .gen_block_filter(id, current_number + 1);
        drop(filterdb);
        changes
    }

    fn get_filter_logs(&self, id: usize) -> Option<Vec<Log>> {
        let filterdb = self.filter_db();
        let block_filter = self.get_block_filter(id);
        let logs = match filterdb.try_lock().unwrap().get_logs_filter(id) {
            Some(filter) => Some(self.get_logs_with_filter(filter.clone(), block_filter)),
            _ => None,
        };
        drop(filterdb);
        logs
    }

    fn uninstall_filter(&self, id: usize) -> bool {
        let filterdb = self.filter_db();
        let uninstall_ok = filterdb.try_lock().unwrap().uninstall(id);
        drop(filterdb);
        uninstall_ok
    }
}

/// Split the logs with the limit filter.
fn split_logs(mut logs: Vec<Log>, limit: Option<usize>) -> Vec<Log> {
    let len = logs.len();
    match limit {
        Some(limit) if len >= limit => logs.split_off(len - limit),
        _ => logs,
    }
}
