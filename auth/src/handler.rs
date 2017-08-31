use libproto::*;
use libproto::communication::*;
use protobuf::core::parse_from_bytes;


pub fn handle_msg(payload: Vec<u8>) {

            let (_, _, content) = parse_msg(payload.as_slice());
            match content {
                MsgClass::STATUS(status) => {
                    let height = status.get_height();
                    info!("got height {:?}", height);
                }
                MsgClass::VERIFYREQ(req) => {
                   let req_msgs= req.get_reqs();
                   for req in req_msgs {
                     verify_sig(req)
                     verify_vub(req)    
                   }
                
                
                }        
                _ => {}
            }


}
    
       
pub fn verify_sig(req:VerifyReqMsg) -> Result<Pubkey, Error> {
            let mut ret = true;
            let hash = req.get_hash()     
            let sig = req.get_signature()
            if sig.len() != SIGNATURE_BYTES_LEN {
                ret =false;
            } else {
                match sig.recover(&hash)
                  Ok(pubkey) => {
                    ret = pubkey;
                  }    
            }
            ret 
}
