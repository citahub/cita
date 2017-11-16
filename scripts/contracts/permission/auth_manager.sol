pragma solidity ^0.4.14;

import "./util.sol";

/// @title Manager the authorization
library AuthorizationManager {

    using Util for *;

    struct Authorization {
        mapping(bytes32 => bytes32[]) role_groups;
        mapping(bytes32 => bytes32[]) group_roles;
    }

    event AuthorizationSetted(bytes32 _group, bytes32 _role);
    event AuthorizationCanceled(bytes32 _group, bytes32 _role);
    event GroupReplaced(bytes32 _oldName, bytes32 _newName);
    event RoleReplaced(bytes32 _oldName, bytes32 _newName);
    event GroupDeleted(bytes32 _name);
    event RoleDeleted(bytes32 _name);

    /// @dev Set authorization
    function setAuthorization(Authorization storage self, bytes32 _group, bytes32 _role) internal returns(bool) {
        self.role_groups[_role].push(_group);
        self.group_roles[_group].push(_role);
        AuthorizationSetted(_group, _role);
        return true;
    }

    /// @dev Cancel authorization
    function cancelAuthorization(Authorization storage self, bytes32 _group, bytes32 _role) internal returns(bool) {
        delete self.role_groups[_role];
        delete self.group_roles[_group];
        AuthorizationCanceled(_group, _role);
        return true;
    }

    /// @dev Replace the group name
    function replaceGroup(Authorization storage self, bytes32 _oldName, bytes32 _newName) internal returns(bool) {
        bytes32[] memory roles = self.group_roles[_oldName];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++) {
            var index = Util.bytes32Index(_oldName, self.role_groups[roles[i]]);

            // Not found
            if (index >= self.role_groups[roles[i]].length)
                return false;

            self.role_groups[roles[i]][index] = _newName;
        }

        // Change the group_roles
        self.group_roles[_newName] = roles;
        delete self.group_roles[_oldName];
        GroupReplaced(_oldName, _newName);
        return true;
    }

    /// @dev Replace the role name
    function replaceRole(Authorization storage self, bytes32 _oldName, bytes32 _newName) internal returns(bool) {
        bytes32[] memory groups = self.group_roles[_oldName];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++) {
            var index = Util.bytes32Index(_oldName, self.group_roles[groups[i]]);

            // Not found
            if (index >= self.group_roles[groups[i]].length)
                return false;

            self.group_roles[groups[i]][index] = _newName;
        }

        // Change the role_groups
        self.role_groups[_newName] = groups;
        delete self.role_groups[_oldName];
        RoleReplaced(_oldName, _newName);
        return true;
    }

    /// @dev Delete the group
    function deleteGroup(Authorization storage self, bytes32 _name) internal returns (bool) {
        bytes32[] memory roles = self.group_roles[_name];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++)
            Util.bytes32Delete(_name, self.role_groups[roles[i]]);

        // Change the group_roles
        self.group_roles[_name] = roles;
        delete self.group_roles[_name];
        GroupDeleted(_name);
        return true;
    }

    /// @dev Delete the role
    function deleteRole(Authorization storage self, bytes32 _name) internal returns (bool) {
        bytes32[] memory groups = self.role_groups[_name];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++)
            Util.bytes32Delete(_name, self.group_roles[groups[i]]);

        // Change the role_groups
        self.role_groups[_name] = groups;
        delete self.role_groups[_name];
        RoleDeleted(_name);
        return true;
    }
}
