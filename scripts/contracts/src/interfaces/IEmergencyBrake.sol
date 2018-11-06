pragma solidity ^0.4.24;

/// @title The interface of emergency brake
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IEmergencyBrake {
    function setState(bool _state) external;
}
