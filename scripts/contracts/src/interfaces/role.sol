pragma solidity ^0.4.24;

/// @title The interface of role
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IRole {

    /// @notice only role management
    function deleteRole() external;

    /// @notice only role management
    function updateName(bytes32 _name) external returns (bool);

    /// @notice only role management
    function addPermissions(address[] _permissions) external returns (bool);

    /// @notice only role management
    function deletePermissions(address[] _permissions) external returns (bool);

    function queryRole() external returns (bytes32, address[]);

    function queryName() external returns (bytes32);

    function queryPermissions() external returns (address[]);

    function lengthOfPermissions() external returns (uint);

    function inPermissions(address _permission) external returns (bool);
}
