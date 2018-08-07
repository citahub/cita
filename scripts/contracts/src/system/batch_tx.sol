pragma solidity ^0.4.24; 


/// @title Batch tx
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract BatchTx {

    /// @notice Proxy multiple transactions
    ///         The encoded transactions: tuple(address,dataLen,data)
    function multiTxs(bytes)
        external
    {
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            // Ignore the function sig: 0x4
            //        the offset of bytes: 0x20
            //        the len of bytes: 0x20
            let offset := 0x44
            for { } lt(offset, calldatasize) { } {
                let to := calldataload(offset)
                let dataLen := calldataload(add(offset, 0x20))
                let ptr := mload(0x40)
                calldatacopy(ptr, add(offset, 0x40), dataLen)
                switch call(gas, to, 0, ptr, dataLen, 0, 0)
                case 0 { revert(0, 0) }
                // Ignore the excess 0
                offset := add(offset, add(0x40, mul(div(add(dataLen, 0x1f), 0x20), 0x20)))
            }
        }
    }
}
