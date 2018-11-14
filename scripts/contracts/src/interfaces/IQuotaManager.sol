pragma solidity ^0.4.24;

/// @title The interface of quota_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface IQuotaManager {

    event DefaultAqlSetted(uint indexed _value, address indexed _sender);
    event BqlSetted(uint indexed _value, address indexed _sender);
    event AqlSetted(address indexed _account, uint _value, address indexed _sender);

    /// @notice Set the block quota limit
    function setBQL(uint _value) external returns (bool);

    /// @notice Set the default block quota limit
    function setDefaultAQL(uint _value) external returns (bool);

    /// @notice Set the account quota limit
    function setAQL(address _account, uint _value) external returns (bool);

    /// @notice Get all accounts that have account quota limit
    function getAccounts() external view returns (address[]);

    /// @notice Get all accounts' quotas
    function getQuotas() external view returns (uint[]);

    /// @notice Get block quota limit
    function getBQL() external view returns (uint);

    /// @notice Get default account quota limit
    function getDefaultAQL() external view returns (uint);

    /// @notice Get account quota limit
    function getAQL(address _account) external view returns (uint);

    /// @notice Get account quota limit
    function getAutoExecQL() external pure returns (uint);
}
