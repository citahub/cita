// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use super::{limit_logs, PollFilter, PollId};
use cita_types::H256;
use jsonrpc_types::rpctypes::{Filter, FilterChanges, Log};
use libchain::chain::Chain;
use types::filter::Filter as EthcoreFilter;
use types::ids::BlockId;

pub trait EthFilter {
    fn new_filter(&self, filter: Filter) -> PollId;
    fn new_block_filter(&self) -> PollId;
    fn filter_changes(&self, index: usize) -> Option<FilterChanges>;
    fn filter_logs(&self, index: usize) -> Option<Vec<Log>>;
    fn uninstall_filter(&self, index: usize) -> bool;
}

impl EthFilter for Chain {
    fn new_filter(&self, filter: Filter) -> PollId {
        let polls = self.poll_filter();
        let block_number = self.get_current_height();
        let id =
            polls
                .lock()
                .create_poll(PollFilter::Logs(block_number, Default::default(), filter));
        drop(polls);
        id
    }

    fn new_block_filter(&self) -> PollId {
        let polls = self.poll_filter();
        let id = polls
            .lock()
            .create_poll(PollFilter::Block(self.get_current_height()));
        drop(polls);
        id
    }

    fn filter_changes(&self, index: usize) -> Option<FilterChanges> {
        let polls = self.poll_filter();
        let log = match polls.lock().poll_mut(index) {
            None => Some(FilterChanges::Empty),
            Some(filter) => match *filter {
                PollFilter::Block(ref mut block_number) => {
                    // + 1, cause we want to return hashes including current block hash.
                    let current_number = self.get_current_height() + 1;
                    let hashes = (*block_number..current_number)
                        .filter_map(|_id| self.block_hash_by_height(_id))
                        .collect::<Vec<H256>>();

                    *block_number = current_number;
                    Some(FilterChanges::Hashes(
                        hashes.into_iter().map(Into::into).collect(),
                    ))
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
                    Some(FilterChanges::Logs(limit_logs(
                        self.get_logs(&filter).into_iter().map(Into::into).collect(),
                        limit,
                    )))
                }
            },
        };
        drop(polls);
        log
    }

    fn filter_logs(&self, index: usize) -> Option<Vec<Log>> {
        let polls = self.poll_filter();
        let log = match polls.lock().poll(index) {
            Some(&PollFilter::Logs(ref _block_number, ref _previous_log, ref filter)) => {
                let filter: EthcoreFilter = filter.clone().into();
                Some(self.get_logs(&filter).into_iter().map(Into::into).collect())
            }
            // just empty array
            _ => None,
        };
        drop(polls);
        log
    }

    fn uninstall_filter(&self, index: usize) -> bool {
        let polls = self.poll_filter();
        let mut polls = polls.lock();
        let is_uninstall = match polls.poll(index) {
            Some(_) => {
                polls.remove_poll(index);
                true
            }
            None => false,
        };
        drop(polls);
        is_uninstall
    }
}
