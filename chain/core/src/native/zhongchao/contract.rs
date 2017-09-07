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
use super::storage::{Array, Map};
use rustc_hex::ToHex;
use std::str;

// prob: return the creator/sender. static lifetime
#[allow(unused_variables, dead_code)]
pub struct ZcPermission {
    // Key is signature of function in the contract, value is contract function
    functions: HashMap<Signature, Box<Function>>,
    // Group that has 'create' permission
    // position: 0
    creator: Vec<String>,
    // Group that has 'send' permission
    // position: 1
    sender: Vec<String>,
    // user : permissions 
    // position: 2
    user_permissions: HashMap<String, Vec<String>>,
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
            user_permissions: HashMap::<String, Vec<String>>::new(),
        };
        contract.functions.insert(0, Box::new(ZcPermission::init));
        contract.functions.insert(0x52e1ef9f, Box::new(ZcPermission::grant_permission));
        contract.functions.insert(0x077ece9d ,Box::new(ZcPermission::revoke_permission));
        contract.functions.insert(0x9fd99bc2, Box::new(ZcPermission::query_users_of_permission));
        contract.functions.insert(0xd8987c1c ,Box::new(ZcPermission::query_permission));
        // contract.functions.insert(5, Box::new(ZcPermission::quit_group));
        contract
    }

    // setup
    // undo: Vec<String>
    // undo: use Address
    // pub fn init(user_send: &String, user_create: &String, ext: &mut Ext) -> Result<(), storage::Error> {
    pub fn init(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            // user_send 
            let send = data.get(100..132).unwrap();
            // user_create
            let create = data.get(164..196).unwrap();
            let user_send = String::from_utf8(send.to_vec()).unwrap();
            let user_create = String::from_utf8(create.to_vec()).unwrap();
            // init the array
            let create_array = Array::new(H256::from(0));
            let send_array = Array::new(H256::from(1));
            // init the map
            let mut map = Map::new(H256::from(2));
            let send_permissions = map.get_array(user_send.clone()).unwrap();
            let create_permissions = map.get_array(user_create.clone()).unwrap();
            // the two group name
            let sender = String::from("sender");
            let creator = String::from("creator");
            // set to the array
            let _ = send_array.set_len(ext, 1u64);
            let _ = send_array.set_bytes::<String>(ext, 0u64, user_send.clone());
            let _ = create_array.set_len(ext, 1u64);
            let _ = create_array.set_bytes::<String>(ext, 0u64, user_create.clone());
            // add to the groups
            let _ = send_permissions.set_len(ext, 1u64);
            let _ = send_permissions.set_bytes::<String>(ext, 0u64, sender.clone());
            let _ = create_permissions.set_len(ext, 1u64);
            let _ = create_permissions.set_bytes::<String>(ext, 0u64, creator.clone());

        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // grant the permission to a user
    // interface: grant_permission(permission: &String, user: &String) -> bool
    // permission: "send"/"create"
    pub fn grant_permission(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            // send or create
            let permission_data = data.get(100..132).unwrap();
            // user
            let user_data = data.get(164..196).unwrap();
            let param_permission = String::from_utf8(permission_data.to_vec()).unwrap();
            let param_user_group = String::from_utf8(user_data.clone().to_vec()).unwrap();
            let param_user_map = String::from_utf8(user_data.clone().to_vec()).unwrap();

            if !(ZcPermission::check_permission(&sender, &param_permission, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }
            if ZcPermission::check_permission(&param_user_group, &param_permission, ext) {
                return Err(Error::Internal("The user already has the permission. no need to grant".to_string()));
            }

            // add map
            let mut groups_map = Map::new(H256::from(2));
            let groups_array = groups_map.get_array(param_user_map).unwrap();
            let groups_length = groups_array.get_len(ext).unwrap();
            let _ = groups_array.set_len(ext, groups_length + 1);
            let _ = groups_array.set_bytes::<String>(ext, groups_length, param_permission.clone());

            if !(ZcPermission::into_group(user_data, param_user_group, ext)) {
                return Err(Error::Internal("fail to grant the permission".to_string()));
            }

        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // revoke the permission to a user
    // interface: revoke_permission(permission: &String, user: &String) -> bool;
    // permission: "send"/"create"
    pub fn revoke_permission(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            // send/create
            let permission_data = data.get(100..132).unwrap();
            // user
            let user_data = data.get(164..196).unwrap();
            let param_permission = String::from_utf8(permission_data.to_vec()).unwrap();
            let param_user_group = String::from_utf8(user_data.clone().to_vec()).unwrap();
            let param_user_map = String::from_utf8(user_data.clone().to_vec()).unwrap();

            if !(ZcPermission::check_permission(&sender, &param_permission, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }
            if !(ZcPermission::check_permission(&param_user_group, &param_permission, ext)) {
                return Err(Error::Internal("The user already has left the group".to_string()));
            }

            if !(ZcPermission::out_group(user_data, param_user_group, ext)) {
                return Err(Error::Internal("fail to revoke the permission".to_string()));
            }

            // map
            let mut groups_map = Map::new(H256::from(2));
            let groups_array = groups_map.get_array(param_user_map).unwrap();
            let groups_length = groups_array.get_len(ext).unwrap();
            let send = String::from("sender");
            let create = String::from("creator");

            match groups_length {
                1 => {
                    let _ =groups_array.set_len(ext, 0u64);
                    Ok(GasLeft::Known(U256::from(0)))
                }
                2 => {
                    let _ = groups_array.set_len(ext, 1u64);
                    if create.eq(&param_permission) {
                        let _ = groups_array.set_bytes::<String>(ext, groups_length, send.clone());
                    } else {
                        let _ = groups_array.set_bytes::<String>(ext, groups_length, create.clone());
                    }
                    Ok(GasLeft::Known(U256::from(0)))
                },
                _ => Err(Error::Internal("wrong data".to_string()))
            }
        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
    }

    // query users of the permission: send/create
    // interface: query_users_of_permission(permission: &String);
    pub fn query_users_of_permission(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        let mut result = String::new();

        if let Some(ref data) = params.data {
            // permission 
            let permission_data = data.get(68..100).unwrap();
            // let permission = String::from_utf8(permission_data.to_vec()).unwrap();

            // init the array
            let create = Array::new(H256::from(0));
            let send = Array::new(H256::from(1));
            match str::from_utf8(permission_data).unwrap() {
                "send" =>  {
                    let length = send.get_len(ext).unwrap();
                    for len in 0..length {
                        let user = send.get_bytes::<String>(ext, len).unwrap();
                        result.push_str(&user);
                    }
                },
                "create" =>  {
                    let length = create.get_len(ext).unwrap();
                    for len in 0..length {
                        let user = create.get_bytes::<String>(ext, len).unwrap();
                        result.push_str(&user);
                    }
                },
                _ => return Err(Error::Internal("wrong data".to_string())),
            }
        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }

        // undo: should return result: solve the static lifetime prob
        // Ok(GasLeft::NeedsReturn(U256::from(0), result.as_bytes()))
        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the user's permission 
    // interface: query_permission()
    // return: "create"/"send"/"create send"
    pub fn query_permission(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        let sender = params.sender.to_hex();
        let mut map = Map::new(H256::from(2));
        let user_groups = map.get_array(sender.clone()).unwrap();
        let length = user_groups.get_len(ext).unwrap();

        let sender = String::from("sender");
        let sender_box = Box::new(sender.clone());

        let creator = String::from("creator");
        let creator_box = Box::new(creator.clone());

        match length {
            // null
            0 => {
                Ok(GasLeft::NeedsReturn(U256::from(0), &[]))
            }
            // creator or sender
            1 => {
                let group = user_groups.get_bytes::<String>(ext, 0).unwrap();

                if sender_box.eq(&group) {
                    let ret: &'static str = "send";
                    return Ok(GasLeft::NeedsReturn(U256::from(0), ret.as_bytes()));
                }

                if creator_box.eq(&group) {
                    let ret: &'static str = "create";
                    return Ok(GasLeft::NeedsReturn(U256::from(0), ret.as_bytes()));
                }

                Ok(GasLeft::NeedsReturn(U256::from(0), &[]))
            }
            // creator and sender
            2 => {
                let creator_sender: &'static str = "create send";
                Ok(GasLeft::NeedsReturn(U256::from(0), creator_sender.as_bytes()))
            }
            // error
            _ => {
                Err(Error::Internal("length should not bigger than two".to_string()))
            }
        }
    }


    // check the permission: send/create
    fn check_permission(user: &String, permission: &String, ext: &mut Ext) -> bool {
        let sender = "sender".to_string();
        let creator = "creator".to_string();
        let send = "send".to_string();
        let create = "create".to_string();
        if permission.eq(&send) {
            return ZcPermission::check_user_in_group(user, &sender, ext);
        } else if permission.eq(&create) {
            return ZcPermission::check_user_in_group(user, &creator, ext);
        }
        false
        // match permission {
        //     send => ZcPermission::check_user_in_group(user, &sender, ext),
        //     create => ZcPermission::check_user_in_group(user, &creator, ext),
        // }
    }

    // check user in group
    // use groups
    fn check_user_in_group(user: &String, group: &String, ext: &mut Ext) -> bool {

        let mut map = Map::new(H256::from(2));
        let user_groups = map.get_array(user.clone()).unwrap();
        let length = user_groups.get_len(ext).unwrap();
        let group_box = Box::new(group.clone());

        for len in 0..length {
            let gr = user_groups.get_bytes::<String>(ext, len).unwrap();
            if group_box.eq(&gr) {
                return true;
            }
        }
        false
    }

    // add user to the group: sender/creator
    fn into_group(permission: &[u8], user: String, ext: &mut Ext) -> bool {
        match str::from_utf8(permission).unwrap() {
            "send" => {
                let sender = Array::new(H256::from(1));
                let length = sender.get_len(ext).unwrap();
                let _ = sender.set_bytes(ext, length, user.clone());
                true
            },
            "create" => {
                let creator = Array::new(H256::from(0));
                let length = creator.get_len(ext).unwrap();
                let _ = creator.set_bytes(ext, length, user.clone());
                true
            },
            _ => false,
        }
    }

    // delete user of the group
    fn out_group(permission: &[u8], user: String, ext: &mut Ext) -> bool {
         match str::from_utf8(permission).unwrap() {
            "send" => {
                let sender = Array::new(H256::from(1));
                let length = sender.get_len(ext).unwrap();
                let user_box = Box::new(user);
                let mut len = 0u64;
                for l in 0..length {
                    let u = sender.get_bytes::<String>(ext, len).unwrap();
                    if user_box.eq(&u) {
                        len = l;
                    }
                }

                let _ = sender.set_bytes(ext, length, String::default());
                true
            },
            "create" => {
                let creator = Array::new(H256::from(0));
                let length = creator.get_len(ext).unwrap();
                let user_box = Box::new(user);
                let mut len = 0u64;
                for l in 0..length {
                    let u = creator.get_bytes::<String>(ext, len).unwrap();
                    if user_box.eq(&u) {
                        len = l;
                    }
                }

                let _ = creator.set_bytes(ext, length, String::default());
                true
            },
            _ => false,
        }
    }
        
    // quit the group
    // interface: quit_group(group: &String)
    // group: "sender"/"creator"
    // pub fn quit_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
    //     let sender = params.sender.to_hex();
    //     if let Some(ref data) = params.data {
    //         // sender or creator
    //         let group_data = data.get(4..36).unwrap();
    //         let param_group = String::from_utf8(group_data.to_vec()).unwrap();

    //         if !(ZcPermission::check_user_in_group(&sender, &param_group, ext)) {
    //             return Err(Error::Internal("The sender has no permission".to_string()));
    //         }

    //         let mut groups_map = Map::new(H256::from(2));
    //         let groups_array = groups_map.get_array(sender).unwrap();
    //         let groups_length = groups_array.get_len(ext).unwrap();

    //         let send = String::from("sender");
    //         let create = String::from("creator");

    //         if groups_length == 1 {
    //             let _ = groups_array.set_len(ext, 0u64);
    //         } else if groups_length == 2 {
    //             let _ = groups_array.set_len(ext, 1u64);
    //             if create.eq(&param_group) {
    //                 let _ = groups_array.set_bytes::<String>(ext, groups_length, send.clone());
    //             } else {
    //                 let _ = groups_array.set_bytes::<String>(ext, groups_length, create.clone());
    //             }
    //         }
    //     } else {
    //         return Err(Error::Internal("wrong data".to_string()));
    //     }
    //     Ok(GasLeft::Known(U256::from(0)))
    // }
}
