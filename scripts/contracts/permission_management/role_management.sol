pragma solidity ^0.4.14;

import "./role_creator.sol";
import "./permission_management.sol";
import "../common/contract_check.sol";
import "../common/address_array.sol";


/// @title Role management contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xe3b5ddb80addb513b5c981e27bb030a86a8821ee
///         The interface the can be called: All
/// @dev TODO Split to a new file: role_auth.sol
contract RoleManagement {

    address roleCreatorAddress = 0xe9E2593C7D1Db5EE843c143E9cB52b8d996b2380;
    RoleCreator roleCreator = RoleCreator(roleCreatorAddress);

    address internal roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;
    address internal permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    address internal authorizationAddr = 0x00000000000000000000000000000000013241b4;
    // bytes4 internal queryPermissionsHash = 0x46f02832;

    PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
    Authorization authContract = Authorization(authorizationAddr);

    mapping(address => address[]) internal accounts;
    mapping(address => address[]) internal roles;

    event RoleSetted(address indexed _account, address indexed _role);
    event RoleCanceled(address indexed _account, address indexed _role);
    event RoleCleared(address indexed _account);

    /// @notice Create a new role
    /// @param _name The name of role
    /// @param _permissions The permissions of role
    /// @return New role's address
    function newRole(bytes32 _name, address[] _permissions)
        external
        returns (address roleid)
    {
        return roleCreator.createRole(_name, _permissions);
    }

    /// @notice Delete the role
    /// @param _roleid The address of role
    /// @return true if successed, otherwise false
    function deleteRole(address _roleid)
        external
        returns (bool)
    {
        // Cancel the role of the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++)
            assert(_cancelRole(accounts[_roleid][i], _roleid));

        Role roleContract = Role(_roleid);
        require(roleContract.deleteRole());

        return true;
    }

    /// @notice Update role's name
    /// @param _roleid The address of role
    /// @param _name The new name of role
    /// @return true if successed, otherwise false
    function updateRoleName(address _roleid, bytes32 _name)
        external
        returns (bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.updateName(_name);
    }

    /// @notice Add permissions of role
    /// @param _roleid The address of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function addPermissions(address _roleid, address[] _permissions)
        external
        returns (bool)
    {
        // Set the authorization of all the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++)
            require(_setPermissions(accounts[_roleid][i], _permissions));

        Role roleContract = Role(_roleid);
        require(roleContract.addPermissions(_permissions));
        return true;
    }

    /// @notice Delete permissions of role
    /// @param _roleid The address of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function deletePermissions(address _roleid, address[] _permissions)
        external
        returns (bool)
    {
        Role roleContract = Role(_roleid);
        require(roleContract.deletePermissions(_permissions));

        // Cancel the authorization of all the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++)
            require(_cancelPermissions(accounts[_roleid][i], _permissions));

        return true;
    }

    /// @notice Set the role to the account
    /// @param _account The account to be setted
    /// @param _role The role to be setted
    /// @return true if successed, otherwise false
    function setRole(address _account, address _role)
        external
        returns (bool)
    {

        if (!AddressArray.exist(_role, roles[_account])) {
            roles[_account].push(_role);
            // Set role permissions to account.
            require(_setRolePermissions(_account, _role));
        }
        if (!AddressArray.exist(_account, accounts[_role]))
            accounts[_role].push(_account);

        RoleSetted(_account, _role);
        return true;
    }

    /// @notice Cancel the account's role
    /// @param _account The account to be canceled
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function cancelRole(address _account, address _role)
        external
        returns (bool)
    {
        return _cancelRole(_account, _role);
    }

    /// @notice Clear the account's role
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearRole(address _account)
        external
        returns (bool)
    {
        // Clear account and roles
        for (uint i = 0; i < roles[_account].length; i++) {
            // Clear account auth
            require(_cancelRolePermissions(_account, roles[_account][i]));
            // Clear _account in all roles array.
            assert(AddressArray.remove(_account, accounts[roles[_account][i]]));
        }

        // Clear all roles associate with _account
        delete roles[_account];
        RoleCleared(_account);

        return true;
    }

    /// @notice Query the permissions of the role
    /// @param _role The role to be queried
    /// @return The permissions of the role
    function queryPermissions(address _role)
        public
        returns (address[])
    {
        require(ContractCheck.isContract(_role));
        Role roleContract = Role(_role);
        uint len = roleContract.lengthOfPermissions();
        address[] memory permissions = new address[](len);

        uint tmp;
        uint result;
        bytes4 queryPermissionsHash = 0x46f02832;

        // permissions = roleContract.querypermissions();
        assembly {
            // Free memory pointer
            let ptr := mload(0x40)
            // Function signature
            mstore(ptr, queryPermissionsHash)
            result := call(sub(gas, 10000), _role, 0, ptr, 0x4, ptr, mul(add(len, 0x2), 0x20))
            // TODO Why not work: remix not support returndatacopy
            // returndatacopy(permissions, 0, returndatasize)
            switch eq(result, 0)
            case 1 {
                revert(ptr, 0)
            }
        }

        for (uint i = 0; i<len; i++) {
            assembly {
                let ptr := mload(0x40)
                ptr := add(ptr, 0x40)
                tmp := mload(add(ptr,mul(i, 0x20)))
            }
            permissions[i] = address(tmp);
        }

        return permissions;
    }

    /// @notice Query the roles of the account
    /// @param _account The account to be queried
    /// @return The roles of the account
    function queryRoles(address _account)
        public
        constant
        returns (address[])
    {
        return roles[_account];
    }

    /// @notice Query the accounts that have the role
    /// @param _roleId The role to be queried
    /// @return The accounts that have the role
    function queryAccounts(address _roleId)
        public
        constant
        returns (address[])
    {
        return accounts[_roleId];
    }

    /// @notice Private: cancelRole
    function _cancelRole(address _account, address _role)
        private
        returns (bool)
    {
        assert(AddressArray.remove(_account, accounts[_role]));
        assert(AddressArray.remove(_role, roles[_account]));

        // Cancel role permissions of account.
        require(_cancelRolePermissions(_account, _role));

        RoleCanceled(_account, _role);
        return true;
    }

    /// @notice Private: cancel role of account
    function _cancelRolePermissions(address _account, address _role)
        private
        returns (bool)
    {
        address[] memory permissions = queryPermissions(_role);
        require(_cancelPermissions(_account, permissions));
        return true;
    }

    /// @notice Private: cancel permissions of account
    function _cancelPermissions(address _account, address[] _permissions)
        private
        returns (bool)
    {
        for (uint i = 0; i<_permissions.length; i++) {
            // Cancel this permission when account has not it in any of his other roles
            if (!hasPermission(_account, _permissions[i]))
                require(pmContract.cancelAuthorization(_account, _permissions[i]));
        }

        return true;
    }

    /// @notice Private: account has permission in one of his roles
    function hasPermission(address _account, address _permission)
        private
        constant
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
        address[] memory permissions = queryPermissions(_role);
        require(_setPermissions(_account, permissions));
        return true;
    }

    /// @notice Private: set permissions of account
    function _setPermissions(address _account, address[] _permissions)
        private
        returns (bool)
    {
        for (uint i = 0; i<_permissions.length; i++)
            require(pmContract.setAuthorization(_account, _permissions[i]));

        return true;
    }
}
