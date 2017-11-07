pragma solidity ^0.4.14;

import "./authentication.sol";

/// @notice TODO. Use modifier
/// @title Used by user
contract Router {

    /// @dev Apply to into the group
    function applyGroup(bytes32 _group) public returns (bool);

    /// @dev Verify the application
    function verifyGroup(address _user, bytes32 _group) public returns (bool);

    /// @dev Query the role of the group
    function queryRole(bytes32 _group) public returns (bytes32);

    /// @dev Query the the group of the user
    /// @notice Use msg.sender
    function queryGroup() public returns (bytes32);

    /// @dev Grant the role to a user
    function grantRole(bytes32 _group, bytes32 _role, address _user) public returns (bool);

    /// @dev Revoke the user's role
    function revokeRole(bytes32 _group, bytes32 _role, address _user) public returns (bool);

    /// @dev Quit the group
    function quitGroup(bytes32 _group) public returns (bool);
}
