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

use libproto::*;
use libproto::communication::*;
use protobuf::core::parse_from_bytes;


pub fn handle_msg(payload: Vec<u8>) {

    if let Ok(msg) = parse_from_bytes::<communication::Message>(payload.as_ref()) {
        let t = msg.get_field_type();
        let cid = msg.get_cmd_id();
        if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) && t == MsgType::STATUS {
            let (_, _, content) = parse_msg(payload.as_slice());
            match content {
                MsgClass::STATUS(status) => {
                    let height = status.get_height();
                    info!("got height {:?}", height);
                }
                _ => {}
            }
        }
    }

}
