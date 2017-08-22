use super::switch::Switch;
use super::operate::{Operate, check, union, diff};
use super::action::ElementAction;


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

impl Operate for Group{
    fn modify_element(&mut self, element: &Vec<String>, action: ElementAction) {
        // check the permission
        check(&self.name, &"update_group".to_string());
        match action {
            ElementAction::Add => self.add_user(element),
            ElementAction::Delete => self.delete_user(element),
        }
    }
}


impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name: name,
            users: vec![],
            groups: vec![],
            switch: Switch::Off,
        }
    }

    pub fn set_switch(&mut self, switch: Switch) {
        self.switch = switch;
    }

    fn add_user(&mut self, element: &Vec<String>) {
        self.users = union(&self.users, element);
    }

    fn delete_user(&mut self, element: &Vec<String>) {
        self.users = diff(&self.users, element);
    }

//    pub fn add_user(&mut self, user: String) {
//        self.users.push(user);
//    }
//
//    pub fn delete_user(&mut self, user: String) -> Option<String> {
//        self.users.remove_item(&user)
//    }

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
