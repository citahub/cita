// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use std::collections::HashSet;
use util::instrument::{unix_now, AsMillis};
use util::BLOCKLIMIT;

#[derive(Debug, Default)]
pub struct HistoryHeights {
    heights: HashSet<u64>,
    max_height: u64,
    min_height: u64,
    is_init: bool,
    last_timestamp: u64,
}

impl HistoryHeights {
    pub fn new() -> Self {
        HistoryHeights {
            heights: HashSet::new(),
            max_height: 0,
            min_height: 0,
            is_init: false,
            //init value is 0 mean first time must not too frequent
            last_timestamp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.heights.clear();
        self.max_height = 0;
        self.min_height = 0;
        self.is_init = false;
        self.last_timestamp = 0;
    }

    pub fn update_height(&mut self, height: u64) {
        // update 'min_height', 'max_height', 'heights'
        if height < self.min_height {
            trace!(
                "height is small than min_height: {} < {}",
                height,
                self.min_height,
            );
            return;
        } else if height > self.max_height {
            self.max_height = height;

            let old_min_height = self.min_height;
            self.min_height = if height > BLOCKLIMIT {
                height - BLOCKLIMIT + 1
            } else {
                0
            };

            self.heights.insert(height);
            for i in old_min_height..self.min_height {
                self.heights.remove(&i);
            }
        } else {
            self.heights.insert(height);
        }

        // update 'is_init'
        let mut is_init = true;
        for i in self.min_height..self.max_height {
            if !self.heights.contains(&i) {
                is_init = false;
                break;
            }
        }
        self.is_init = is_init;
    }

    pub fn next_height(&self) -> u64 {
        self.max_height + 1
    }

    pub fn is_init(&self) -> bool {
        self.is_init
    }

    pub fn max_height(&self) -> u64 {
        self.max_height
    }

    pub fn min_height(&self) -> u64 {
        self.min_height
    }

    // at least wait 3s from latest update
    pub fn is_too_frequent(&self) -> bool {
        AsMillis::as_millis(&unix_now()) < self.last_timestamp + 3000
    }

    pub fn update_time_stamp(&mut self) {
        // update time_stamp
        self.last_timestamp = AsMillis::as_millis(&unix_now());
    }
}

#[cfg(test)]
mod history_heights_tests {
    use super::HistoryHeights;

    #[test]
    fn basic() {
        let mut h = HistoryHeights::new();
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 1);

        h.update_height(60);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 61);

        for i in 0..60 {
            h.update_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 61);

        h.update_height(70);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 71);

        for i in 0..70 {
            h.update_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 71);

        h.update_height(99);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 100);

        for i in 0..99 {
            h.update_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 100);

        h.update_height(100);
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 101);

        h.update_height(101);
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 102);
    }
}
