#![allow(unused_variables, unused_imports)]
use super::{PollManager, PollFilter, PollId, limit_logs};
use bigint::{H256, U256};
use jsonrpc_types::rpctypes::{Filter, Log, BlockNumber, Index, FilterChanges};
use libchain::chain::Chain;
use std::collections::HashSet;
use std::sync::Arc;

use types::filter::Filter as EthcoreFilter;
use types::ids::BlockId;
use util::Mutex;


pub trait EthFilter {
    fn new_filter(&self, filter: Filter) -> PollId;
    fn new_block_filter(&self) -> PollId;
    fn new_pending_transaction_filter(&self) -> PollId;
    fn filter_changes(&self, index: Index) -> Option<FilterChanges>;
    fn filter_logs(&self, index: Index) -> Option<Vec<Log>>;
    fn uninstall_filter(&self, index: Index) -> bool;
}


impl EthFilter for Chain {
    fn new_filter(&self, filter: Filter) -> PollId {
        let polls = self.poll_filter();
        let block_number = self.get_current_height();
        let id = polls.lock().create_poll(PollFilter::Logs(block_number, Default::default(), filter));
        drop(polls);
        id
    }

    fn new_block_filter(&self) -> PollId {
        let polls = self.poll_filter();
        let id = polls.lock().create_poll(PollFilter::Block(self.get_current_height()));
        drop(polls);
        id
    }

    fn new_pending_transaction_filter(&self) -> PollId {
        let polls = self.poll_filter();
        let best_block = self.get_current_height();
        let pending_transactions = vec![];
        let id = polls.lock().create_poll(PollFilter::PendingTransaction(pending_transactions));
        drop(polls);
        id
    }

    fn filter_changes(&self, index: Index) -> Option<FilterChanges> {
        let polls = self.poll_filter();
        let log = match polls.lock().poll_mut(&index.value()) {
            None => Some(FilterChanges::Empty),
            Some(filter) => match *filter {
                PollFilter::Block(ref mut block_number) => {
                    // + 1, cause we want to return hashes including current block hash.
                    let current_number = self.get_current_height() + 1;
                    let hashes = (*block_number..current_number)
                        .into_iter()
                        .filter_map(|_id| self.block_hash(_id))
                        .collect::<Vec<H256>>();

                    *block_number = current_number;
                    Some(FilterChanges::Hashes(hashes))

                }
                PollFilter::PendingTransaction(ref mut previous_hashes) => {
                    Some(FilterChanges::Hashes(vec![]))
                }
                PollFilter::Logs(ref mut block_number, ref mut _previous_logs, ref filter) => {
                    // retrive the current block number
                    let current_number = self.get_current_height();
                    // build appropriate filter
                    let mut filter: EthcoreFilter = filter.clone().into();
                    filter.from_block = BlockId::Number(*block_number);
                    filter.to_block = BlockId::Latest;
                    // save the number of the next block as a first block from which
                    // we want to get logs
                    *block_number = current_number + 1;
                    // retrieve logs in range from_block..min(BlockId::Latest..to_block)
                    let limit = filter.limit;
                    Some(FilterChanges::Logs(limit_logs(self.get_logs(filter).into_iter().map(Into::into).collect(), limit)))
                }
            },
        };
        drop(polls);
        log
    }

    fn filter_logs(&self, index: Index) -> Option<Vec<Log>> {
        let polls = self.poll_filter();
        let log = match polls.lock().poll(&index.value()) {
            Some(&PollFilter::Logs(ref _block_number, ref _previous_log, ref filter)) => {
                let filter: EthcoreFilter = filter.clone().into();
                Some(self.get_logs(filter).into_iter().map(Into::into).collect())
            }
            // just empty array
            _ => None,
        };
        drop(polls);
        log
    }

    fn uninstall_filter(&self, index: Index) -> bool {
        let polls = self.poll_filter();
        let mut polls = polls.lock();
        let is_uninstall = match polls.poll(&index.value()) {
            Some(_) => {
                polls.remove_poll(&index.value());
                true
            }
            None => false,
        };
        drop(polls);
        is_uninstall
    }
}
