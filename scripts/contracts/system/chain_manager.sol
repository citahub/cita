pragma solidity ^0.4.24;

import "./error.sol";


/// @title Chain Manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract ChainManager is Error {

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
            require(_addrs.length == 0);
        } else {
            require(_addrs.length > 0);
            parentChainId = _pid;
            parentChainNodes = _addrs;
        }
    }

    function getChainId()
        public
        view
        returns (uint32)
    {
        // SysConfig Contract
        address sysConfigAddr = 0x0000000000000000000000000000000031415926;
        // getChainId() function
        bytes4 getChainIdHash = bytes4(keccak256("getChainId()"));

        uint256 result;
        uint256 cid;

        assembly {
            let ptr := mload(0x40)
            mstore(ptr, getChainIdHash)
            result := call(10000, sysConfigAddr, 0, ptr, 0x4, ptr, 0x20)
            if eq(result, 0) { revert(ptr, 0) }
            cid := mload(ptr)
        }
        return uint32(cid);
    }

    function getParentChainId()
        public
        hasParentChain
        returns (uint32)
    {
        return parentChainId;
    }

    // Register a new side chain.
    function newSideChain(uint32 sideChainId, address[] addrs)
        public
    {
        require(addrs.length > 0);
        uint32 myChainId = getChainId();
        require(myChainId != sideChainId);
        require(sideChains[sideChainId].status == ChainStatus.Unknown);
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
        }
    }
}
