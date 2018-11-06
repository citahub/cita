pragma solidity ^0.4.24;

/// @title The interface of role management
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IRoleManagement {
    function newRole(bytes32 _name, address[] _permissions) external returns (address);

    function deleteRole(address _role) external returns (bool);

    function updateRoleName(address _role, bytes32 _name) external returns (bool);

    function addPermissions(address _role, address[] _permissions) external returns (bool);

    function deletePermissions(address _role, address[] _permissions) external returns (bool);

    function setRole(address _account, address _role) external returns (bool);

    function cancelRole(address _account, address _role) external returns (bool);

    function clearRole(address _account) external returns (bool);
}
