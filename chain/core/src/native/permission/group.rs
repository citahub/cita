use super::switch::Switch;

#[derive(Clone, Debug)]
pub struct Group {
    // the Group name
    name: String,
    // the users
    users: Vec<String>,
    // the sub groups
    groups: Vec<String>,
    // default: off
    switch: Switch,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name: name,
            users: vec![],
            groups: vec![],
            switch: Switch::OFF,
        }
    }

    pub fn set_switch(&mut self, switch: Switch) {
        self.switch = switch;
    }

    pub fn add_user(&mut self, user: String) {
        self.users.push(user);
    }

    pub fn delete_user(&mut self, user: String) -> Option<String> {
        self.users.remove_item(&user)
    }

    pub fn is_exist_user(&self, user: &String) -> bool {
        for val in &self.users {
            if val == user {
                return true;
            }
        }
        return false;
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}
