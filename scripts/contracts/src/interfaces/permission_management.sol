pragma solidity ^0.4.24;

/// @title The interface of permission management
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IPermissionManagement {
    function newPermission(bytes32 _name, address[] _conts, bytes4[] _funcs) external returns (address);

    function deletePermission(address _permission) external returns (bool);

    function updatePermissionName(address _permission, bytes32 _name) external returns (bool);

    function addResources(address _permission, address[] _conts, bytes4[] _funcs) external returns (bool);

    function deleteResources(address _permission, address[] _conts, bytes4[] _funcs) external returns (bool);

    function setAuthorizations(address _account, address[] _permissions) external returns (bool);

    function setAuthorization(address _account, address _permission) external returns (bool);

    function cancelAuthorizations(address _account, address[] _permissions) external returns (bool);

    function cancelAuthorization(address _account, address _permission) external returns (bool);

    function clearAuthorization(address _account) external returns (bool);
}
