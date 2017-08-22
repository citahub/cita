use super::action::ElementAction;
pub trait Operate {
    // fn new<T>(&self, name: &string, object: T, action: Option<SetAction>) -> T;
    // fn modify_name(&self, name: &string) -> Option<T>;
    fn modify_element(&self, element: &Vec<String>, action: &ElementAction);
    // fn delete(&self) -> Option<T>;
    // fn query(&self) -> Vec<string>;
}
