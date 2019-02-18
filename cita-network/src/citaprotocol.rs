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

//! A multiplexed cita protocol

use byteorder::{ByteOrder, NetworkEndian};
use bytes::BufMut;
use bytes::BytesMut;
use std::io;
use std::str;
use tokio::codec::{Decoder, Encoder};

pub type CitaRequest = (String, Vec<u8>);
pub type CitaResponse = Option<(String, Vec<u8>)>;

/// Our multiplexed line-based codec
pub struct CitaCodec;

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
/// +------------------------+--------------------------+
/// | Length of Key          | u8                       |
/// | Key                    | bytes of a str           |
/// +------------------------+--------------------------+
/// | Message                | a serialize data         |
/// +------------------------+--------------------------+
///

// Start of network messages.
const NETMSG_START: u64 = 0xDEAD_BEEF_0000_0000;

fn opt_bytes_extend(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

impl Decoder for CitaCodec {
    type Item = CitaRequest;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        Ok(network_message_to_pubsub_message(buf))
    }
}

impl Encoder for CitaCodec {
    type Item = CitaResponse;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        pubsub_message_to_network_message(buf, msg);
        Ok(())
    }
}

pub fn pubsub_message_to_network_message(buf: &mut BytesMut, msg: Option<(String, Vec<u8>)>) {
    let mut request_id_bytes = [0; 8];
    if let Some((key, body)) = msg {
        let length_key = key.len();
        // Use 1 byte to store key length.
        if length_key > u8::max_value() as usize {
            error!("The MQ message key is too long {}.", key);
            return;
        }
        // Use 1 bytes to store the length for key, then store key, the last part is body.
        let length_full = 1 + length_key + body.len();
        if length_full > u32::max_value() as usize {
            error!(
                "The MQ message with key {} is too long {}.",
                key,
                body.len()
            );
            return;
        }
        let request_id = NETMSG_START + length_full as u64;
        NetworkEndian::write_u64(&mut request_id_bytes, request_id);
        opt_bytes_extend(buf, &request_id_bytes);
        buf.put_u8(length_key as u8);
        opt_bytes_extend(buf, key.as_bytes());
        opt_bytes_extend(buf, &body);
    } else {
        let request_id = NETMSG_START;
        NetworkEndian::write_u64(&mut request_id_bytes, request_id);
        opt_bytes_extend(buf, &request_id_bytes);
    }
}

pub fn network_message_to_pubsub_message(buf: &mut BytesMut) -> Option<(String, Vec<u8>)> {
    if buf.len() < 8 {
        return None;
    }

    let request_id = NetworkEndian::read_u64(buf.as_ref());
    let netmsg_start = request_id & 0xffff_ffff_0000_0000;
    let length_full = (request_id & 0x0000_0000_ffff_ffff) as usize;
    if netmsg_start != NETMSG_START {
        return None;
    }
    if length_full + 8 > buf.len() {
        return None;
    }
    let _request_id_buf = buf.split_to(8);

    if length_full == 0 {
        return None;
    }
    let mut payload_buf = buf.split_to(length_full);

    let length_key = payload_buf[0] as usize;
    let _length_key_buf = payload_buf.split_to(1);
    if length_key == 0 {
        error!("network message key is empty.");
        return None;
    }
    if length_key > payload_buf.len() {
        error!(
            "Buffer is not enough for key {} > {}.",
            length_key,
            buf.len()
        );
        return None;
    }
    let key_buf = payload_buf.split_to(length_key);
    let key_str_result = str::from_utf8(&key_buf);
    if key_str_result.is_err() {
        error!("network message parse key error {:?}.", key_buf);
        return None;
    }
    let key = key_str_result.unwrap().to_string();
    if length_full == 1 + length_key {
        warn!("network message is empty.");
    }
    Some((key, payload_buf.to_vec()))
}

#[cfg(test)]
mod test {
    use super::{network_message_to_pubsub_message, pubsub_message_to_network_message};
    use bytes::BytesMut;

    #[test]
    fn convert_empty_message() {
        let mut buf = BytesMut::with_capacity(4 + 4);
        pubsub_message_to_network_message(&mut buf, None);
        let pub_msg_opt = network_message_to_pubsub_message(&mut buf);
        assert!(pub_msg_opt.is_none());
    }

    #[test]
    fn convert_messages() {
        let key = "this-is-the-key".to_string();
        let msg: Vec<u8> = vec![1, 3, 5, 7, 9];
        let mut buf = BytesMut::with_capacity(4 + 4 + 1 + key.len() + msg.len());
        pubsub_message_to_network_message(&mut buf, Some((key.clone(), msg.clone())));
        let pub_msg_opt = network_message_to_pubsub_message(&mut buf);
        assert!(pub_msg_opt.is_some());
        let (key_new, msg_new) = pub_msg_opt.unwrap();
        assert_eq!(key, key_new);
        assert_eq!(msg, msg_new);
    }
}
