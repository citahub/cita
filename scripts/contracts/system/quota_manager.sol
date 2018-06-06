pragma solidity ^0.4.18;

import "./quota_interface.sol";


/// @title Node manager contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0x00000000000000000000000000000000013241a3
contract QuotaManager is QuotaInterface {

    mapping(address => bool) admins;
    mapping(address => uint) quota;
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

    /// @notice Setup
    function QuotaManager(address _admin) public {
        admins[_admin] = true;
        quota[_admin] = 1073741824;
        accounts.push(_admin);
        quotas.push(1073741824);
    }

    /// @notice Add an admin
    /// @param _account Address of the admin
    /// @return true if successed, otherwise false
    function addAdmin(address _account)
        public
        onlyAdmin
        returns (bool)
    {
        admins[_account] = true;
        AdminAdded(_account, msg.sender);
        return true;
    }

    /// @notice Set the block quota limit
    /// @param _value The value to be setted
    /// @return true if successed, otherwise false
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

    /// @notice Set the default account quota limit
    /// @param _value The value to be setted
    /// @return true if successed, otherwise false
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

    /// @notice Set the account quota limit
    /// @param _account The account to be setted
    /// @param _value The value to be setted
    /// @return true if successed, otherwise false
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

    /// @notice Check the account is admin
    /// @param _account The address to be checked
    /// @return true if it is, otherwise false
    function isAdmin(address _account)
        view
        public
        returns (bool)
    {
        return admins[_account];
    }

    /// @notice Get all accounts that have account quota limit
    /// @return The accounts that have AQL
    function getAccounts()
        view
        public
        returns (address[])
    {
        return accounts;
    }

    /// @notice Get all accounts' quotas
    /// @return The accounts' quotas
    function getQuotas()
        view
        public
        returns (uint[])
    {
        return quotas;
    }

    /// @notice Get block quota limit
    /// @return The block quota limit
    function getBQL()
        view
        public
        returns (uint)
    {
        return BQL;
    }

    /// @notice Get default account quota limit
    /// @return The default account quota limit
    function getDefaultAQL()
        view
        public
        returns (uint)
    {
        return defaultAQL;
    }

    /// @notice Get account quota limit
    /// @return The account quota limit
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
