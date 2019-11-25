// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
            trace!("height is small than min_height: {} < {}", height, self.min_height,);
            return;
        } else if height > self.max_height {
            self.max_height = height;

            let old_min_height = self.min_height;
            self.min_height = if height > BLOCKLIMIT { height - BLOCKLIMIT + 1 } else { 0 };

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

#[cfg(test)]
mod history_heights_quick_check {
    use super::HistoryHeights;
    use quickcheck::Arbitrary;
    use quickcheck::Gen;

    #[derive(Clone, Debug)]
    struct HistoryHeightsArgs {
        history_heights: Vec<u64>,
    }

    impl Arbitrary for HistoryHeightsArgs {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut heights_ranges: Vec<u64> = vec![];
            for i in 0..201 {
                heights_ranges.push(i);
            }
            let mut history_heights: Vec<u64> = vec![];
            for _ in 0..200 {
                let index = g.next_u64() as usize % heights_ranges.len();
                history_heights.push(heights_ranges.remove(index));
            }

            HistoryHeightsArgs { history_heights }
        }
    }

    quickcheck! {
          fn prop(args: HistoryHeightsArgs) -> bool {
              let mut h = HistoryHeights::new();
              for i in args.history_heights {
                h.update_height(i);
              }
              let min = h.min_height();
              let mut sum: u64 = 0;
              for j in &h.heights {
                  sum += j - min + 1;
              }
              if h.is_init() {
                  sum == 101 * 50
              } else {
                  sum != 101 * 50
              }
          }
    }
}
