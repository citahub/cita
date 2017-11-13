pragma solidity ^0.4.14;

/// @notice TODO. Only from router's address
/// @title Manager the authorization
contract AuthorizationManager {

    mapping(bytes32 => bytes32[]) role_groups;
    mapping(bytes32 => bytes32[]) group_roles;

    event AuthorizationSetted(bytes32 _group, bytes32 _role);
    event AuthorizationCanceled(bytes32 _group, bytes32 _role);
    event GroupReplaced(bytes32 _oldName, bytes32 _newName);
    event RoleReplaced(bytes32 _oldName, bytes32 _newName);
    event GroupDeleted(bytes32 _name);
    event RoleDeleted(bytes32 _name);

    /// @dev Set authorization
    function setAuthorization(bytes32 _group, bytes32 _role) public returns(bool) {
        role_groups[_role].push(_group);
        group_roles[_group].push(_role);
        AuthorizationSetted(_group, _role);
        return true;
    }

    /// @dev Cancel authorization
    function cancelAuthorization(bytes32 _group, bytes32 _role) public returns(bool) {
        delete role_groups[_role];
        delete group_roles[_group];
        AuthorizationCanceled(_group, _role);
        return true;
    }

    /// @dev Replace the group name
    function replaceGroup(bytes32 _oldName, bytes32 _newName) public returns(bool) {
        bytes32[] memory roles = group_roles[_oldName];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++) {
            var index = bytes32Index(_oldName, role_groups[roles[i]]);

            // Not found
            if (index >= role_groups[roles[i]].length)
                return false;

            role_groups[roles[i]][index] = _newName;
        }

        // Change the group_roles
        group_roles[_newName] = roles;
        delete group_roles[_oldName];
        GroupReplaced(_oldName, _newName);
        return true;
    }

    /// @dev Replace the role name
    function replaceRole(bytes32 _oldName, bytes32 _newName) public returns(bool) {
        bytes32[] memory groups = group_roles[_oldName];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++) {
            var index = bytes32Index(_oldName, group_roles[groups[i]]);

            // Not found
            if (index >= group_roles[groups[i]].length)
                return false;

            group_roles[groups[i]][index] = _newName;
        }

        // Change the role_groups
        role_groups[_newName] = groups;
        delete role_groups[_oldName];
        RoleReplaced(_oldName, _newName);
        return true;
    }

    /// @dev Delete the group
    function deleteGroup(bytes32 _name) public returns (bool) {
        bytes32[] memory roles = group_roles[_name];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++)
            bytes32Delete(_name, role_groups[roles[i]]);

        // Change the group_roles
        group_roles[_name] = roles;
        delete group_roles[_name];
        GroupDeleted(_name);
        return true;
    }

    /// @dev Delete the role
    function deleteRole(bytes32 _name) public returns (bool) {
        bytes32[] memory groups = role_groups[_name];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++)
            bytes32Delete(_name, group_roles[groups[i]]);

        // Change the role_groups
        role_groups[_name] = groups;
        delete role_groups[_name];
        RoleDeleted(_name);
        return true;
    }

    /// @dev Delete the user of the users
    function bytes32Delete(bytes32 _value, bytes32[] storage _array) internal returns (bool) {
        var index = bytes32Index(_value,  _array);
        // Not found
        if (index >= _array.length)
            return false;

        // Remove the gap
        for (uint i = index; i < _array.length - 1; i++) {
            _array[i] = _array[i + 1];
        }

        // Also delete the last element
        delete _array[_array.length - 1];
        _array.length--;
        return true;
    }

    /// @dev Get the index in the nodes_of_start array
    function bytes32Index(bytes32 _value, bytes32[] _array) internal returns (uint) {
        // Find the index of the value in the array
        for (uint i = 0; i < _array.length; i++) {
            if (_value == _array[i])
                return i;
        }
        // If i == length, means not find
        return i;
    }

    /// @dev Query the roles of group
    function queryRoles(bytes32 _group) constant returns(bytes32[]) {
        return group_roles[_group];
    }

    /// @dev Query the groups of role
    function queryGroups(bytes32 _role) constant returns(bytes32[]) {
        return role_groups[_role] ;
    }
}
