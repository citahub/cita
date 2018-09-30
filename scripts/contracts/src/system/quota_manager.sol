pragma solidity ^0.4.24;

import "../common/error.sol";
import "../lib/address_array.sol";
import "../common/admin.sol";
import "../common/address.sol";
import "../permission_management/authorization.sol";


/// @title The interface of quota_manager
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface QuotaInterface {
    
    event DefaultAqlSetted(uint indexed _value, address indexed _sender);
    event BqlSetted(uint indexed _value, address indexed _sender);
    event AqlSetted(address indexed _account, uint _value, address indexed _sender);

    /// @notice Set the block quota limit
    function setBQL(uint _value) external returns (bool);

    /// @notice Set the default block quota limit
    function setDefaultAQL(uint _value) external returns (bool);

    /// @notice Set the account quota limit
    function setAQL(address _account, uint _value) external returns (bool);

    /// @notice Get all accounts that have account quota limit
    function getAccounts() external view returns (address[]);

    /// @notice Get all accounts' quotas
    function getQuotas() external view returns (uint[]);

    /// @notice Get block quota limit
    function getBQL() external view returns (uint);

    /// @notice Get default account quota limit
    function getDefaultAQL() external view returns (uint);
    
    /// @notice Get account quota limit
    function getAQL(address _account) external view returns (uint);
}


/// @title Node manager contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020003
contract QuotaManager is QuotaInterface, Error, ReservedAddress {

    mapping(address => uint) quota;
    // Block quota limit
    uint BQL = 1073741824;
    // Default account quota limit
    uint defaultAQL = 268435456;
    address[] accounts;
    uint[] quotas;
    uint maxLimit = 2 ** 63 - 1;
    uint baseLimit = 2 ** 22 - 1;
    Admin admin = Admin(adminAddr);
    Authorization auth = Authorization(authorizationAddr);

    modifier checkBaseLimit(uint _v) {
        if (_v <= maxLimit && _v >= baseLimit)
            _;
        else {
            emit ErrorLog(ErrorType.OutOfBaseLimit, "The value is out of base limit");
            return;
        }
    }

    modifier checkBlockLimit(uint _v) {
        uint blockLimit = 2 ** 28 - 1;
        if (_v > blockLimit)
            _;
        else {
            emit ErrorLog(ErrorType.OutOfBlockLimit, "The value is out of block limit");
            return;
        }
    }

    modifier checkPermission(address _permission) {
        require(auth.checkPermission(msg.sender, _permission), "permission denied.");
        _;
    }

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    /// @notice Setup
    constructor(address _admin)
        public
    {
        quota[_admin] = BQL;
        accounts.push(_admin);
        quotas.push(BQL);
    }

    /// @notice Set the default account quota limit
    /// @param _value The value to be setted
    /// @return true if successed, otherwise false
    function setDefaultAQL(uint _value)
        public
        onlyAdmin
        checkBaseLimit(_value)
        checkPermission(builtInPermissions[18])
        returns (bool)
    {
        defaultAQL = _value;
        emit DefaultAqlSetted(_value, msg.sender);
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
        checkPermission(builtInPermissions[18])
        returns (bool)
    {
        uint i = AddressArray.index(_account, accounts);
        if (i == accounts.length) {
            // Not exist
            accounts.push(_account);
            quotas.push(_value);
        } else {
            quotas[i] = _value;
        }
        quota[_account] = _value;
        emit AqlSetted(
            _account,
            _value,
            msg.sender
        );
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
        checkPermission(builtInPermissions[19])
        returns (bool)
    {
        BQL = _value;
        emit BqlSetted(_value, msg.sender);
        return true;
    }

    /// @notice Get all accounts that have account quota limit
    /// @return The accounts that have AQL
    function getAccounts()
        public
        view
        returns (address[])
    {
        return accounts;
    }

    /// @notice Get all accounts' quotas
    /// @return The accounts' quotas
    function getQuotas()
        public
        view
        returns (uint[])
    {
        return quotas;
    }

    /// @notice Get block quota limit
    /// @return The block quota limit
    function getBQL()
        public
        view
        returns (uint)
    {
        return BQL;
    }

    /// @notice Get default account quota limit
    /// @return The default account quota limit
    function getDefaultAQL()
        public
        view
        returns (uint)
    {
        return defaultAQL;
    }

    /// @notice Get account quota limit
    /// @return The account quota limit
    function getAQL(address _account)
        public
        view
        returns (uint)
    {
        if (quota[_account] == 0)
            return defaultAQL;
        return quota[_account];
    }
}
