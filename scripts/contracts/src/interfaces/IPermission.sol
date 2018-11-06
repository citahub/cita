pragma solidity ^0.4.24;

/// @title The interface of permission
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IPermission {
    /// @notice only permission management
    function addResources(address[] _conts, bytes4[] _funcs) external returns (bool);

    /// @notice only permission management
    function deleteResources(address[] _conts, bytes4[] _funcs) external returns (bool);

    /// @notice only permission management
    function updateName(bytes32 _name) external returns (bool);

    /// @notice only permission management
    function close() external;

    function inPermission(address cont, bytes4 func) external returns (bool);

    function queryInfo() external returns (bytes32, address[], bytes4[]);

    function queryName() external returns (bytes32);

    function queryResource() external returns (address[], bytes4[]);
}
