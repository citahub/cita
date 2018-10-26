pragma solidity ^0.4.24;

/// @title The interface of system config
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface ISysConfig {
    /// @notice Update current chain name
    function setChainName(string) external;

    /// @notice Update current operator
    function setOperator(string) external;

    /// @notice Update current operator's website URL
    function setWebsite(string) external;

    /// @notice Update to chainIdV1
    function updateToChainIdV1() external;

    /// @notice Get delay block number before validate
    function getDelayBlockNumber() external view returns (uint);

    /// @notice Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() external view returns (bool);

    /// @notice Check sender's send transaction permission
    function getSendTxPermissionCheck() external view returns (bool);

    /// @notice Check sender's create contract permission
    function getCreateContractPermissionCheck() external view returns (bool);

    /// @notice Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() external view returns (bool);

    /// @notice Whether check transaction fee back to operation platform or not, true represents back to platform and false represents back to nodes
    function getFeeBackPlatformCheck() external view returns (bool);

    /// @notice The owner of the chain
    function getChainOwner() external view returns (address);

    /// @notice The name of current chain
    function getChainName() external view returns (string);

    /// @notice The id of current chain
    function getChainId() external view returns (uint32);

    /// @notice The operator of current chain
    function getOperator() external view returns (string);

    /// @notice Current operator's website URL
    function getWebsite() external view returns (string);

    /// @notice The interval time for creating a block (milliseconds)
    function getBlockInterval() external view returns (uint64);

    /// @notice The token information
    function getTokenInfo() external view returns (string, string, string);

    function getEconomicalModel() external view returns (uint8);

}
