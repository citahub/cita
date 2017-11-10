pragma solidity ^0.4.14;

import "./strings.sol";
import "./quota_interface.sol";

contract QuotaManager is QuotaInterface {

    using strings for *;

    mapping (address => bool) admins;
    mapping (bytes32 => bool) is_global;
    mapping (bytes32 => bytes32) global;
    mapping (address => mapping(bytes32 => bytes32)) special;
    address[] special_users;
    bytes32[] users_quota;
    
    modifier onlyAdmin {
        require(admins[msg.sender]);
        _;
    }

    modifier checkLimit(uint _v) {
        uint maxLimit = 2 ** 63 - 1;
        uint baseLimit = 2 ** 22 - 1;
        require(_v <= maxLimit && _v >= baseLimit);
        _;
    }

    modifier checkBlockLimit(uint _v) {
        uint blockLimit = 2 ** 25 -1;
        require(_v >= blockLimit);
        _;
    }

    function QuotaManager(address _account) {
        admins[_account] = true;
        is_global["blockGasLimit"] = true;
        global["blockGasLimit"] = bytes32(61415926);
        global["accountGasLimit"] = bytes32(25141592);
        special[_account]["accountGasLimit"] = bytes32(61415926);
        special_users.push(_account);
        users_quota.push(bytes32(61415926));
    }

    function addAdmin(address _account) public onlyAdmin returns (bool) {
        admins[_account] = true;
        AddAdminEvent(_account, msg.sender);
    }

    function setIsGlobal(bytes32 key, bool value) public onlyAdmin returns (bool) {
        is_global[key] = value;
        SetIsGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setGlobal(bytes32 key, bytes32 value) public onlyAdmin returns (bool) {
        global[key] = value;
        SetGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setSpecial(address _account, bytes32 key, bytes32 value)
        public 
        onlyAdmin
        returns (bool)
    {
        special[_account][key] = value;
        SetSpecialEvent(
            _account, 
            key, 
            value, 
            msg.sender
        );
        return true;
    }

    function setBlockGasLimit(uint _value)
        public 
        onlyAdmin
        checkLimit(_value)
        returns (bool)
    {
        bytes32 value = bytes32(_value);
        global["blockGasLimit"] = value;
        SetGlobalEvent(bytes32("blockGasLimit"), value, msg.sender);
        return true;
    }

    function setGlobalAccountGasLimit(uint _value)
        public 
        onlyAdmin
        checkLimit(_value)
        returns (bool)
    {
        bytes32 value = bytes32(_value);
        global["accountGasLimit"] = bytes32(value);
        SetGlobalEvent(bytes32("accountGasLimit"), bytes32(value), msg.sender);
        return true;
    }

    function setAccountGasLimit(address _account, uint _value)
        public 
        onlyAdmin
        checkLimit(_value)
        returns (bool)
    {
        bytes32 key = bytes32("accountGasLimit");
        bytes32 value = bytes32(_value);
        special[_account]["accountGasLimit"] = value;
        special_users.push(_account);
        users_quota.push(bytes32(value));
        SetSpecialEvent(
            _account, 
            key, 
            value, 
            msg.sender
        );
        return true;
    }

    /// Cancat bytes32 
    function concatBytes(bytes32[] _users) internal returns (string bytes32List) {
        if (_users.length > 0)
            bytes32List = bytes32ToString(_users[0]);

        for (uint i = 1; i < _users.length; i++)
            bytes32List = bytes32List.toSlice().concat(bytes32ToString(_users[i]).toSlice());
    }

    /// Cancat address
    function concatUser(address[] _users) internal returns (string userList) {
        if (_users.length > 0)
            userList = toString(_users[0]);

        for (uint i = 1; i < _users.length; i++)
            userList = userList.toSlice().concat(toString(_users[i]).toSlice());
    }

    function _getData(bytes32 key) internal returns (bytes32) {
        bytes32 blank;
        if (special[msg.sender][key] != blank)
            return special[msg.sender][key];
        else
            return global[key];
    }

    /// Address to string 
    /// The returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++)
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));

        return string(b);
    }

    function bytes32ToString(bytes32 x) internal returns (string) {
        bytes memory bytesString = new bytes(32);
        uint charCount = 0;

        for (uint j = 0; j < 32; j++) {
            byte char = byte(bytes32(uint(x) * 2 ** (8 * j)));
            if (char != 0) {
                bytesString[charCount] = char;
                charCount++;
            }
        }

        bytes memory bytesStringTrimmed = new bytes(charCount);

        for (j = 0; j < charCount; j++)
            bytesStringTrimmed[j] = bytesString[j];

        return string(bytesStringTrimmed);
    }

    function isAdmin(address _account) constant returns (bool) {
        return admins[_account];
    }

    function getData(bytes32 key) constant returns (bytes32) {
        if (is_global[key])
            return global[key];
        else
            return _getData(key);
    }

    function getSpecialUsers() constant returns (string) {
        return concatUser(special_users);
    }

    function getUsersQuota() constant returns (string) {
        return concatBytes(users_quota);
    }

    function getblockGasLimit() constant returns (bytes32) {
        return global["blockGasLimit"];
    }

    function getAccountGasLimit() constant returns (bytes32) {
        return global["accountGasLimit"];
    }

    function getAccountQuota(address _user) constant returns (bytes32) {
        // Not special users, then return accountGasLimit
        // or query the special users array
        if (special[_user]["accountGasLimit"] == bytes32(0))
            return global["accountGasLimit"];
        return special[_user]["accountGasLimit"];
    }
}
