///
/// zhongchao contract
///
///
///

use super::*;


pub struct ZcPermission {
    functions: HashMap<Signature, Box<Function>>,
    creater: Vec<String>,
    sender: Vec<String>,
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
            creater: vec![],
            sender: vec![],
        };
        contract.functions.insert(0, Box::new(ZcPermission::apply_group));
        contract.functions.insert(1, Box::new(ZcPermission::verify_group));
        contract.functions.insert(2, Box::new(ZcPermission::query_group));
        contract

    }


    // TODO
    // apply to into the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        Ok(GasLeft::Known(U256::from(0)))
    }
    // verify the application
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        Ok(GasLeft::Known(U256::from(0)))
    }
    // query the permission
    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        Ok(GasLeft::Known(U256::from(0)))
    }
}


