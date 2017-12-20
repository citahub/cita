pragma solidity ^0.4.18;

interface QuotaInterface {
    
    event SetGlobalEvent(bytes32 indexed key, uint indexed value, address indexed _sender);
    event SetIsGlobalEvent(bytes32 indexed key, bool indexed value, address indexed _sender);
    event AddAdminEvent(address indexed _account, address indexed _sender);
    event SetSpecialEvent(address indexed _account, bytes32 indexed key, uint value, address indexed _sender);

    function addAdmin(address _account) public returns (bool);
    function setIsGlobal(bytes32 _key, bool _value) public returns (bool);
    function setGlobal(bytes32 _key, uint _value) public returns (bool);
    function setSpecial(address _account, bytes32 _key, uint _value) public returns (bool);
    function setBlockGasLimit(uint _value) public returns (bool);
    function setGlobalAccountGasLimit(uint _value) public returns (bool);
    function setAccountGasLimit(address _account, uint _value) public returns (bool);
    function isAdmin(address _account) view public returns (bool);
    function getData(bytes32 _key) view public returns (uint);
    function getSpecialUsers() view public returns (address[]);
    function getUsersQuota() view public returns (uint[]);
    function getblockGasLimit() view public returns (uint);
    function getAccountGasLimit() view public returns (uint);
    function getAccountQuota(address _user) view public returns (uint);
}
