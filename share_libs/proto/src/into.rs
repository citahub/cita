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

use super::*;
use blockchain::{RichStatus, Status};
use communication;
use protobuf::Message;
use submodules;
use topics;

impl Into<communication::Message> for Request {
    fn into(self) -> communication::Message {
        let msg = factory::create_msg(
            submodules::JSON_RPC,
            topics::REQUEST,
            communication::MsgType::REQUEST,
            self.write_to_bytes().unwrap(),
        );
        msg
    }
}

// impl Into<communication::Message> for blockchain::UnverifiedTransaction {
//     fn into(self) -> communication::Message {
//         let msg = factory::create_msg(
//             submodules::JSON_RPC,
//             topics::NEW_TX,
//             communication::MsgType::TX,
//             self.write_to_bytes().unwrap(),
//         );
//         msg
//     }
// }

impl Into<communication::Message> for Response {
    fn into(self) -> communication::Message {
        let msg = factory::create_msg(
            submodules::CHAIN,
            topics::RESPONSE,
            communication::MsgType::RESPONSE,
            self.write_to_bytes().unwrap(),
        );
        msg
    }
}

impl From<RichStatus> for Status {
    fn from(rich_status: RichStatus) -> Self {
        let mut status = Status::new();
        status.hash = rich_status.get_hash().to_vec();
        status.height = rich_status.get_height();

        status
    }
}
