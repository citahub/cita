// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

//! A multiplexed cita protocol

use byteorder::{ByteOrder, NetworkEndian};
use bytes::{BufMut, Bytes, BytesMut};
use cita_types::Address;
use logger::{error, warn};
use std::str;

/// Implementation of the multiplexed line-based protocol.
///
/// Frames begin with a 4 byte header, consisting of the numeric request ID
/// encoded in network order, followed by the frame payload encoded as a UTF-8
/// string and terminated with a '\n' character:
///
/// # An example frame:
///
/// +------------------------+--------------------------+
/// | Type                   | Content                  |
/// +------------------------+--------------------------+
/// | Symbol for Start       | \xDEADBEEF               |
/// | Length of Full Payload | u32                      |
/// | Version                | u64                      |
/// | Address                | u8;20                    |
/// | Length of Key          | u8                       |
/// | TTL                    | u8                       |
/// | Reserved               | u8;2                     |
/// +------------------------+--------------------------+
/// | Key                    | bytes of a str           |
/// +------------------------+--------------------------+
/// | Message                | a serialize data         |
/// +------------------------+--------------------------+
///

// Start of network messages.
const NETMSG_START: u64 = 0xDEAD_BEEF_0000_0000;

/// According to CITA frame, defines its frame header length as:
/// "Symbol for Start" + "Length of Full Payload" + "Version"+
///  "Address"+ "Length of Key"+"TTL"+"Reserved",
/// And this will consume "4 + 4 + 8 + 20 + 1 + 1+ 2" fixed-lengths of the frame.
pub const CITA_FRAME_HEADER_LEN: usize = 4 + 4 + 8 + 20 + 1 + 1 + 2;
pub const HEAD_VERSION_OFFSET: usize = 4 + 4;
pub const HEAD_ADDRESS_OFFSET: usize = 4 + 4 + 8;
pub const HEAD_KEY_LEN_OFFSET: usize = 4 + 4 + 8 + 20;
pub const HEAD_TTL_OFFSET: usize = 4 + 4 + 8 + 20 + 1;
pub const DEFAULT_TTL_NUM: u8 = 9;

pub const CONSENSUS_STR: &str = "consensus";

#[derive(Debug, Clone)]
pub struct NetMessageUnit {
    pub key: String,
    pub data: Vec<u8>,
    pub addr: Address,
    pub version: u64,
    pub ttl: u8,
}

impl NetMessageUnit {
    pub fn new(key: String, data: Vec<u8>, addr: Address, version: u64, ttl: u8) -> Self {
        NetMessageUnit {
            key,
            data,
            addr,
            version,
            ttl,
        }
    }
}

impl Default for NetMessageUnit {
    fn default() -> Self {
        NetMessageUnit {
            key: String::default(),
            data: Vec::new(),
            addr: Address::zero(),
            version: 0,
            ttl: DEFAULT_TTL_NUM,
        }
    }
}

pub fn pubsub_message_to_network_message(info: &NetMessageUnit) -> Option<Bytes> {
    let length_key = info.key.len();
    // Use 1 byte to store key length.
    if length_key == 0 || length_key > u8::max_value() as usize {
        error!(
            "[CitaProtocol] The MQ message key is too long or empty {}.",
            info.key
        );
        return None;
    }
    let length_full = length_key + info.data.len();
    // Use 1 bytes to store the length for key, then store key, the last part is body.
    if length_full > u32::max_value() as usize {
        error!(
            "[CitaProtocol] The MQ message with key {} is too long {}.",
            info.key,
            info.data.len()
        );
        return None;
    }

    let mut buf = BytesMut::with_capacity(length_full + CITA_FRAME_HEADER_LEN);
    let request_id = NETMSG_START + length_full as u64;
    buf.put_u64_be(request_id);
    buf.put_u64_be(info.version);
    buf.put(info.addr.to_vec());
    buf.put_u8(length_key as u8);
    buf.put_u8(info.ttl);
    buf.put_u16_be(0);

    buf.put(info.key.as_bytes());
    buf.put_slice(&info.data);

    Some(buf.into())
}

pub fn network_message_to_pubsub_message(buf: &mut BytesMut) -> Option<NetMessageUnit> {
    if buf.len() < CITA_FRAME_HEADER_LEN {
        return None;
    }

    let head_buf = buf.split_to(CITA_FRAME_HEADER_LEN);

    let request_id = NetworkEndian::read_u64(&head_buf);
    let netmsg_start = request_id & 0xffff_ffff_0000_0000;
    let length_full = (request_id & 0x0000_0000_ffff_ffff) as usize;
    if netmsg_start != NETMSG_START {
        return None;
    }
    if length_full > buf.len() || length_full == 0 {
        return None;
    }

    let addr = Address::from_slice(&head_buf[HEAD_ADDRESS_OFFSET..]);
    let version = NetworkEndian::read_u64(&head_buf[HEAD_VERSION_OFFSET..]);

    let length_key = head_buf[HEAD_KEY_LEN_OFFSET] as usize;
    let ttl = head_buf[HEAD_TTL_OFFSET];
    if length_key == 0 {
        error!("[CitaProtocol] Network message key is empty.");
        return None;
    }
    if length_key > buf.len() {
        error!(
            "[CitaProtocol] Buffer is not enough for key {} > {}.",
            length_key,
            buf.len()
        );
        return None;
    }
    let key_buf = buf.split_to(length_key);
    let key_str_result = str::from_utf8(&key_buf);
    if key_str_result.is_err() {
        error!(
            "[CitaProtocol] Network message parse key error {:?}.",
            key_buf
        );
        return None;
    }
    let key = key_str_result.unwrap().to_string();
    if length_full == length_key {
        warn!("[CitaProtocol] Network message is empty.");
    }
    Some(NetMessageUnit {
        key,
        data: buf.to_vec(),
        addr,
        version,
        ttl,
    })
}

#[cfg(test)]
mod test {
    use super::{
        network_message_to_pubsub_message, pubsub_message_to_network_message, NetMessageUnit,
    };
    use bytes::BytesMut;

    #[test]
    fn convert_empty_message() {
        let buf = pubsub_message_to_network_message(&NetMessageUnit::default());
        let pub_msg_opt = network_message_to_pubsub_message(&mut BytesMut::new());
        assert!(pub_msg_opt.is_none());
        assert!(buf.is_none());
    }

    #[test]
    fn convert_messages() {
        let mut msg = NetMessageUnit::default();
        let key = "this-is-the-key".to_string();
        let data = vec![1, 3, 5, 7, 9];
        msg.key = key.clone();
        msg.data = data.clone();

        let buf = pubsub_message_to_network_message(&msg).unwrap();
        let pub_msg_opt = network_message_to_pubsub_message(&mut buf.try_mut().unwrap());
        assert!(pub_msg_opt.is_some());
        let info = pub_msg_opt.unwrap();
        assert_eq!(key, info.key);
        assert_eq!(data, info.data);
    }
}
