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

use bloomchain::Bloom;
use bloomchain::group::{BloomGroup, GroupPosition};
use rlp::*;
use basic_types::LogBloom;

/// Helper structure representing bloom of the trace.
#[derive(Clone)]
pub struct BlockTracesBloom(LogBloom);

impl From<LogBloom> for BlockTracesBloom {
	fn from(bloom: LogBloom) -> BlockTracesBloom {
		BlockTracesBloom(bloom)
	}
}

impl From<Bloom> for BlockTracesBloom {
	fn from(bloom: Bloom) -> BlockTracesBloom {
		let bytes: [u8; 256] = bloom.into();
		BlockTracesBloom(LogBloom::from(bytes))
	}
}

impl Into<Bloom> for BlockTracesBloom {
	fn into(self) -> Bloom {
		let log = self.0;
		Bloom::from(log.0)
	}
}

/// Represents group of X consecutive blooms.
#[derive(Clone)]
pub struct BlockTracesBloomGroup {
	blooms: Vec<BlockTracesBloom>,
}

impl From<BloomGroup> for BlockTracesBloomGroup {
	fn from(group: BloomGroup) -> Self {
		let blooms = group.blooms
			.into_iter()
			.map(From::from)
			.collect();

		BlockTracesBloomGroup {
			blooms: blooms
		}
	}
}

impl Into<BloomGroup> for BlockTracesBloomGroup {
	fn into(self) -> BloomGroup {
		let blooms = self.blooms
			.into_iter()
			.map(Into::into)
			.collect();

		BloomGroup {
			blooms: blooms
		}
	}
}

impl Decodable for BlockTracesBloom {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		Decodable::decode(decoder).map(BlockTracesBloom)
	}
}

impl Encodable for BlockTracesBloom {
	fn rlp_append(&self, s: &mut RlpStream) {
		Encodable::rlp_append(&self.0, s)
	}
}

impl Decodable for BlockTracesBloomGroup {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		let blooms = Decodable::decode(decoder)?;
		let group = BlockTracesBloomGroup {
			blooms: blooms
		};
		Ok(group)
	}
}

impl Encodable for BlockTracesBloomGroup {
	fn rlp_append(&self, s: &mut RlpStream) {
		Encodable::rlp_append(&self.blooms, s)
	}
}

/// Represents `BloomGroup` position in database.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TraceGroupPosition {
	/// Bloom level.
	pub level: u8,
	/// Group index.
	pub index: u32,
}

impl From<GroupPosition> for TraceGroupPosition {
	fn from(p: GroupPosition) -> Self {
		TraceGroupPosition {
			level: p.level as u8,
			index: p.index as u32,
		}
	}
}
