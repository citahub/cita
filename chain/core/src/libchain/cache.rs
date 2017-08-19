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

/// Represents blockchain's in-memory cache size in bytes.
#[derive(Debug)]
pub struct CacheSize {
    /// Blocks cache size.
    pub blocks: usize,
    /// Transaction addresses cache size.
    pub transaction_addresses: usize,
    /// Blooms cache size.
    pub blocks_blooms: usize,
    /// Block receipts size.
    pub block_receipts: usize,
}

impl CacheSize {
    /// Total amount used by the cache.
    pub fn total(&self) -> usize {
        self.blocks + self.transaction_addresses + self.blocks_blooms + self.block_receipts
    }
}
