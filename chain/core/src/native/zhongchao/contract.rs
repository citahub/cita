///
/// zhongchao contract
///
///
///

use super::*;
pub struct UserGroup {
    functions: HashMap<Signature, Box<Function>>,
}

impl Contract for UserGroup {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        self.functions.get(hash)
    }
}

//impl UserGroup {
//        let mut groups = HashMap::new();
//        let mut role_groups = HashMap::new();
//        let zc_creater = Group {
//        name: String::from("zc_creater"),
//        users: [];
//        groups: [],
//        switch: Switch::off,
//        }
//
//        let zc_senter = Group {
//        name: String::from("zc_senter"),
//        users: [];
//        groups: [],
//        switch: Switch::off,
//        }
//
//        let zc_create = Role {
//        name: String::from("zc_create"),
//        permissions: ["create_contract", "update_group"],
//        }
//
//        let zc_sent = Role {
//        name: String::from("zc_sent"),
//        permissions: ["sent_tx", "update_group"],
//        }

//    pub fn new() -> Self {
//
//        let mut contract = NowPay { functions: HashMap::<Signature, Box<Function>>::new() };
//        contract.functions.insert(0, Box::new(UserGroup::apply_group));
//        contract.functions.insert(1, Box::new(UserGroup::verify_group));
//        contract.functions.insert(2, Box::new(UserGroup::query_group));
//
//        contract
//    }

//    // apply to into the group
//    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {}
//    // verify the application
//    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {}
//    // query the permission
//    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {}
//}
