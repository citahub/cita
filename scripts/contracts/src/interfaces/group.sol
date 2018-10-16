pragma solidity ^0.4.24;

/// @title The interface of group
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IGroup {
    function queryName() external returns (bytes32);

    function queryAccounts() external returns (address[]);

    function queryChild() external returns (address[]);

    function queryChildLength() external returns (uint);

    function queryParent() external returns (address);

    function inGroup(address _account) external returns (bool);
}
