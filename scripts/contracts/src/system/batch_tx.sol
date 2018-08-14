pragma solidity ^0.4.24;


/// @title Batch tx
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO use native contract
contract BatchTx {

    /// @notice Proxy multiple transactions
    ///         The encoded transactions data: tuple(address,dataLen,data)
    ///         dataLen: uint32
    ///         address: uint160
    ///         Example:
    ///             address: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    ///             datalen: 00000004
    ///                data: xxxxxxxx
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
                // 0xc bytes forward from the offset(0x20-0x14)
                // Use `and` instruction just for safe
                let to := and(calldataload(sub(offset, 0xc)), 0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff)
                // 0x8 bytes forward from the offset(0x20-0x14-0x4)
                let dataLen := and(calldataload(sub(offset, 0x8)), 0x00000000000000000000000000000000000000000000000000000000ffffffff)
                let ptr := mload(0x40)
                // Jump the address and dataLen(0x14+0x4)
                calldatacopy(ptr, add(offset, 0x18), dataLen)
                switch call(gas, to, 0, ptr, dataLen, ptr, 0)
                case 0 { revert(0, 0) }
                offset := add(add(offset, 0x18), dataLen)
            }
        }
    }
}
