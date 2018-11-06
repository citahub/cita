pragma solidity ^0.4.24;

/// @title The interface of version manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IVersionManager {
    function setVersion(uint32 _version) external;
}
