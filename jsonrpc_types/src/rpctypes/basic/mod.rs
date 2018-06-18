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

mod arbitrary_data;
mod boolean;
mod fixed_data;
mod integer;
mod quantity;
mod tags;
mod variadic;

pub use self::arbitrary_data::Data;
pub use self::boolean::Boolean;
pub use self::fixed_data::{Data20, Data32};
pub use self::integer::Integer;
pub use self::quantity::Quantity;
pub use self::tags::BlockTag;
pub use self::variadic::VariadicValue;

// serde: Tuple enums with single element should not be a json-array
// https://github.com/serde-rs/serde/pull/111
#[derive(Debug, Clone, PartialEq)]
pub struct OneItemTupleTrick {}

impl Default for OneItemTupleTrick {
    fn default() -> Self {
        OneItemTupleTrick {}
    }
}
