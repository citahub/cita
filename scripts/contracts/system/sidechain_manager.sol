pragma solidity ^0.4.18;

contract SidechainManager {

    // Id of main chain.
    uint constant mainChain = 0;

    // Number of side chain id
    uint numChainId;

    // This stores a `Sidechain` struct for each side chain id
    mapping(uint => Sidechain) public sidechains;

    struct Sidechain {
        address[] nodes;
        // The status of side chain. True: enable, False: disable
        bool status;
    }


    function newChain(address[] nodes) public returns (uint chainId) {
        numChainId++;
        sidechains[numChainId] = Sidechain(nodes, false);
        return numChainId;
    }

    function enableChain(uint id) public {
        Sidechain storage sc = sidechains[id];
        sc.status = true;
    }

    function disableChain(uint id) public {
        Sidechain storage sc = sidechains[id];
        sc.status = false;
    }

    function getStatus(uint id) public view returns (bool) {
        Sidechain storage sc = sidechains[id];
        return sc.status;
    }

    function getNodes(uint id) public view returns (address[]) {
        Sidechain storage sc = sidechains[id];
        return sc.nodes;
    }
}
