pragma solidity ^0.4.18;

import "./quota_interface.sol";

contract QuotaManager is QuotaInterface {

    mapping (address => bool) admins;
    mapping (bytes32 => bool) is_global;
    mapping (bytes32 => uint) global;
    mapping (address => mapping(bytes32 => uint)) special;
    address[] special_users;
    uint[] users_quota;

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
        uint blockLimit = 2 ** 28 - 1;
        require(_v > blockLimit);
        _;
    }

    function QuotaManager(address _account) public {
        admins[_account] = true;
        is_global["blockGasLimit"] = true;
        global["blockGasLimit"] = 1073741824;
        global["accountGasLimit"] = 268435456;
        special[_account]["accountGasLimit"] = 1073741824;
        special_users.push(_account);
        users_quota.push(1073741824);
    }

    function addAdmin(address _account)
        public
        onlyAdmin
        returns (bool)
    {
        admins[_account] = true;
        AddAdminEvent(_account, msg.sender);
    }

    function setIsGlobal(bytes32 key, bool value)
        public
        onlyAdmin
        returns (bool)
    {
        is_global[key] = value;
        SetIsGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setGlobal(bytes32 key, uint value)
        public
        onlyAdmin
        returns (bool)
    {
        global[key] = value;
        SetGlobalEvent(key, value, msg.sender);
        return true;
    }

    function setSpecial(address _account, bytes32 key, uint value)
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
        checkBlockLimit(_value)
        returns (bool)
    {
        global["blockGasLimit"] = _value;
        SetGlobalEvent(bytes32("blockGasLimit"), _value, msg.sender);
        return true;
    }

    function setGlobalAccountGasLimit(uint _value)
        public
        onlyAdmin
        checkLimit(_value)
        returns (bool)
    {
        global["accountGasLimit"] = _value;
        SetGlobalEvent(bytes32("accountGasLimit"), _value, msg.sender);
        return true;
    }

    function setAccountGasLimit(address _account, uint _value)
        public
        onlyAdmin
        checkLimit(_value)
        returns (bool)
    {
        bytes32 key = bytes32("accountGasLimit");
        special[_account]["accountGasLimit"] = _value;
        special_users.push(_account);
        users_quota.push(_value);
        SetSpecialEvent(
            _account,
            key,
            _value,
            msg.sender
        );
        return true;
    }

    function _getData(bytes32 key)
        view
        internal
        returns (uint)
    {
        if (special[msg.sender][key] != 0)
            return special[msg.sender][key];
        else
            return global[key];
    }

    function isAdmin(address _account)
        view
        public
        returns (bool)
    {
        return admins[_account];
    }

    function getData(bytes32 key)
        view
        public
        returns (uint)
    {
        if (is_global[key])
            return global[key];
        else
            return _getData(key);
    }

    function getSpecialUsers()
        view
        public
        returns (address[])
    {
        return special_users;
    }

    function getUsersQuota()
        view
        public
        returns (uint[])
    {
        return users_quota;
    }

    function getblockGasLimit()
        view
        public
        returns (uint)
    {
        return global["blockGasLimit"];
    }

    function getAccountGasLimit()
        view
        public
        returns (uint)
    {
        return global["accountGasLimit"];
    }

    function getAccountQuota(address _user)
        view
        public
        returns (uint)
    {
        // Not special users, then return accountGasLimit
        // or query the special users array
        if (special[_user]["accountGasLimit"] == 0)
            return global["accountGasLimit"];
        return special[_user]["accountGasLimit"];
    }
}
