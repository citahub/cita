pragma solidity ^0.4.24;

/// @title The interface of chain management
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO chain manager's assembly
interface IChainManager {
    function getChainId() external returns (uint32);

    function newSideChain(uint32 sideChainId, address[] addrs) external;

    function enableSideChain(uint32 id) external;

    function disableSideChain(uint32 id) external;

    function verifyBlockHeader(uint32 chainId, bytes blockHeader) external;

    function getExpectedBlockNumber(uint32 chainId) external returns (uint64);

    function verifyState(
        uint32 chainId,
        uint64 blockNumber,
        bytes stateProof
    ) external returns (address, uint, uint);

    function getParentChainId() external returns (uint32);

    function getAuthorities(uint32 id) external returns (address[]);
}
