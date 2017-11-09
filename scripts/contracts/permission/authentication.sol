pragma solidity ^0.4.14;

import "./group_manager.sol";
import "./role_manager.sol";
import "./authorization_manager.sol";

/// @notice TODO. Only from router's address
/// @title Manager the authentication
contract Authentication {

    /// @dev Check the user group has the permission with the scope of the resource group
    function check(bytes32 _userGroup, bytes32 _resourceGroup, bytes32 _permission) public returns (bool);

    /// @dev Check user in group
    function checkUserInGroup(address _user, bytes32 _group) public returns (bool);

    /// @dev Group has permission
    function checkGroupHasPermission(bytes32 _permission, bytes32 _group) public returns (bool);

    /// @dev Check permission in role
    function checkPermissionInRole(bytes32 _permission, bytes32 _role) public returns (bool);

    /// @dev Check group has role
    function checkGroupHasRole(bytes32 _group, bytes32 _role) public returns (bool);

    /// @dev Check scope switch
    function checkSwitch(bytes32 _group) public returns (bool);

    /// @dev Check element in set
    function checkIn(bytes32 _elem, bytes32[] _set) private returns (bool);
}
