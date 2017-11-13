pragma solidity ^0.4.14;

/// @notice TODO. Only from router's address
/// @title Manager the authorization
contract AuthorizationManager {

    mapping(bytes32 => bytes32[]) role_groups;
    mapping(bytes32 => bytes32[]) group_roles;

    /// @dev Set authorization
    function setAuthorization(bytes32 _group, bytes32 _role) public returns(bool) {}

    /// @dev Cancel authorization
    function cancelAuthorization(bytes32 _group, bytes32 _role) public returns(bool) {}

    /// @dev Query the role of group
    function queryRoles(bytes32 _group) public returns(bool) {}

    /// @dev Query the group of role
    function queryGroups(bytes32 _role) public returns(bool) {}

    /// @dev Replace the group name
    function replaceGroup(bytes32 _oldName, bytes32 _newName) public returns(bool) {}

    /// @dev Replace the role name
    function replaceRole(bytes32 _oldName, bytes32 _newName) public returns(bool) {}

    /// @dev Delete the group
    function deleteGroup(bytes32 _name) public returns (bool) {}

    /// @dev Delete the role
    function deleteRole(bytes32 _name) public returns (bool) {}
}
