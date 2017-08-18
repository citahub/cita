// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Helper type with all filter state data.

use jsonrpc_types::rpctypes::{Log, Filter};
use std::collections::HashSet;
pub type BlockNumber = u64;

/// Filter state.
#[derive(Clone)]
pub enum PollFilter {
    /// Number of last block which client was notified about.
    Block(BlockNumber),
    /// Number of From block number, pending logs and log filter itself.
    Logs(BlockNumber, HashSet<Log>, Filter),
}

/// Returns only last `n` logs
pub fn limit_logs(mut logs: Vec<Log>, limit: Option<usize>) -> Vec<Log> {
    let len = logs.len();
    match limit {
        Some(limit) if len >= limit => logs.split_off(len - limit),
        _ => logs,
    }
}
