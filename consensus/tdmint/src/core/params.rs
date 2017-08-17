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

use crypto::Signer;
use engine_json;
use std::time::Duration;
use util::Address;

#[derive(Debug, Clone)]
pub struct TendermintTimer {
    pub propose: Duration,
    pub prevote: Duration,
    pub precommit: Duration,
    pub commit: Duration,
}

impl Default for TendermintTimer {
    fn default() -> Self {
        TendermintTimer {
            propose: Duration::from_millis(1400),
            prevote: Duration::from_millis(400),
            precommit: Duration::from_millis(400),
            commit: Duration::from_millis(600),
        }
    }
}


pub struct TendermintParams {
    pub timer: TendermintTimer,
    pub duration: Duration,
    pub is_test: bool,
    /// Valid authorities
    pub authorities: Vec<Address>,
    pub authority_n: usize,
    pub signer: Signer,
    pub block_tx_limit: usize,
    pub tx_filter_size: usize,
    pub tx_pool_size: usize,
}

fn to_duration(s: u64) -> Duration {
    Duration::from_millis(s)
}

impl From<engine_json::TendermintParams> for TendermintParams {
    fn from(p: engine_json::TendermintParams) -> Self {
        let dt = TendermintTimer::default();
        TendermintParams {
            duration: Duration::from_millis(p.duration.into()),
            is_test: p.is_test,
            authority_n: p.authorities.len(),
            authorities: p.authorities.into_iter().map(Into::into).collect::<Vec<_>>(),
            signer: Signer::from(p.signer),
            block_tx_limit: p.block_tx_limit as usize,
            tx_filter_size: p.tx_filter_size as usize,
            tx_pool_size: p.tx_pool_size as usize,
            timer: TendermintTimer {
                propose: p.timeout_propose.map_or(dt.propose, to_duration),
                prevote: p.timeout_prevote.map_or(dt.prevote, to_duration),
                precommit: p.timeout_precommit.map_or(dt.precommit, to_duration),
                commit: p.timeout_commit.map_or(dt.commit, to_duration),
            },
        }
    }
}
