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
#![rustfmt_skip]


use rand::{self, Rng};
use std::cmp;

/// A randomized exponential backoff policy for retrying operations.
///
/// See [Exponential Backoff in Distributed Systems]
/// (http://dthain.blogspot.com/2009/02/exponential-backoff-in-distributed.html)
/// for algorithm details.
pub struct Backoff {
    /// Initial backoff duration.
    initial: u32,

    /// Maximum backoff duration.
    max: u32,

    /// Number of retries since last reset.
    retries: u32,
}

impl Backoff {
    /// Creates a new exponential backoff policy with the provided initial
    /// and maximum duration in milliseconds.
    ///
    /// The initial duration should be set at the outer limits of expected
    /// response time for the service. For example, if your service responds in
    /// 1ms on average but in 10ms for 99% of requests, then set t=10.
    pub fn with_duration_range(initial: u32, max: u32) -> Backoff {
        assert!(initial > 0, "round-trip time must be greater than 0");
        Backoff {
            initial: initial,
            max: max,
            retries: 0,
        }
    }

    /// Resets the backoff to the initial state.
    pub fn reset(&mut self) {
        self.retries = 0;
    }

    /// Retrieves the next backoff duration in milliseconds.
    pub fn next_backoff_ms(&mut self) -> u64 {
        // Prevent overflow by testing if the backoff will be greater than the
        // max in an arithmeticaly stable manner, and if so return the max.
        if (self.max as f64 / self.initial as f64).log2() < self.retries as f64 {
            return self.max as u64;
        }

        let rand = rand::thread_rng().gen_range::<f64>(1.0, 2.0);
        let duration = ((self.initial as u64 * 2u64.pow(self.retries)) as f64 * rand) as u64;
        let ms = cmp::min(self.max as u64, duration);
        self.retries += 1;
        ms
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let mut backoff = Backoff::with_duration_range(1, 18);

        let a = backoff.next_backoff_ms();
        assert!(a >= 1 && a < 2);

        let b = backoff.next_backoff_ms();
        assert!(b >= 2 && b < 4);

        let c = backoff.next_backoff_ms();
        assert!(c >= 4 && c < 8);

        let d = backoff.next_backoff_ms();
        assert!(d >= 8 && d < 16);

        let e = backoff.next_backoff_ms();
        assert!(e >= 16 && e <= 18);

        let f = backoff.next_backoff_ms();
        assert!(f >= 18 && e <= 18);

        backoff.reset();

        let g = backoff.next_backoff_ms();
        assert!(g >= 1 && g < 2);
    }
}
