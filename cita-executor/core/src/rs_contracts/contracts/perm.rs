use cita_types::Address;
use std::collections::HashSet;

pub type FuncSig = [u8; 4];

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Permission {
    name: String,
    resources: HashSet<Resource>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    addr: Address,
    func: Vec<u8>,
}

impl Permission {
    pub fn new(name: String, contracts: Vec<Address>, funcs: Vec<Vec<u8>>) -> Self {
        let mut perm = Permission::default();
        trace!("Permission name in new is {:?}", name);
        perm.name = name;
        for i in 0..contracts.len() {
            let resource = Resource {
                addr: contracts[i],
                func: funcs[i].clone(),
            };
            perm.resources.insert(resource);
        }
        perm
    }

    pub fn add_resources(&mut self, contracts: Vec<Address>, funcs: Vec<Vec<u8>>) {
        for i in 0..contracts.len() {
            let resource = Resource {
                addr: contracts[i],
                func: funcs[i].clone(),
            };
            self.resources.insert(resource);
        }
    }

    pub fn delete_resources(&mut self, contracts: Vec<Address>, funcs: Vec<Vec<u8>>) {
        for i in 0..contracts.len() {
            let resource = Resource {
                addr: contracts[i],
                func: funcs[i].clone(),
            };
            self.resources.remove(&resource);
        }
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn in_permission(&self, cont: Address, func: Vec<u8>) -> bool {
        // let cont_addr = Address::from(&params.input[16..36]);
        // let func = &params.input[36..40];
        let resource = Resource {
            addr: cont,
            func: func,
        };
        self.resources.contains(&resource)
    }

    pub fn query_name(&self) -> String {
        trace!("Permission name in query name is {:?}", self.name);
        self.name.clone()
    }

    pub fn query_resource(&self) -> (Vec<Address>, Vec<Vec<u8>>) {
        let mut conts = Vec::new();
        let mut funcs = Vec::new();

        for r in self.resources.iter() {
            conts.push(r.addr.clone());
            funcs.push(r.func.clone());
        }

        (conts, funcs)
    }
}
