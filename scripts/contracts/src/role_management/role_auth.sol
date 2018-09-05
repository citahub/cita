pragma solidity ^0.4.24;

import "./role_creator.sol";
import "../permission_management/authorization.sol";
import "../lib/contract_check.sol";
import "../lib/address_array.sol";


/// @title Authorization about role and account
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff02000d
///         The interface can be called: Only query type
contract RoleAuth is ReservedAddress {

    Authorization auth = Authorization(authorizationAddr);

    mapping(address => address[]) internal accounts;
    mapping(address => address[]) internal roles;

    event RoleSetted(address indexed _account, address indexed _role);
    event RoleCanceled(address indexed _account, address indexed _role);
    event RoleCleared(address indexed _account);

    modifier onlyRoleManagement {
        require(roleManagementAddr == msg.sender, "permission denied.");
        _;
    }

    /// @notice Set the role to the account
    /// @param _account The account to be setted
    /// @param _role The role to be setted
    /// @return true if successed, otherwise false
    function setRole(address _account, address _role)
        external
        onlyRoleManagement
        returns (bool)
    {

        if (!AddressArray.exist(_role, roles[_account])) {
            roles[_account].push(_role);
            // Set role permissions to account.
            require(_setRolePermissions(_account, _role), "setRolePermissions failed.");
        }
        if (!AddressArray.exist(_account, accounts[_role]))
            accounts[_role].push(_account);

        emit RoleSetted(_account, _role);
        return true;
    }

    /// @notice Cancel the account's role
    /// @param _account The account to be canceled
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function cancelRole(address _account, address _role)
        external
        onlyRoleManagement
        returns (bool)
    {
        return _cancelRole(_account, _role);
    }

    /// @notice Clear all the accounts that have the role
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function clearAuthOfRole(address _role)
        external
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < accounts[_role].length; i++)
            require(_cancelRole(accounts[_role][i], _role), "cancelRole failed.");

        return true;
    }

    /// @notice Set all the accounts that have the permissions
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function setPermissionsOfRole(address _role, address[] _permissions)
        external
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < accounts[_role].length; i++)
            require(_setPermissions(accounts[_role][i], _permissions), "setPermissions failed.");

        return true;
    }

    /// @notice Cancel all the accounts that have the permissions
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function cancelPermissionsOfRole(address _role, address[] _permissions)
        external
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < accounts[_role].length; i++)
            require(_cancelPermissions(accounts[_role][i], _permissions), "cancelPermissions failed.");

        return true;
    }

    /// @notice Clear the account's role
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearRole(address _account)
        external
        onlyRoleManagement
        returns (bool)
    {
        // Clear account and roles
        for (uint i = 0; i < roles[_account].length; i++) {
            // Clear account auth
            require(_cancelRolePermissions(_account, roles[_account][i]), "cancelRolePermissions failed.");
            // Clear _account in all roles array.
            assert(AddressArray.remove(_account, accounts[roles[_account][i]]));
        }

        // Clear all roles associate with _account
        delete roles[_account];
        emit RoleCleared(_account);

        return true;
    }

    /// @notice Query the roles of the account
    /// @param _account The account to be queried
    /// @return The roles of the account
    function queryRoles(address _account)
        public
        view
        returns (address[])
    {
        return roles[_account];
    }

    /// @notice Query the accounts that have the role
    /// @param _role The role to be queried
    /// @return The accounts that have the role
    function queryAccounts(address _role)
        public
        view
        returns (address[])
    {
        return accounts[_role];
    }

    /// @notice Check the account has the permission
    /// @param _account The account to be checked
    /// @param _permission The permission to be checked
    /// @return true if has, otherwise false
    function hasPermission(address _account, address _permission)
        public 
        view
        returns (bool)
    {

        return auth.checkPermission(_account, _permission);
    }

    /// @notice Private: cancelRole
    function _cancelRole(address _account, address _role)
        private
        returns (bool)
    {
        assert(AddressArray.remove(_account, accounts[_role]));
        assert(AddressArray.remove(_role, roles[_account]));

        // Cancel role permissions of account.
        require(_cancelRolePermissions(_account, _role), "cancelRolePermissions failed.");

        emit RoleCanceled(_account, _role);
        return true;
    }

    /// @notice Private: cancel role of account
    function _cancelRolePermissions(address _account, address _role)
        private
        returns (bool)
    {
        address[] memory permissions = _queryPermissions(_role);
        require(_cancelPermissions(_account, permissions), "cancelPermissions failed.");
        return true;
    }

    /// @notice Private: cancel permissions of account
    function _cancelPermissions(address _account, address[] _permissions)
        private
        returns (bool)
    {
        for (uint i = 0; i<_permissions.length; i++) {
            // Cancel this permission when account has not it in any of his other roles
            if (!_hasPermission(_account, _permissions[i]))
                require(auth.cancelAuth(_account, _permissions[i]), "cancelAuth failed.");
        }

        return true;
    }

    /// @notice Private: account has permission in one of his roles
    function _hasPermission(address _account, address _permission)
        private
        view
        returns (bool)
    {
        for (uint i = 0; i < roles[_account].length; i++) {
            Role roleContract = Role(roles[_account][i]);
            if (roleContract.inPermissions(_permission))
                return true;
        }
    }
    /// @notice Private: set all role permissions of account
    function _setRolePermissions(address _account, address _role)
        private
        returns (bool)
    {
        address[] memory permissions = _queryPermissions(_role);
        require(_setPermissions(_account, permissions), "setPermissions failed.");
        return true;
    }

    /// @notice Private: set permissions of account
    function _setPermissions(address _account, address[] _permissions)
        private
        returns (bool)
    {
        for (uint i = 0; i<_permissions.length; i++)
            require(auth.setAuth(_account, _permissions[i]), "setAuth failed.");

        return true;
    }

    /// @notice Private: Query the permissions of the role
    function _queryPermissions(address _role)
        private
        view
        returns (address[] permissions)
    {
        require(ContractCheck.isContract(_role), "not a valid contract address.");
        Role roleContract = Role(_role);
        permissions = roleContract.queryPermissions();
    }
}
