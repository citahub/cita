pragma solidity ^0.4.24;

import "../common/error.sol";


/// @title Chain Manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract ChainManager is Error {

    address sysConfigAddr = 0xFFfffFFfFfFffFFfFFfffFffFfFFFffFFf020000;
    address crossChainVerifyAddr = 0xffFfffFfFFFfFFfffFFFffffFfFfffFfFF030002;

    // Id of the parent chain. 0 means no parent chain.
    uint32 parentChainId;

    // Nodes of the parent chain.
    address[] parentChainNodes;

    // Stores `ChainInfo` struct for each chain.
    mapping(uint32 => ChainInfo) public sideChains;

    // Default: Unknown
    enum ChainStatus { Unknown, Disable, Enable }

    struct ChainInfo {
        ChainStatus status;
        address[] nodes;
    }

    modifier hasParentChain {
        if (parentChainId != 0)
            _;
        else {
            emit ErrorLog(ErrorType.NoParentChain, "has no parent chain");
            return;
        }
    }

    modifier hasSideChain(uint32 _id) {
        if (sideChains[_id].status != ChainStatus.Unknown)
            _;
        else {
            emit ErrorLog(ErrorType.NoSideChain, "has no side chain");
            return;
        }
    }

    // Constructor.
    constructor(uint32 _pid, address[] _addrs)
        public
    {
        if (_pid == 0) {
            require(_addrs.length == 0, "The length should be zero.");
        } else {
            require(_addrs.length > 0, "The length should larger than zero.");
            parentChainId = _pid;
            parentChainNodes = _addrs;
        }
    }

    function getChainId()
        public
        returns (uint32)
    {
        address contractAddr = sysConfigAddr;
        bytes4 getChainIdHash = bytes4(keccak256("getChainId()"));

        uint256 result;
        uint256 cid;

        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, getChainIdHash)
            result := call(10000, contractAddr, 0, ptr, 0x4, ptr, 0x20)
            if eq(result, 0) { revert(ptr, 0) }
            cid := mload(ptr)
        }
        return uint32(cid);
    }

    // @notice Register a new side chain.
    function newSideChain(uint32 sideChainId, address[] addrs)
        public
    {
        require(addrs.length > 0, "The length should larger than zero.");
        uint32 myChainId = getChainId();
        require(myChainId != sideChainId, "ChainId should not equal to sideChainId.");
        require(sideChains[sideChainId].status == ChainStatus.Unknown, "ChainStatus not the same witch sideChainStatus.");
        sideChains[sideChainId] = ChainInfo(ChainStatus.Disable, addrs);
        // TODO A sorted array can search data more fast.
        //      And we can remove duplicated data, simply.
    }

    function enableSideChain(uint32 id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Enable;
    }

    function disableSideChain(uint32 id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Disable;
    }

    function verifyBlockHeader(
        uint32 chainId,
        bytes blockHeader
    )
        public
        hasSideChain(chainId)
    {
        address contractAddr = crossChainVerifyAddr;
        bytes4 funcSig = bytes4(keccak256("verifyBlockHeader(uint32,bytes)"));
        bool verifyResult;
        uint blockHeaderSize = 0x20 + blockHeader.length / 0x20 * 0x20;
        if (blockHeader.length % 0x20 != 0) {
            blockHeaderSize += 0x20;
        }
        // bool
        uint outSize = 0x20;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, funcSig)
            mstore(add(ptr, 0x04), chainId)
            mstore(add(ptr, 0x24), 0x40)
            let ptrL := add(ptr, 0x44)
            for {
                    let blockHeaderL := blockHeader
                    let blockHeaderR := add(blockHeader, blockHeaderSize)
                }
                lt(blockHeaderL, blockHeaderR)
                {
                    blockHeaderL := add(blockHeaderL, 0x20)
                    ptrL := add(ptrL, 0x20)
                }
                {
                mstore(ptrL, mload(blockHeaderL))
            }
            let inSize := sub(ptrL, ptr)
            let result := call(100000, contractAddr, 0, ptr, inSize, ptr, outSize)
            if eq(result, 0) { revert(ptr, 0) }
            verifyResult := mload(ptr)
        }
        require(verifyResult == true, "The verifyResult should be true.");
    }

    function getExpectedBlockNumber(
        uint32 chainId
    )
        public
        hasSideChain(chainId)
        returns (uint64)
    {
        address contractAddr = crossChainVerifyAddr;
        bytes4 funcSig = bytes4(keccak256("getExpectedBlockNumber(uint32)"));
        uint256 blockNumber;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, funcSig)
            mstore(add(ptr, 0x04), chainId)
            let result := call(20000, contractAddr, 0, ptr, 0x24, ptr, 0x20)
            if eq(result, 0) { revert(ptr, 0) }
            blockNumber := mload(ptr)
        }
        return uint32(blockNumber);
    }

    function verifyState(
        uint32 chainId,
        uint64 blockNumber,
        bytes stateProof
    )
        public
        hasSideChain(chainId)
        returns (address, uint, uint)
    {
        address contractAddr = crossChainVerifyAddr;
        bytes4 funcSig = bytes4(keccak256("verifyState(uint32,uint64,bytes)"));
        uint stateProofSize = 0x20 + stateProof.length / 0x20 * 0x20;
        if (stateProof.length % 0x20 != 0) {
            stateProofSize += 0x20;
        }
        // address, key, value
        address addr;
        uint key;
        uint value;
        uint outSize = 0x60;
        // solium-disable-next-line security/no-inline-assembly
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, funcSig)
            mstore(add(ptr, 0x04), chainId)
            mstore(add(ptr, 0x24), blockNumber)
            mstore(add(ptr, 0x44), 0x60)
            let ptrL := add(ptr, 0x64)
            for {
                    let stateProofL := stateProof
                    let stateProofR := add(stateProof, stateProofSize)
                }
                lt(stateProofL, stateProofR)
                {
                    stateProofL := add(stateProofL, 0x20)
                    ptrL := add(ptrL, 0x20)
                }
                {
                mstore(ptrL, mload(stateProofL))
            }
            let inSize := sub(ptrL, ptr)
            let result := call(100000, contractAddr, 0, ptr, inSize, ptr, outSize)
            if eq(result, 0) { revert(ptr, 0) }
            addr := mload(ptr)
            key := mload(add(ptr, 0x20))
            value := mload(add(ptr, 0x40))
        }
        return (addr, key, value);
    }

    function getParentChainId()
        public
        view
        returns (uint32)
    {
        return parentChainId;
    }

    function getAuthorities(uint32 id)
        public
        view
        returns (address[])
    {
        // Is it the parent chain?
        if (parentChainId != 0 && parentChainId == id) {
            return parentChainNodes;
        // Is it a enabled side chain?
        } else if (sideChains[id].status == ChainStatus.Enable) {
            return sideChains[id].nodes;
        } else {
            // Returns an empty array;
            return ;
        }
    }
}
