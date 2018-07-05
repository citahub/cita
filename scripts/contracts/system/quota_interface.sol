pragma solidity ^0.4.18;


/// @title The interface of quota_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface QuotaInterface {
    
    event DefaultAqlSetted(uint indexed _value, address indexed _sender);
    event BqlSetted(uint indexed _value, address indexed _sender);
    event AdminAdded(address indexed _account, address indexed _sender);
    event AqlSetted(address indexed _account, uint _value, address indexed _sender);

    /// @notice Add an admin
    function addAdmin(address _account) public returns (bool);

    /// @notice Set the block quota limit
    function setBQL(uint _value) public returns (bool);

    /// @notice Set the default block quota limit
    function setDefaultAQL(uint _value) public returns (bool);

    /// @notice Set the account quota limit
    function setAQL(address _account, uint _value) public returns (bool);

    /// @notice Check the account is admin
    function isAdmin(address _account) public view returns (bool);

    /// @notice Get all accounts that have account quota limit
    function getAccounts() public view returns (address[]);

    /// @notice Get all accounts' quotas
    function getQuotas() public view returns (uint[]);

    /// @notice Get block quota limit
    function getBQL() public view returns (uint);

    /// @notice Get default account quota limit
    function getDefaultAQL() public view returns (uint);
    
    /// @notice Get account quota limit
    function getAQL(address _account) public view returns (uint);
}
