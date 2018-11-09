pragma solidity ^0.4.24;

/// @title The interface of group management
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IGroupManagement {
    function newGroup(address _origin, bytes32 _name, address[] _accounts) external returns (address);

    function deleteGroup(address _origin, address _target) external returns (bool);

    function updateGroupName(address _origin, address _target, bytes32 _name) external returns (bool);

    function addAccounts(address _origin, address _target, address[] _accounts) external returns (bool);

    function deleteAccounts(address _origin, address _target, address[] _accounts) external returns (bool);

    function checkScope(address _origin, address _target) external returns (bool);
}
