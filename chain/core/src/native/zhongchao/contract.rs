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

///
/// zhongchao contract
///

use super::*;
use native::permission::action::ElementAction;
use std::str;
use rustc_hex::ToHex;

// undo: check
// check_user_in(group);
// check_has_role(group, role);
// check_permission(group, permission);
//
pub struct ZcPermission {
    // Key is signature of function in the contract, value is contract function
    functions: HashMap<Signature, Box<Function>>,
    // Group that has 'create' permission
    creator: Vec<String>,
    // Group that has 'send' permission
    sender: Vec<String>,
    // 申请加入含create的Group的user
    applicant_of_creator: Vec<String>,
    // 申请加入含send的Group的user
    applicant_of_sender: Vec<String>,
    // user : groups
    groups: HashMap<String, Vec<String>>,
    // group : roles
    roles: HashMap<String, Vec<String>>,
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
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        contract.functions.insert(0, Box::new(ZcPermission::apply_group));
        contract.functions.insert(1, Box::new(ZcPermission::verify_group));
//        contract.functions.insert(2, Box::new(ZcPermission::query_group));
//        contract.functions.insert(3, Box::new(ZcPermission::query_role));
//        contract.functions.insert(4, Box::new(ZcPermission::grant_role));
//        contract.functions.insert(5, Box::new(ZcPermission::revoke_role));
//        contract.functions.insert(6, Box::new(ZcPermission::quit_group));
//        contract.functions.insert(7, Box::new(ZcPermission::check_user_in));
        contract
    }

    // init
    // just init one user for each group: sender and creator
    // !! need to discuss
    pub fn init(user_send: &String, user_creat: &String, ext:&mut Ext)  {
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };

        // init
        contract.sender.push(user_send.to_string());
        contract.creator.push(user_creat.to_string());
    }

    // check_user_in(group);
    pub fn check_user_in(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        // undo: check the permission
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let groups = contract.groups;

        let user = params.clone().sender.to_hex();
        let data = params.clone().data.unwrap_or("error".as_bytes().to_owned());
        // TODO unwrap()
        let gr = String::from_utf8(data.get(4..36).unwrap().to_vec()).unwrap();
        match groups.get(&user) {
            Some(group) => {
                let ug = group.clone();
                if ug.contains(&gr) {
                    return true;
                }
                else {
                    return false;
                }
                //                let ret_code = ZcPermission::to_u8(ug).as_bytes();
                //                Ok(GasLeft::NeedsReturn(U256::from(0), ret_code))
            },
            None => { return false; }
        }
//        Ok(GasLeft::Known(U256::from(0)))
    }

    // apply to into/quit the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };

        let user = params.sender.to_hex();
        if let Some(ref data) = params.data {
            ZcPermission::into_temp(data.get(4..36).unwrap(), ext, &mut contract, user);
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // verify the application
    // fn verify_group(group: &string);
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        // undo: check the permission
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };

        if let Some(ref data) = params.data {
            // let _ = ext.set_storage(H256::from(0), H256::from(data));
            ZcPermission::into_group(data.get(4..36).unwrap(), ext, &mut contract);
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the permission
    pub fn query_group(params: & ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        // undo: check the permission
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let groups = contract.groups;

        let user = params.sender.to_hex();
        match groups.get(&user) {
            Some(group) => {
                let ug = group.clone();
//                let ret_code = ZcPermission::to_u8(ug).as_bytes();
//                Ok(GasLeft::NeedsReturn(U256::from(0), ret_code))
            },
            None => {}
        }
        Ok(GasLeft::Known(U256::from(0)))
    }


    // query the role of the group
    // fn query_role(group: String) -> Vec<String>
    pub fn query_role(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        // undo: check the permission
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let group = data.get(4..36).unwrap();
            match roles.get(&String::from_utf8(group.to_vec()).unwrap()) {
                Some(role) => {
                    let group_role = role.clone();
//                    let ret_code = ZcPermission::to_u8(group_role).as_bytes();
//                    Ok(GasLeft::NeedsReturn(U256::from(0), ret_code))
                    Ok(GasLeft::Known(U256::from(0)))
                },
                None => Ok(GasLeft::Known(U256::from(0)))
            }
        } else {
            Ok(GasLeft::Known(U256::from(0)))
        }
    }

    // grant the role to a user
    // fn grant_role(group: &String, role: &Role, user: &String) -> bool
    pub fn grant_role(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            let param_role = data.get(36..68).unwrap();
            let param_user = data.get(68..100).unwrap();

            match roles.get(&String::from_utf8(param_group.to_vec()).unwrap()) {
                Some(mut role) => {
                    if !role.contains(&String::from_utf8(param_role.to_vec()).unwrap()) {
                        return false;
                    }
                },
                None => { return false; },
            };

            match groups.get_mut(&String::from_utf8(param_user.to_vec()).unwrap()) {
                Some(group) => group.push(String::from_utf8(param_group.to_vec()).unwrap()),
                None => { return false; },
            };
        }
        true
    }

    // revoke the role to a user
    // fn revoke_role(group: &String, role: &Role, user: &String) -> bool;
    pub fn revoke_role(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            let param_role = data.get(36..68).unwrap();
            let param_user = data.get(68..100).unwrap();

            match roles.get(&String::from_utf8(param_group.to_vec()).unwrap()) {
                Some(mut role) => {
                    if !role.contains(&String::from_utf8(param_role.to_vec()).unwrap()) {
                        return false;
                    }
                },
                None => { return false; }
            }

            match groups.get_mut(&String::from_utf8(param_user.to_vec()).unwrap()) {
                Some(group) => {
                    match group.remove_item(&String::from_utf8(param_user.to_vec()).unwrap()) {
                        Some(user) => { return true; },
                        None => { return false; }
                    }
                },
                None => { return false; }
            }
        }
        true
    }

    // quit the group
    // fn quit_group(group: &String) -> bool;
    pub fn quit_group(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;

        let user = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            match groups.get_mut(&user) {
                Some(vec_group) => {
                    match vec_group.remove_item(&String::from_utf8(param_group.to_vec()).unwrap()) {
                        Some(group) => { return true; },
                        None => { return false; }
                    }
                }
                None => { return false; }
            }
        }
        true
    }


    pub fn into_temp(group: &[u8], ext: &mut Ext, contract: &mut ZcPermission, user: String) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                contract.applicant_of_sender.push(user);
                true
            },
            "creator" => {
                contract.applicant_of_creator.push(user);
                false
            },
            _ => false,
        }
    }

    // verify the into application
    pub fn into_group(group: &[u8], ext: &mut Ext, contract: &mut ZcPermission) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                // ext.set_storage(H256::from(0), H256::from(data));
                for user in contract.applicant_of_sender.clone() {
                    contract.sender.push(user);
                }
                true
            }
            "creator" => {
                // ext.set_storage(H256::from(0), H256::from(data));
                for user in contract.applicant_of_creator.clone() {
                    contract.creator.push(user);
                }
                true
            },
            _ => false,
        }
    }

    // convert Vec<String> to &[u8], every string followed by a blank
    pub fn to_u8(groups: Vec<String>) -> String {
        let mut res = String::new();
        let ug = groups.clone();
        for gr in ug {
            res.push_str(&gr);
            res.push_str(" ");
        }
        res
    }
}
