pragma solidity ^0.4.14;

contract ChainManager {

    // Id of self chain. Must greater than 0.
    uint chainId;

    // Id of the parent chain. 0 means no parent chain.
    uint parentChainId;

    // Nodes of the parent chain.
    address[] parentChainNodes;

    // Count of side chain.
    uint sideChainCount;

    // Stores `ChainInfo` struct for each chain.
    mapping(uint => ChainInfo) public sideChains;

    // Default: Unknown
    enum ChainStatus { Unknown, Disable, Enable }

    struct ChainInfo {
        ChainStatus status;
        address[] nodes;
    }

    modifier hasParentChain {
        require(parentChainId != 0);
        _;
    }

    modifier hasSideChain(uint _id) {
        require(sideChains[_id].status != ChainStatus.Unknown);
        _;
    }

    // Constructor.
    function ChainManager(uint _id, uint _pid, address[] _addrs)
        public
    {
        require(_id > 0);
        if (_pid == 0) {
            require(_addrs.length == 0);
        } else {
            require(_addrs.length > 0);
            parentChainId = _pid;
            parentChainNodes = _addrs;
        }
        chainId = _id;
    }

    function getChainId()
        public
        constant
        returns (uint)
    {
        return chainId;
    }

    function getParentChainId()
        public
        constant
        hasParentChain
        returns (uint)
    {
        return parentChainId;
    }

    // Register a new side chain.
    function newSideChain(address[] addrs)
        public
        returns (uint)
    {
        require(addrs.length > 0);
        sideChainCount++;
        uint sideChainId = chainId + sideChainCount;
        sideChains[sideChainId] = ChainInfo(ChainStatus.Disable, addrs);
        // TODO A sorted array can search data more fast.
        //      And we can remove duplicated data, simply.
        return sideChainId;
    }

    function enableSideChain(uint id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Enable;
    }

    function disableSideChain(uint id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Disable;
    }

    function getAuthorities(uint id)
        public
        constant
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
