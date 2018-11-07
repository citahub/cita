pragma solidity ^0.4.24;

/// @title The interface of batch tx
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IBatchTx {
    /// @notice Proxy multiple transactions
    function multiTxs(bytes) external;
}
