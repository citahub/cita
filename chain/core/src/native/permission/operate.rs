use super::action::ElementAction;
pub trait Operate {
    // fn new<T>(&self, name: &string, object: T, action: Option<SetAction>) -> T;
    // fn modify_name(&self, name: &string) -> Option<T>;
    fn modify_element(&mut self, element: &Vec<String>, action: ElementAction);
    // fn delete(&self) -> Option<T>;
    // fn query(&self) -> Vec<string>;
}

// TODO
pub fn check(group: &String, permission: &String) -> bool {

    true
}

// union of two string vector and return a new string vector
pub fn union(one: &Vec<String>, other: &Vec<String>) -> Vec<String> {

    vec![]
}

// intersection of two string vector and return a new string vector
pub fn intersection(one: &Vec<String>, other: &Vec<String>) -> Vec<String>{

    vec![]
}

// diff of two string vector and return a new string vector
pub fn diff(one: &Vec<String>, other: &Vec<String>) -> Vec<String>{

    vec![]
}
