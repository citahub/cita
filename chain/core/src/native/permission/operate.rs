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
pub fn intersection(one: &Vec<String>, other: &Vec<String>) -> Vec<String> {

    vec![]
}

// diff of two string vector and return a new string vector
pub fn diff(one: &Vec<String>, other: &Vec<String>) -> Vec<String> {

    vec![]
}
