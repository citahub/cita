pragma solidity ^0.4.14;

import "./group_manager.sol";
import "./role_manager.sol";
import "./authorization_manager.sol";

/// @notice TODO. Multiple routers
/// @title Permission system including authentication(modifier)
contract PermissionSystem {

    using GroupManager for *;
    using RoleManager for *;
    using AuthorizationManager for *;

    /// @dev Only from permission system contract
    modifier onlyFromPermissionSystem {
        _; 
    }

    /// @dev User in group
    modifier userInGroup(address _user, bytes32 _group) {
        _; 
    }

    /// @dev Group has permission
    modifier groupHasPermission(bytes32 _permission, bytes32 _group) {
        _; 
    }

    /// @dev Permission in role
    modifier permissionInRole(bytes32 _permission, bytes32 _role) {
        _; 
    }

    /// @dev Group has role
    modifier groupHasRole(bytes32 _group, bytes32 _role) {
        _; 
    }

    /// @dev Scope switch is on
    modifier switchOn(bytes32 _group) {
        _; 
    }

    /// @dev Resource group in zone of the user group
    /// @notice For scope
    modifier resourceInZone(bytes32 _resourceGroup, bytes32 _userGroup) {
        _; 
    }

    /// @dev Element in set
    modifier elementInSet(bytes32 _elem, bytes32[] _set) {
        _; 
    }

    /// @dev Check user group has the permission with the scope of the resource group
    function check(bytes32 _userGroup, bytes32 _resourceGroup, bytes32 _permission) public returns (bool);

    /// @dev Apply to into the group
    function applyGroup(bytes32 _group) public returns (bool);

    /// @dev Verify the application
    function verifyGroup(address _user, bytes32 _group) public returns (bool);

    /// @dev Query the roles of the group
    function queryRole(bytes32 _group) public returns (bytes32[]);

    /// @dev Query the the groups of the user
    /// @notice Use msg.sender
    function queryGroup() public returns (bytes32[]);

    /// @dev Grant the role to a user
    function grantRole(bytes32 _group, bytes32 _role, address _user) public returns (bool);

    /// @dev Revoke the user's role
    function revokeRole(bytes32 _group, bytes32 _role, address _user) public returns (bool);

    /// @dev Quit the group
    function quitGroup(bytes32 _group) public returns (bool);

}
