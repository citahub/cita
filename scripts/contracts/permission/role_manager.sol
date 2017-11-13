pragma solidity ^0.4.14;

import "./set_operate.sol";
import "./authorization_manager.sol";

/// @notice TODO. Only from router's address
/// @title Manager the role 
contract RoleManager {

    using SetOperate for *;

    mapping(bytes32 => Role) roles;

    struct Role {
        bytes32 name;
        bytes32[] permissions;
    }

    event RoleNewed(bytes32 _name);
    event NameModified(bytes32 _oldName, bytes32 _newName);
    event PermissionsAdded(bytes32 _name, bytes32[] _permissions);
    event PermissionsDeleted(bytes32 _name, bytes32[] _permissions);
    event RoleDeleted(bytes32 _name);

    /// @dev New a role
    /// @param _name The role name of the caller
    /// @return The new role name
    function newRole(
        bytes32 _name,
        bytes32 _newName,
        bytes32[] _newPermissions,
        SetOperate.SetOp _op
    )
        public
        returns (bytes32) 
    {
        Role memory role;
        role.name = _newName;

        if (SetOperate.SetOp.None == _op) {
            for (uint i = 0; i < _newPermissions.length; i++)
                role.permissions[i] = _newPermissions[i];
        } else {
            bytes32[] memory one = SetOperate.setOpBytes32(roles[_name].permissions, _newPermissions, _op);
            for (uint j = 0; j < one.length; j++)
                role.permissions[j] = one[j];
        }

        RoleNewed(_newName);
        return role.name;

    }

    /// @dev Modify the name
    function modifyName(bytes32 _oldName, bytes32 _newName) public returns (bool) {
        Role memory role = roles[_oldName];
        role.name = _newName;
        roles[_newName] = role;
        delete roles[_oldName];
        // Also change authorization
        AuthorizationManager auth = new AuthorizationManager(); 
        auth.replaceRole(_oldName, _newName); 
        NameModified(_oldName, _newName);
        return true;

    }

    /// @dev Add permissions 
    function addPermissions(bytes32 _name, bytes32[] _permissions) public returns (bool) {
        bytes32[] memory result = SetOperate.opUnionBytes32(roles[_name].permissions, _permissions);

        for (uint i = 0; i < result.length; i++)
            roles[_name].permissions[i] = result[i];

        PermissionsAdded(_name, _permissions);
        return true;

    }

    /// @dev Delete permissions 
    function deletePermissions(bytes32 _name, bytes32[] _permissions) public returns (bool) {
        bytes32[] memory result = SetOperate.opDiffBytes32(roles[_name].permissions, _permissions);

        for (uint i = 0; i < result.length; i++)
            roles[_name].permissions[i] = result[i];

        PermissionsDeleted(_name, _permissions);
        return true;
 
    }

    /// @dev Delete permissions
    function deleteRole(bytes32 _name) public returns (bool) {
        delete roles[_name];
        // Also delete the authorization
        AuthorizationManager auth = new AuthorizationManager(); 
        auth.deleteRole(_name);
        RoleDeleted(_name);
        return true;
    }

    /// @dev Query the permissions 
    function queryPermissions(bytes32 _name) public returns (bytes32[]) {
        return roles[_name].permissions;
    }
}
