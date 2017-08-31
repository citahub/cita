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
