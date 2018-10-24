pragma solidity ^0.4.24;

/// @title The interface of chain management
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO chain manager's assembly
interface IChainManager {
    function getChainId() external returns (uint);

    function newSideChain(uint sideChainId, address[] addrs) external;

    function enableSideChain(uint id) external;

    function disableSideChain(uint id) external;

    function verifyBlockHeader(uint chainId, bytes blockHeader) external;

    function getExpectedBlockNumber(uint chainId) external returns (uint64);

    function verifyState(
        uint chainId,
        uint64 blockNumber,
        bytes stateProof
    ) external returns (address, uint, uint);

    function getParentChainId() external returns (uint);

    function getAuthorities(uint id) external returns (address[]);
}
