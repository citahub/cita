///
/// zhongchao contract
///
///
///

use super::*;
use native::permission::action::ElementAction;
use std::str;

pub struct ZcPermission {
    functions: HashMap<Signature, Box<Function>>,
    creator: Vec<String>,
    sender: Vec<String>,
    applicant_of_creator: Vec<String>,
    applicant_of_sender: Vec<String>,
}

impl Contract for ZcPermission {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        self.functions.get(hash)
    }
}

impl ZcPermission {
    pub fn new() -> Self {
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
        };
        contract.functions.insert(0, Box::new(ZcPermission::apply_group));
        contract.functions.insert(1, Box::new(ZcPermission::verify_group));
        contract.functions.insert(2, Box::new(ZcPermission::query_group));
        contract

    }

    // TODO
    // apply to into/quit the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        Ok(GasLeft::Known(U256::from(0)))
    }
    // verify the application
    // fn verify_group(group: &string, action: &ElementAction);
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(36..68) {
                match data[68] {
                    0 => into_group(data.get(4..36).unwrap(), ext),
                    1 => quit_group(data.get(4..36).unwrap(), ext),
                    _ => false,
                };
                // let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }       
        // verify the into application
        fn into_group(group: &[u8], ext: &mut Ext) -> bool {
            match str::from_utf8(group).unwrap() {
                "sender" => {
                    // ext.set_storage(H256::from(0), H256::from(data));
                    true
                }
                "creator" => {
                    // ext.set_storage(H256::from(0), H256::from(data));
                    true
                },
                _ => false,
            }
        }
        // verify the quit application
        fn quit_group(group: &[u8], ext: &mut Ext) -> bool {
            match str::from_utf8(group).unwrap() {
                "sender" => {
                    // ext.set_storage(H256::from(0), H256::from(data));
                    true
                },
                "creator" => {
                    // ext.set_storage(H256::from(0), H256::from(data));
                    true
                },
                _ => false,
            }
        }

        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the permission
    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        Ok(GasLeft::Known(U256::from(0)))
    }
}
