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

//! Bridge between bloomchain crate types and cita LogBloom.

use bloomchain::group::BloomGroup;
use bloomchain::Bloom;
use log_entry::LogBloom;
use rlp::*;
use util::HeapSizeOf;

/// Represents group of X consecutive blooms.
#[derive(Debug, Clone)]
pub struct LogBloomGroup {
    blooms: Vec<LogBloom>,
}

impl From<BloomGroup> for LogBloomGroup {
    fn from(group: BloomGroup) -> Self {
        let blooms = group
            .blooms
            .into_iter()
            .map(|x| LogBloom::from(Into::<[u8; 256]>::into(x)))
            .collect();
        LogBloomGroup { blooms }
    }
}

impl Into<BloomGroup> for LogBloomGroup {
    fn into(self) -> BloomGroup {
        let blooms = self
            .blooms
            .into_iter()
            .map(|x| Bloom::from(Into::<[u8; 256]>::into(x)))
            .collect();
        BloomGroup { blooms }
    }
}

impl Decodable for LogBloomGroup {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let blooms = rlp.as_list()?;
        let group = LogBloomGroup { blooms };
        Ok(group)
    }
}

impl Encodable for LogBloomGroup {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.blooms);
    }
}

impl HeapSizeOf for LogBloomGroup {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}
