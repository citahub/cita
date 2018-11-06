pragma solidity ^0.4.24;

import "../lib/AddressArray.sol";
import "../common/ReservedAddrPublic.sol";
import "../interfaces/ISysConfig.sol";
import "../interfaces/IAuthorization.sol";

/// @title Authorization about the permission and account
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020006
///         The interface can be called: Only query type
contract Authorization is IAuthorization, ReservedAddrPublic {

    mapping(address => address[]) permissions;
    mapping(address => address[]) accounts;

    address[] all_accounts;
    ISysConfig sysConfig = ISysConfig(sysConfigAddr);

    event AuthSetted(address indexed _account, address indexed _permission);
    event AuthCanceled(address indexed _account, address indexed _permission);
    event AuthCleared(address indexed _account);

    modifier onlyPermissionManagement {
        require(
            permissionManagementAddr == msg.sender || roleAuthAddr == msg.sender,
            "permission denied"
        );
        _;
    }

    modifier notSuperAdmin(address _account) {
        require(_account != all_accounts[0], "not superAdmin");
        _;
    }

    /// @notice Initialize the superAdmin's auth
    constructor(address _superAdmin) public {
        for (uint8 i;i < builtInPermissions.length; i++)
            _setAuth(_superAdmin, builtInPermissions[i]);

        // rootGroup: basic permissions
        _setAuth(rootGroupAddr, sendTxAddr);
        _setAuth(rootGroupAddr, createContractAddr);
    }

    /// @notice Set permission to the account
    /// @param _account The account to be setted
    /// @param _permission The permission to be setted
    /// @return true if successed, otherwise false
    function setAuth(address _account, address _permission)
        public
        onlyPermissionManagement
        returns (bool)
    {
        return _setAuth(_account, _permission);
    }

    /// @notice Cancel the account's permission
    /// @param _account The account to be canceled
    /// @param _permission The permission to be canceled
    /// @return true if successed, otherwise false
    function cancelAuth(address _account, address _permission)
        public
        onlyPermissionManagement
        notSuperAdmin(_account)
        returns (bool)
    {
        AddressArray.remove(_account, accounts[_permission]);
        AddressArray.remove(_permission, permissions[_account]);
        emit AuthCanceled(_account, _permission);
        return true;
    }

    /// @notice Clear the account's permissions
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearAuth(address _account)
        public
        onlyPermissionManagement
        notSuperAdmin(_account)
        returns (bool)
    {
        // Delete the account of all the account's permissions
        for (uint i = 0; i < permissions[_account].length; i++)
            AddressArray.remove(_account, accounts[permissions[_account][i]]);

        delete permissions[_account];
        AddressArray.remove(_account, all_accounts);

        emit AuthCleared(_account);
        return true;
    }

    /// @notice Clear the auth of the accounts who have the permission
    /// @param _permission The permission to be cleared
    /// @return true if successed, otherwise false
    function clearAuthOfPermission(address _permission)
        public
        onlyPermissionManagement
        returns (bool)
    {
        address[] memory _accounts = queryAccounts(_permission);

        for (uint i = 0; i < _accounts.length; i++)
            assert(cancelAuth(_accounts[i], _permission));

        return true;
    }

    /// @notice Query the account's permissions
    /// @param _account The account to be queried
    /// @return The permissions of account
    function queryPermissions(address _account)
        public
        view
        returns (address[] _permissions)
    {
        return permissions[_account];
    }

    /// @notice Query the permission's accounts
    /// @param _permission The permission to be queried
    /// @return The accounts of permission
    function queryAccounts(address _permission)
        public
        view
        returns (address[] _accounts)
    {
        return accounts[_permission];
    }

    /// @notice Query all accounts
    /// @return All the accounts
    function queryAllAccounts()
        public
        view
        returns (address[])
    {
        return all_accounts;
    }

    /// @notice Check account has a resource(deprecation)
    /// @return true if passed, otherwise false
    function checkResource(address, address, bytes4)
        public
        pure
        returns (bool)
    {
        // address[] memory perms = queryPermissions(_account);

        // for (uint i = 0; i < perms.length; i++) {
        //     Permission perm = Permission(perms[i]);
        //     if (perm.inPermission(_cont, _func))
        //         return true;
        // }

        // return false;
    }

    /// @notice Check account has a permission
    /// @param _account The account to be checked
    /// @param _permission The address of permission
    /// @return true if passed, otherwise false
    function checkPermission(address _account, address _permission)
        public
        view
        returns (bool)
    {
        if (sysConfig.getPermissionCheck()) {
            return AddressArray.exist(_permission, permissions[_account]);
        }
        return true;
    }

    /// @notice Private: Set the permission to the account
    function _setAuth(address _account, address _permission)
        private
        returns (bool)
    {
        if (!AddressArray.exist(_permission, permissions[_account]))
            permissions[_account].push(_permission);
        if (!AddressArray.exist(_account, accounts[_permission]))
            accounts[_permission].push(_account);
        if (!AddressArray.exist(_account, all_accounts))
            all_accounts.push(_account);

        emit AuthSetted(_account, _permission);
        return true;
    }
}
