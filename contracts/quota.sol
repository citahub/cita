pragma solidity ^0.4.14;


contract QutotaInterface {
    mapping (address => bool) admins;
    mapping (bytes32 => bool) is_global;
    mapping (bytes32 => bytes32) global;
    mapping (address => mapping(bytes32 => bytes32)) special;
    // address[] special_accounts_array;
    // mapping (address => bool) special_accouts_mapping;

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
        // special_accounts_array.push(_account);
        // special_accouts_mapping[_account] = true;
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
        // if (!special_accouts_mapping[_account]) {
        //     special_accounts_array.push(_account);
        //     special_accouts_mapping[_account] = true;
        // }
        SetSpecialEvent(
            _account, 
            key, 
            value, 
            msg.sender
        );
        return true;
    }

    // function getAllGasLimit() {
    //     
    // }

    function getData(bytes32 key) constant returns (bytes32) {
        if (is_global[key]) {
            return global[key];
        } else {
            return _getData(key);
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
}
