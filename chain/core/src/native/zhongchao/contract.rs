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
use super::storage::{Array, Scalar, Map};
use native::permission::action::ElementAction;
use rustc_hex::ToHex;
use std::str;

pub struct ZcPermission {
    // Key is signature of function in the contract, value is contract function
    functions: HashMap<Signature, Box<Function>>,
    // Group that has 'create' permission
    creator: Vec<String>,
    // Group that has 'send' permission
    sender: Vec<String>,
    // The users who apply to the group owning the create permission
    applicant_of_creator: Vec<String>,
    // The users who apply to the group owning the send permission
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
        contract.functions.insert(2, Box::new(ZcPermission::query_group));
        contract.functions.insert(3, Box::new(ZcPermission::grant_role));
        contract.functions.insert(4, Box::new(ZcPermission::revoke_role));
        contract.functions.insert(5, Box::new(ZcPermission::quit_group));
        contract
    }

    // TODO init
    pub fn init(user_send: &String, user_create: &String, ext: &mut Ext) {

        let send_array = Array::new(H256::from(1));
        let create_array = Array::new(H256::from(0));

        send_array.set_len(ext, 1u64);
        send_array.set_bytes::<String>(ext, 0u64, user_send.clone());
        create_array.set_len(ext, 1u64);
        create_array.set_bytes::<String>(ext, 0u64, user_create.clone());
    }

    // check_user_in(group);
    pub fn check_user_in(user: &String, group: &String, ext: &mut Ext) -> bool {

        let mut map = Map::new(H256::from(4));
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



    // Users apply to join in the group that has some permission, first stored in the middle array
    pub fn into_temp(group: &[u8], user: String, ext: &mut Ext) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                // position 'three' in the contract struct
                let apply_sender = Array::new(H256::from(3));
                let length = apply_sender.get_len(ext).unwrap();
                apply_sender.set_bytes(ext, length, user.clone());
                true
            }
            "creator" => {
                let apply_creator = Array::new(H256::from(2));
                let length = apply_creator.get_len(ext).unwrap();
                apply_creator.set_bytes(ext, length, user.clone());
                true
            }
            _ => false,
        }
    }

    // Users in the group can verify the application
    pub fn into_group(group: &[u8], ext: &mut Ext) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                let send_group = Array::new(H256::from(1));
                let group_length = send_group.get_len(ext).unwrap();

                let apply_sender = Array::new(H256::from(3));
                let apply_length = apply_sender.get_len(ext).unwrap();

                for len in 0..apply_length {
                    let user = *apply_sender.get_bytes::<String>(ext, len).unwrap();
                    send_group.set_bytes::<String>(ext, group_length, user);
                }
                // when verified all the application, set length to zero
                apply_sender.set_len(ext, 0u64);
                true
            }
            "creator" => {
                let create_group = Array::new(H256::from(0));
                let group_length = create_group.get_len(ext).unwrap();

                let apply_creator = Array::new(H256::from(2));
                let apply_length = apply_creator.get_len(ext).unwrap();

                for len in 0..apply_length {
                    let user = *apply_creator.get_bytes::<String>(ext, len).unwrap();
                    create_group.set_bytes::<String>(ext, group_length, user);
                }
                apply_creator.set_len(ext, 0u64);
                true
            }
            _ => false,
        }
    }

    // apply to join the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        let user = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let group = data.get(4..36).unwrap();
            if !(ZcPermission::into_temp(group.clone(), user, ext)) {
                return Err(Error::Internal("fail to enter the application array".to_string()));
            }
        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // verify the application
    // fn verify_group(group: &string)
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let group_data = data.get(4..36).unwrap();
            let group = String::from_utf8(group_data.to_vec()).unwrap();
            if !(ZcPermission::check_user_in(&sender, &group, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }
            if !(ZcPermission::into_group(group_data, ext)) {
                return Err(Error::Internal("fail to enter the application array".to_string()));
            }
        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the permission
    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO add return value to 'ret'
        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            if let Some(ref data) = params.data {
                let group_data = data.get(4..36).unwrap();
                let param_group = String::from_utf8(group_data.to_vec()).unwrap();
                if !(ZcPermission::check_user_in(&sender, &param_group, ext)) {
                    return Err(Error::Internal("The sender has no permission".to_string()));
                }
            } else {
                return Err(Error::Internal("wrong data".to_string()));
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // grant the role to a user
    // fn grant_role(group: &String, role: &Role, user: &String) -> bool
    pub fn grant_role(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            // TODO 判断数据传入不对的情况
            let group_data = data.get(4..36).unwrap();
            let role_data = data.get(36..68).unwrap();
            let user_data = data.get(68..100).unwrap();
            let param_group = String::from_utf8(group_data.to_vec()).unwrap();
            let param_role = String::from_utf8(role_data.to_vec()).unwrap();
            let param_user = String::from_utf8(user_data.to_vec()).unwrap();

            if !(ZcPermission::check_user_in(&sender, &param_group, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }

            let mut groups_map = Map::new(H256::from(5));
            let mut roles_map = Map::new(H256::from(6));
            let groups_array = groups_map.get_array(param_user).unwrap();
            let roles_array = roles_map.get_array(param_group.clone()).unwrap();
            let groups_length = groups_array.get_len(ext).unwrap();
            let roles_length = roles_array.get_len(ext).unwrap();

            let role_box = Box::new(param_role.clone());
            let mut flag1 = false;
            for len in 0..roles_length {
                let role = roles_array.get_bytes::<String>(ext, len).unwrap();
                if role_box.eq(&role) {
                    flag1 = true;
                }
            }
            if !flag1 {
                roles_array.set_len(ext, roles_length + 1);
                roles_array.set_bytes::<String>(ext, roles_length, param_role);
            }

            let group_box = Box::new(param_group.clone());
            let mut flag2 = false;
            for len in 0..groups_length {
                let group = groups_array.get_bytes::<String>(ext, len).unwrap();
                if group_box.eq(&group) {
                    flag2 = true;
                }
            }
            if !flag2 {
                groups_array.set_len(ext, groups_length + 1);
                groups_array.set_bytes::<String>(ext, groups_length, param_group.clone());
            }

        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // revoke the role to a user
    // fn revoke_role(group: &String, role: &Role, user: &String) -> bool;
    pub fn revoke_role(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // role -> group -> user
        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let group_data = data.get(4..36).unwrap();
            let role_data = data.get(36..68).unwrap();
            let user_data = data.get(68..100).unwrap();
            let param_group = String::from_utf8(group_data.to_vec()).unwrap();
            let param_role = String::from_utf8(role_data.to_vec()).unwrap();
            let param_user = String::from_utf8(user_data.to_vec()).unwrap();

            if !(ZcPermission::check_user_in(&sender, &param_group, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }

            let mut groups_map = Map::new(H256::from(5));
            let mut roles_map = Map::new(H256::from(6));
            let groups_array = groups_map.get_array(param_user).unwrap();
            let roles_array = roles_map.get_array(param_group.clone()).unwrap();
            let groups_length = groups_array.get_len(ext).unwrap();
            let roles_length = roles_array.get_len(ext).unwrap();

            let role_box = Box::new(param_role);
            let mut flag1 = false;
            let mut length1 = 0u64;
            for len in 0..roles_length {
                let role = roles_array.get_bytes::<String>(ext, len).unwrap();
                if role_box.eq(&role) {
                    flag1 = true;
                    length1 = len;
                }
            }
            if !flag1 {
                return Err(Error::Internal("The group doesn't has the role".to_string()));
            }
            // TODO: add del operation of array. Now the length is unchange
            roles_array.set_bytes::<String>(ext, length1, String::default());

            let group_box = Box::new(param_group.clone());
            let mut flag2 = false;
            let mut length2 = 0u64;
            for len in 0..groups_length {
                let group = groups_array.get_bytes::<String>(ext, len).unwrap();
                if group_box.eq(&group) {
                    flag2 = true;
                    length2 = len;
                }
            }
            if !flag2 {
                return Err(Error::Internal("The group doesn't has the role".to_string()));
            }
            groups_array.set_bytes(ext, length2, String::default());
        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // quit the group
    // fn quit_group(group: &String) -> bool;
    pub fn quit_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {

        let sender = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let group_data = data.get(4..36).unwrap();
            let param_group = String::from_utf8(group_data.to_vec()).unwrap();

            if !(ZcPermission::check_user_in(&sender, &param_group, ext)) {
                return Err(Error::Internal("The sender has no permission".to_string()));
            }

            let mut groups_map = Map::new(H256::from(5));
            let groups_array = groups_map.get_array(sender).unwrap();
            let groups_length = groups_array.get_len(ext).unwrap();

            let group_box = Box::new(param_group);
            let mut length = 0u64;
            for len in 0..groups_length {
                let group = groups_array.get_bytes::<String>(ext, len).unwrap();
                if group_box.eq(&group) {
                    length = len;
                }
            }
            groups_array.set_bytes(ext, length, String::default());

        } else {
            return Err(Error::Internal("wrong data".to_string()));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }
}
