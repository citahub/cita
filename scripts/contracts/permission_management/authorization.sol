pragma solidity ^0.4.18;

import "./permission.sol";
import "./address_array.sol";


/// @title Authorization about the permission and account
/// @notice Only be called by permission_management contract except query interface
contract Authorization {

    address permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    address newPermissionAddr = 0x00000000000000000000000000000000013241b5;
    address deletePermissionAddr = 0x00000000000000000000000000000000023241b5;
    address updatePermissionAddr = 0x00000000000000000000000000000000033241B5;
    address setAuthAddr = 0x00000000000000000000000000000000043241b5;
    address cancelAuthAddr = 0x00000000000000000000000000000000053241b5;
    address newRoleAddr = 0x00000000000000000000000000000000063241b5;
    address deleteRoleAddr = 0x00000000000000000000000000000000073241b5;
    address updateRoleAddr = 0x00000000000000000000000000000000083241B5;
    address setRoleAddr = 0x00000000000000000000000000000000093241B5;
    address cancelRoleAddr = 0x000000000000000000000000000000000A3241b5;
    address newGroupAddr = 0x000000000000000000000000000000000b3241b5;
    address deleteGroupAddr = 0x000000000000000000000000000000000C3241B5;
    address updateGroupAddr = 0x000000000000000000000000000000000D3241b5;
    address sendTxAddr = 0x0000000000000000000000000000000000000001;
    address createContractAddr = 0x0000000000000000000000000000000000000002;

    address rootGroup = 0x00000000000000000000000000000000013241b6;

    mapping(address => address[]) permissions;
    mapping(address => address[]) accounts;

    address[] all_accounts;

    event AuthSetted(address indexed _account, address indexed _permission);
    event AuthCanceled(address indexed _account, address indexed _permission);
    event AuthCleared(address indexed _account);

    modifier onlyPermissionManagement {
        require(permissionManagementAddr == msg.sender);
        _;
    }

    /// @dev Initialize the superAdmin's auth
    function Authorization(address _superAdmin) public {
        _setAuth(_superAdmin, sendTxAddr);
        _setAuth(_superAdmin, createContractAddr);
        _setAuth(_superAdmin, newPermissionAddr);
        _setAuth(_superAdmin, deletePermissionAddr);
        _setAuth(_superAdmin, updatePermissionAddr);
        _setAuth(_superAdmin, setAuthAddr);
        _setAuth(_superAdmin, cancelAuthAddr);
        _setAuth(_superAdmin, newRoleAddr);
        _setAuth(_superAdmin, deleteRoleAddr);
        _setAuth(_superAdmin, updateRoleAddr);
        _setAuth(_superAdmin, setRoleAddr);
        _setAuth(_superAdmin, cancelRoleAddr);
        _setAuth(_superAdmin, newGroupAddr);
        _setAuth(_superAdmin, deleteGroupAddr);
        _setAuth(_superAdmin, updateGroupAddr);
        // rootGroup: basic permissions
        _setAuth(rootGroup, sendTxAddr);
        _setAuth(rootGroup, createContractAddr);
    }

    /// @dev Set authorization
    function setAuth(address _account, address _permission)
        public
        onlyPermissionManagement
        returns (bool)
    {
        return _setAuth(_account, _permission);
    }

    /// @dev Cancel authorization
    function cancelAuth(address _account, address _permission)
        public
        onlyPermissionManagement
        returns (bool)
    {
        AddressArray.remove(_account, accounts[_permission]);
        AddressArray.remove(_permission, permissions[_account]);
        AuthCanceled(_account, _permission);
        return true;
    }

    /// @dev Clear the account's auth
    function clearAuth(address _account)
        public
        onlyPermissionManagement
        returns (bool)
    {
        // Delete the account of all the account's permissions
        for (uint i = 0; i < permissions[_account].length; i++)
            AddressArray.remove(_account, accounts[permissions[_account][i]]);

        delete permissions[_account];
        AddressArray.remove(_account, all_accounts);

        AuthCleared(_account);
        return true;
    }

    /// @dev Clear the auth of the accounts who have the permission
    /// @notice TODO Rename cancelAuthOfPermission
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

    /// @dev Query the account's permissions
    function queryPermissions(address _account)
        public
        view
        returns (address[] _permissions)
    {
        return permissions[_account];
    }

    /// @dev Query the permission's accounts
    function queryAccounts(address _permission)
        public
        view
        returns (address[] _accounts)
    {
        return accounts[_permission];
    }

    /// @dev Query all accounts
    function queryAllAccounts()
        public
        view
        returns (address[])
    {
        return all_accounts;
    }

    /// @dev Check Permission
    function checkPermission(address _account, address _cont, bytes4 _func)
        public
        view
        returns (bool)
    {
        address[] memory perms = queryPermissions(_account);

        for (uint i = 0; i < perms.length; i++) {
            Permission perm = Permission(perms[i]);
            if (perm.inPermission(_cont, _func))
                return true;
        }

        return false;
    }

    /// @dev Set authorization
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

        AuthSetted(_account, _permission);
        return true;
    }
}
