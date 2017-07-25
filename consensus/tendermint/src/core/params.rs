use std::time::Duration;
use crypto::Signer;
use engine_json;
use serde_types::hash::Address;

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
            propose: Duration::from_millis(9000),
            prevote: Duration::from_millis(6000),
            precommit: Duration::from_millis(6000),
            commit: Duration::from_millis(10000),
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
            authorities: p.authorities
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            signer: Signer::from(p.signer),
            block_tx_limit: p.block_tx_limit as usize,
            tx_filter_size: p.tx_filter_size as usize,
            timer: TendermintTimer {
                propose: p.timeout_propose.map_or(dt.propose, to_duration),
                prevote: p.timeout_prevote.map_or(dt.prevote, to_duration),
                precommit: p.timeout_precommit.map_or(dt.precommit, to_duration),
                commit: p.timeout_commit.map_or(dt.commit, to_duration),
            },
        }
    }
}
