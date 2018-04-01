pragma solidity ^0.4.18;

import "./quota_interface.sol";

contract QuotaManager is QuotaInterface {

    mapping (address => bool) admins;
    mapping (address => uint) quota;
    // Block quota limit
    uint BQL = 1073741824;
    // Default account quota limit
    uint defaultAQL = 268435456;
    address[] accounts;
    uint[] quotas;

    modifier onlyAdmin {
        require(admins[msg.sender]);
        _;
    }

    modifier checkBaseLimit(uint _v) {
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
        quota[_account] = 1073741824;
        accounts.push(_account);
        quotas.push(1073741824);
    }

    function addAdmin(address _account)
        public
        onlyAdmin
        returns (bool)
    {
        admins[_account] = true;
        AdminAdded(_account, msg.sender);
        return true;
    }

    function setBQL(uint _value)
        public
        onlyAdmin
        checkBaseLimit(_value)
        checkBlockLimit(_value)
        returns (bool)
    {
        BQL = _value;
        BqlSetted(_value, msg.sender);
        return true;
    }

    function setDefaultAQL(uint _value)
        public
        onlyAdmin
        checkBaseLimit(_value)
        returns (bool)
    {
        defaultAQL = _value;
        DefaultAqlSetted(_value, msg.sender);
        return true;
    }

    function setAQL(address _account, uint _value)
        public
        onlyAdmin
        checkBaseLimit(_value)
        returns (bool)
    {
        quota[_account] = _value;
        accounts.push(_account);
        quotas.push(_value);
        AqlSetted(
            _account,
            _value,
            msg.sender
        );
        return true;
    }

    function isAdmin(address _account)
        view
        public
        returns (bool)
    {
        return admins[_account];
    }

    function getAccounts()
        view
        public
        returns (address[])
    {
        return accounts;
    }

    function getQuotas()
        view
        public
        returns (uint[])
    {
        return quotas;
    }

    function getBQL()
        view
        public
        returns (uint)
    {
        return BQL;
    }

    function getDefaultAQL()
        view
        public
        returns (uint)
    {
        return defaultAQL;
    }

    function getAQL(address _account)
        view
        public
        returns (uint)
    {
        if (quota[_account] == 0)
            return defaultAQL;
        return quota[_account];
    }
}
