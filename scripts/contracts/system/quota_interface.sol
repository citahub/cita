pragma solidity ^0.4.14;

interface QuotaInterface {
    
    event SetGlobalEvent(bytes32 indexed key, bytes32 indexed value, address indexed _sender);
    event SetIsGlobalEvent(bytes32 indexed key, bool indexed value, address indexed _sender);
    event AddAdminEvent(address indexed _account, address indexed _sender);
    event SetSpecialEvent(address indexed _account, bytes32 indexed key, bytes32 value, address indexed _sender);

    function addAdmin(address _account) public returns (bool);
    function setIsGlobal(bytes32 _key, bool _value) public returns (bool);
    function setGlobal(bytes32 _key, bytes32 _value) public returns (bool);
    function setSpecial(address _account, bytes32 _key, bytes32 _value) public returns (bool);
    function setBlockGasLimit(uint _value) public returns (bool);
    function setGlobalAccountGasLimit(uint _value) public returns (bool);
    function setAccountGasLimit(address _account, uint _value) public returns (bool);
    function isAdmin(address _account) constant returns (bool);
    function getData(bytes32 _key) constant returns (bytes32);
    function getSpecialUsers() constant returns (string);
    function getUsersQuota() constant returns (string);
    function getblockGasLimit() constant returns (bytes32);
    function getAccountGasLimit() constant returns (bytes32);
    function getAccountQuota(address _user) constant returns (bytes32);
}
