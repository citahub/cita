struct Group {
    // the role name
    name: string,
    // the users
    users: Vec<string>,
    // the groups
    groups: Vec<string>,
    // default: off
    switch: Switch::off,
}

enum Switch {
    on,
    off,
}

struct Role {
    // the role name
    name: string,
    // the permissions
    permissions: Vec<string>,
}

trait utils {
    // fn new<T>(&self, name: &string, object: T, action: Option<SetAction>) -> T;
    // fn modify_name(&self, name: &string) -> Option<T>;
    fn modify_element(&self, element: &Vec<string>, action: &ElementAction);
    // fn delete(&self) -> Option<T>;
    // fn query(&self) -> Vec<string>;
}

impl utils for Group {
    // fn new(&self, name: &string, group: &Group, action: Option<SetAction>) -> Group {}
    // fn modify_name(&self, name: &string) -> Option<T> {}
    fn modify_element(&self, users: &Vec<string>, action: &ElementAction) -> Option<T> {
        // check the permission
        check(user, 'update_group');
        match action {
            ElementAction::add => add_user(&self, element),
            ElementAction::delete => delete_user(&self, element),
        }
    }
    // fn delete(&self) -> Option<T> {}
    // fn query(&self) -> Vec<string> {}
}
