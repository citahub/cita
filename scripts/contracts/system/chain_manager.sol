pragma solidity ^0.4.18;

contract ChainManager {

    // Id of self chain. Must greater than 0.
    uint64 chainId;

    // Id of the parent chain. 0 means no parent chain.
    uint64 parentChainId;

    // Nodes of the parent chain.
    address[] parentChainNodes;

    // Count of side chain.
    uint64 sideChainCount;

    // Stores `ChainInfo` struct for each chain.
    mapping(uint64 => ChainInfo) public sideChains;

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

    modifier hasSideChain(uint64 _id) {
        require(sideChains[_id].status != ChainStatus.Unknown);
        _;
    }

    // Constructor.
    function ChainManager(uint64 _id, uint64 _pid, address[] _addrs)
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
        view
        returns (uint64)
    {
        return chainId;
    }

    function getParentChainId()
        public
        view
        hasParentChain
        returns (uint64)
    {
        return parentChainId;
    }

    // Register a new side chain.
    function newSideChain(address[] addrs)
        public
        returns (uint64)
    {
        require(addrs.length > 0);
        sideChainCount++;
        uint64 sideChainId = chainId + sideChainCount;
        sideChains[sideChainId] = ChainInfo(ChainStatus.Disable, addrs);
        // TODO A sorted array can search data more fast.
        //      And we can remove duplicated data, simply.
        return sideChainId;
    }

    function enableSideChain(uint64 id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Enable;
    }

    function disableSideChain(uint64 id)
        public
        hasSideChain(id)
    {
        sideChains[id].status = ChainStatus.Disable;
    }

    function getAuthorities(uint64 id)
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
