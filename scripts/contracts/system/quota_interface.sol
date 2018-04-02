pragma solidity ^0.4.18;

interface QuotaInterface {
    
    event DefaultAqlSetted(uint indexed value, address indexed _sender);
    event BqlSetted(uint indexed value, address indexed _sender);
    event AdminAdded(address indexed _account, address indexed _sender);
    event AqlSetted(address indexed _account, uint value, address indexed _sender);

    function addAdmin(address _account) public returns (bool);
    function setBQL(uint _value) public returns (bool);
    function setDefaultAQL(uint _value) public returns (bool);
    function setAQL(address _account, uint _value) public returns (bool);
    function isAdmin(address _account) view public returns (bool);
    function getAccounts() view public returns (address[]);
    function getQuotas() view public returns (uint[]);
    function getBQL() view public returns (uint);
    function getDefaultAQL() view public returns (uint);
    function getAQL(address _account) view public returns (uint);
}
