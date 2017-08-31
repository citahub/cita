extern crate util;
extern crate protobuf;
#[macro_use]
extern crate cita_crypto as crypto;

use std::sync::mpsc::Sender;

use libproto::auth::*;
use crypto::{PubKey, Signature, SIGNATURE_BYTES_LEN};


pub struct verifier {
   height: Option<u64>,

}

impl verifier {

 pub fn new() -> Self {
     verifier {
        height: None,
     }
 }

  pub fn set_height(&mut self, h: u64) {
      self.height = Some(h);      
  
  }

  pub fn verify_sig(&self, req:VerifyReqMsg) -> Result<Pubkey, Error> {
            let hash = req.get_hash();     
            let sig = req.get_signature();
            if sig.len() != SIGNATURE_BYTES_LEN {
                Err("signature err");
            } else {
                match sig.recover(&hash) {
                  Ok(pubkey) => Ok(pubkey),
                  Err(x) => Err(Error::from(x)),                                                                             
                }
            }
   }
  

  pub fn verify_vub(&self, height:u64) {
         if height < self.height.unwrap() && !self.height.is_none() {
             return true;
         }
         else{ 
             return false; 
         }      

  }

}
