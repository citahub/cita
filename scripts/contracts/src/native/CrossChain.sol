pragma solidity ^0.4.24;

// TODO
// If solidity support return variable length data in cross-contract calls,
// we do NOT need to write this assembly codes, and we can change this
// contract to a interface.
// All native contract can wrap to interfaces, to make them easier to use.
//
// Ref: https://github.com/ethereum/solidity/issues/2708#issuecomment-320957367
contract CrossChain {

    event SendCrossChain(uint32 fromChainId, uint32 toChainId, address destContract, bytes4 destFuncSig, uint64 sendNonce);
    event RecvCrossChain(address indexed sender, bytes txData);

    address crossChainVerifyAddr = 0xffFfffFfFFFfFFfffFFFffffFfFfffFfFF030002;
    address chainManagerAddr = 0xffFFFfffFFfFFfFFFFffFFFfFfFFffFFfF020002;

    uint64 crossChainSendNonce;
    uint64 crossChainRecvNonce;

    function getCrossChainSendNonce() public view returns (uint64) {
        return crossChainSendNonce;
    }

    function getCrossChainRecvNonce() public view returns (uint64) {
        return crossChainRecvNonce;
    }

    function getFromChainId() public view returns (uint32) {
        address contractAddr = chainManagerAddr;
        bytes4 funcSig = bytes4(keccak256("getChainId()"));
        uint256 chainId;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, funcSig)
            let result := call(20000, contractAddr, 0, ptr, 0x4, ptr, 0x20)
            if eq(result, 0) { revert(ptr, 0) }
            chainId := mload(ptr)
        }
        return uint32(chainId);
    }

    function sendTransaction(
        uint32 toChainId,
        address destContract,
        bytes4 destFuncSig
    ) internal {
        uint32 fromChainId = getFromChainId();
        emit SendCrossChain(fromChainId, toChainId, destContract, destFuncSig, crossChainSendNonce);
        crossChainSendNonce += 1;
    }

    function verifyTransaction(
        bytes memory txProof,
        uint256 txDataSize
    ) internal returns (
        address sender,
        bytes memory txData
    ) {
        address recvContAddr = address(this);
        bytes4 recvFuncSig;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            calldatacopy(ptr, 0x0, 0x4)
            recvFuncSig := mload(ptr)
        }
        address contractAddr = crossChainVerifyAddr;
        bytes4 nativeFunc = bytes4(keccak256("verifyTransaction(address,bytes4,uint64,bytes)"));
        uint64 recvNonce = crossChainRecvNonce;
        // bytes len + bytes
        uint txProofSize = 0x20 + txProof.length / 0x20 * 0x20;
        if (txProof.length % 0x20 != 0) {
            txProofSize += 0x20;
        }
        // address + bytes pos + bytes len + bytes
        uint outSize = 0x60 + txDataSize / 0x20 * 0x20;
        if (txDataSize % 0x20 != 0) {
            outSize += 0x20;
        }
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, nativeFunc)
            mstore(add(ptr, 0x04), recvContAddr)
            mstore(add(ptr, 0x24), recvFuncSig)
            mstore(add(ptr, 0x44), recvNonce)
            mstore(add(ptr, 0x64), 0x80)
            // copy txproof bytes
            let ptrL := add(ptr, 0x84)
            for {
                    let txProofL := txProof
                    let txProofR := add(txProof, txProofSize)
                }
                lt(txProofL, txProofR)
                {
                    txProofL := add(txProofL, 0x20)
                    ptrL := add(ptrL, 0x20)
                }
                {
                mstore(ptrL, mload(txProofL))
            }
            let inSize := sub(ptrL, ptr)
            switch call(100000, contractAddr, 0, ptr, inSize, ptr, outSize)
            case 0 { revert(0, 0) }
            default {
                // return(ptr, outSize)
                sender := mload(ptr)
                txData := add(ptr, 0x40)
            }
        }
        emit RecvCrossChain(sender, txData);
        crossChainRecvNonce += 1;
    }
}
