pragma solidity ^0.4.14;

import "./strings.sol";


contract QutotaInterface {
    using strings for *;

    mapping (address => bool) admins;
    mapping (bytes32 => bool) is_global;
    mapping (bytes32 => bytes32) global;
    mapping (address => mapping(bytes32 => bytes32)) special;
    address[] special_users;
    bytes32[] users_quota;
    

    modifier onlyAdmin {
        if (admins[msg.sender]) {
            _;
        } else {
            revert();
        }
    }

    modifier checkLimit(uint _v) {
        uint maxLimit = 2 ** 63 - 1;
        if (_v > maxLimit) {
            revert();
        }
        _;
    }

    function addAdmin(address) onlyAdmin returns (bool) { }
    function isAdmin(address) constant returns (bool) { }
    function setIsGlobal(bytes32, bool) onlyAdmin returns (bool) { }
    function setGlobal(bytes32, bytes32) onlyAdmin returns (bool) { }
    function setSpecial(address, bytes32, bytes32) onlyAdmin returns (bool) { }
    function setBlockGasLimit(uint _value) onlyAdmin checkLimit(_value) returns (bool) { }
    function setGlobalAccountGasLimit(uint _value) onlyAdmin checkLimit(_value) returns (bool) { }
    function setAccountGasLimit(address, uint _value) onlyAdmin checkLimit(_value) returns (bool) { }
    function getData(bytes32) constant returns (bytes32) { }
    function getSpecialUsers() constant returns (string) { }
    function getUsersQuota() constant returns (string) { }
    function getblockGasLimit() constant returns (bytes32) { }
    function getAccountGasLimit() constant returns (bytes32) { }

    event SetGlobalEvent(bytes32 indexed key, bytes32 indexed value, address indexed _sender);
    event SetIsGlobalEvent(bytes32 indexed key, bool indexed value, address indexed _sender);
    event AddAdminEvent(address indexed _account, address indexed _sender);
    event SetSpecialEvent(address indexed _account, bytes32 indexed key, bytes32 value, address indexed _sender);
}


contract Quota is QutotaInterface {
    function Quota(address _account) {
        admins[_account] = true;
        is_global["blockGasLimit"] = true;
        global["blockGasLimit"] = bytes32(61415926);
        global["accountGasLimit"] = bytes32(25141592);
        special[_account]["accountGasLimit"] = bytes32(61415926);
        special_users.push(_account);
        users_quota.push(bytes32(61415926));
    }

    function addAdmin(address _account) onlyAdmin returns (bool) {
        admins[_account] = true;
        AddAdminEvent(_account, msg.sender);
        return true;
    }

    function isAdmin(address _account) constant returns (bool) {
        return admins[_account];
    }

    function setIsGlobal(bytes32 key, bool value) onlyAdmin returns (bool) {
        is_global[key] = value;
        SetIsGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setGlobal(bytes32 key, bytes32 value) onlyAdmin returns (bool) {
        global[key] = value;
        SetGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setSpecial(address _account, bytes32 key, bytes32 value) onlyAdmin returns (bool) {
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
        onlyAdmin checkLimit(_value) returns (bool)
    {
        bytes32 value = bytes32(_value);
        global["blockGasLimit"] = value;
        SetGlobalEvent(bytes32("blockGasLimit"), value, msg.sender);
        return true;
    }

    function setGlobalAccountGasLimit(uint _value)
        onlyAdmin checkLimit(_value) returns (bool)
    {
        bytes32 value = bytes32(_value);
        global["accountGasLimit"] = bytes32(value);
        SetGlobalEvent(bytes32("accountGasLimit"), bytes32(value), msg.sender);
        return true;
    }

    function setAccountGasLimit(address _account, uint _value)
        onlyAdmin checkLimit(_value) returns (bool)
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

    function getData(bytes32 key) constant returns (bytes32) {
        if (is_global[key]) {
            return global[key];
        } else {
            return _getData(key);
        }
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

    // cancat address
    function concatBytes(bytes32[] _users) internal returns (string bytes32List) {
        if (_users.length > 0) {
            bytes32List = bytes32ToString(_users[0]);
        }

        for (uint i = 1; i < _users.length; i++) {
            bytes32List = bytes32List.toSlice().concat(bytes32ToString(_users[i]).toSlice());
        }
    }

    // cancat address
    function concatUser(address[] _users) internal returns (string userList) {
        if (_users.length > 0) {
            userList = toString(_users[0]);
        }

        for (uint i = 1; i < _users.length; i++) {
            userList = userList.toSlice().concat(toString(_users[i]).toSlice());
        }
    }

    function _getData(bytes32 key) internal returns (bytes32) {
        bytes32 blank;
        if (special[msg.sender][key] != blank) {
            return special[msg.sender][key];
        } else {
            return global[key];
        }
    }

    // interface: address to string 
    // the returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++) {
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));
        }

        return string(b);
    }

    function bytes32ToString(bytes32 x) constant internal returns (string) {
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
        for (j = 0; j < charCount; j++) {
            bytesStringTrimmed[j] = bytesString[j];
        }
        return string(bytesStringTrimmed);
    }
}
