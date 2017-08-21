use action_params::ActionParams;
use evm::{self, Ext, GasLeft};
use std::collections::HashMap;
use util::{H256, U256};

use data_structure::{Group, Role};

////////////////////////////////////////////////////////////////////////////////
pub type Signature = u32;
pub type Function = Fn(&ActionParams, &mut Ext) -> evm::Result<GasLeft<'static>> + Sync + Send;

////////////////////////////////////////////////////////////////////////////////
// Contract
pub trait Contract: Sync + Send {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>>;
    fn exec(&self, params: &ActionParams, mut ext: &mut Ext) {
        if let Some(data) = params.clone().data.unwrap().get(0..4) {
            let signature = data.iter().fold(0u32, |acc, &x| (acc << 8) + (x as u32));
            if let Some(exec_call) = self.get_function(&signature) {
                //let cost = self.engine.cost_of_builtin(&params.code_address, data);
                let cost = U256::from(100);
                if cost <= params.gas {
                    let _ = exec_call(params, ext);
                    //self.state.discard_checkpoint();
                    return;
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// ZhongChao
pub struct UserGroup {
    functions: HashMap<Signature, Box<Function>>,
}

impl Contract for Group {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        self.functions.get(hash)
    }
}

impl UserGroup {

    let mut groups = HashMap::new();
    let mut role_groups = HashMap::new();
    let zc_creater = Group {
        name: String::from("zc_creater"),
        users: [];
        groups: [],
        switch: Switch::off,
    }
    
    let zc_senter = Group {
        name: String::from("zc_senter"),
        users: [];
        groups: [],
        switch: Switch::off,
    }
    
    let zc_create = Role {
        name: String::from("zc_create"),
        permissions: ["create_contract", "update_group"],
    }
    
    let zc_sent = Role {
        name: String::from("zc_sent"),
        permissions: ["sent_tx", "update_group"],
    }
    pub fn new() -> Self {
        let mut contract = NowPay { functions: HashMap::<Signature, Box<Function>>::new() };
        contract.functions.insert(0, Box::new(UserGroup::apply_group));
        contract.functions.insert(1, Box::new(UserGroup::verify_group));
        contract.functions.insert(2, Box::new(UserGroup::query_group));
        contract
    }
    // pub fn set_value(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
    //     if let Some(ref data) = params.data {
    //         if let Some(data) = data.get(4..32) {
    //             let _ = ext.set_storage(H256::from(0), H256::from(data));
    //         }
    //     }
    //     Ok(GasLeft::Known(U256::from(0)))
    // }
    // apply to into the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
    }
    // verify the application
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
    }
    // query the permission
    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
    }
}
