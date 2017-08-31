use libproto::*;
use libproto::communication::*;
use protobuf::core::parse_from_bytes;
use verifier::verifier;



pub fn handle_msg(payload: Vec<u8>,tx_pub: Sender<(String, Vec<u8>)>,v:verifier) {

            let (_, _, content) = parse_msg(payload.as_slice());
            match content {
                MsgClass::STATUS(status) => {
                    let height = status.get_height();
                    info!("got height {:?}", height);
                }
                MsgClass::VERIFYREQ(req) => {
                   trace!("got verify request {:?}", req);  
                   let req_msgs= req.get_reqs();
                   let resps = Vec::new();
                   for req in req_msgs {
                     let resp = VerifyRespMsg::new();
                     if v.verify_vub(req.get_height()) {
                         resp.set_ret(Ret::Ok);
                     } else {
                         resp.set_ret(Ret::OutOfTime);
                     }

                     match v.verify_sig(req) {
                         Ok(pubkey) => {
                           resp.set_signer(pubkey.to_vec());
                           resp.set_ret(Ret::Ok);
                           resps.push(resp)
                         }
                         Err(x) => {
                           resp.set_ret(Ret::BadSig);
                           resps.push(resp)
                         }
                     }
                     let vresp = VerifyResp::new();
                     vresp.set_resps(RepeatedField::from_slice(resps));

                   }
                
                }        
                _ => {}
            }


}
    
       
