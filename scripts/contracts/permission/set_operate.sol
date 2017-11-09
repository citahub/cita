pragma solidity ^0.4.14;

/// @notice TODO. Address and bytes32
library SetOperate {

    enum SetOp { None, Union, Interaction, Diff }

    /// @notice Replace bytes32[] with string or not
    function setOpBytes32(bytes32[] _one, bytes[] _other, SetOp _op) internal returns (bytes32[]) {
        _one;
        _other;
        _op;
    }

    /// @dev Union set of bytes32
    function opUnionBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        _one;
        _other;
    }

    /// @dev Interaction set of bytes32
    function opInteractionBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        _one;
        _other;
    }

    /// @dev Diff set of bytes32
    function opDiffBytes32(bytes32[] _one, bytes32[] _other) internal returns (bytes32[]) {
        _one;
        _other;
    }

    /// @notice Set operation of address
    function setOpAddress(address[] _one, address[] _other, SetOp _op) internal returns (address[]) {
        _one;
        _other;
        _op;
    }

    /// @dev Union set of address
    function opUnionAddress(address[] _one, address[] _other) internal returns (address[]) {
        _one;
        _other;
    }

    /// @dev Interaction set of address
    function opInteractionAddress(address[] _one, address[] _other) internal returns (address[]) {
        _one;
        _other;
    }

    /// @dev Diff set of address
    function opDiffAddress(address[] _one, address[] _other) internal returns (address[]) {
        _one;
        _other;
    }
}
