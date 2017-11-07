pragma solidity ^0.4.14;

import "./set_operate.sol";
import "./authorization.sol";

/// @notice TODO. Only from router's address
/// @title Manager the role 
contract RoleManager {

    mapping(bytes32 => Role) roles;

    struct Role {
        bytes32 name;
        bytes32[] permissions;
    }

    /// @dev New a role
    /// @param _name The role name of the caller
    /// @return The new role name
    function newRole(
        bytes32 _name,
        bytes32 _newName,
        bytes32[] _newPermission,
        SetOperate.SetOp _op
    )
        public
        returns (bytes32);

    /// @dev Modify the name
    function modifyName(bytes32 _oldName, bytes32 _newName) public returns (bool);

    /// @dev Add permissions 
    function addPermissions(bytes32 _name, bytes32[] _permissions) public returns (bool);

    /// @dev Delete permissions 
    function deletePermissions(bytes32 _name, bytes32[] _permissions) public returns (bool);

    /// @dev Delete permissions
    function deleteRole(bytes32 _role) public returns (bool);

    /// @dev Query the permissions 
    function queryPermissions(bytes32 _name) public returns (bool);
}
