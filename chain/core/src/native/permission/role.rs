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
use super::operate::{Operate, check, union, diff};

#[derive(Clone, Debug)]
pub struct Role {
    // the role name
    name: String,
    // the permissions
    permissions: Vec<String>,
}

impl Operate for Role {
    fn modify_element(&mut self, element: &Vec<String>, action: ElementAction){
        // check the permission
        check(&self.name, &"update_group".to_string());
        match action {
            ElementAction::Add => self.add_permission(element),
            ElementAction::Delete => self.delete_permission(element),
        }
    }
}

impl Role {
    pub fn add_permission(&mut self, element: &Vec<String>) {
        self.permissions = union(&self.permissions, element);
    }

    pub fn delete_permission(&mut self, element: &Vec<String>) {
        self.permissions = diff(&self.permissions, element);
    }

}