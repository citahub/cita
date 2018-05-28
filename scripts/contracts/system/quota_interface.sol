pragma solidity ^0.4.14;


/// @title The interface of quota_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface QuotaInterface {
    
    event DefaultAqlSetted(uint indexed value, address indexed _sender);
    event BqlSetted(uint indexed value, address indexed _sender);
    event AdminAdded(address indexed _account, address indexed _sender);
    event AqlSetted(address indexed _account, uint value, address indexed _sender);

    /// @notice Add an admin
    function addAdmin(address _account) public returns (bool);
    /// @notice Set the block quota limit
    function setBQL(uint _value) public returns (bool);
    /// @notice Set the default block quota limit
    function setDefaultAQL(uint _value) public returns (bool);
    /// @notice Set the account quota limit
    function setAQL(address _account, uint _value) public returns (bool);
    /// @notice Check the account is admin
    function isAdmin(address _account) public constant returns (bool);
    /// @notice Get all accounts that have account quota limit
    function getAccounts() public constant returns (address[]);
    /// @notice Get all accounts' quotas
    function getQuotas() public constant returns (uint[]);
    /// @notice Get block quota limit
    function getBQL() public constant returns (uint);
    /// @notice Get default account quota limit
    function getDefaultAQL() public constant returns (uint);
    /// @notice Get account quota limit
    function getAQL(address _account) public constant returns (uint);
}
