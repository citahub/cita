use util::{Bytes, Address};
use libproto::request::Call;

/// Call request
#[derive(Debug, Default, PartialEq)]
pub struct CallRequest {
	/// From
	pub from: Option<Address>,
	/// To
	pub to: Address,
	/// Data
	pub data: Option<Bytes>,
}

impl From<Call> for CallRequest {
	fn from(call: Call) -> Self {
		CallRequest {
			from: if call.get_from().is_empty() { None } else { Some(Address::from(call.get_from())) } ,
			to: Address::from(call.get_to()),
			data: if call.data.is_empty() { None } else { Some(Bytes::from(call.data)) },
		}
	}
}

#[cfg(test)]
mod tests {
}
