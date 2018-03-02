pragma solidity ^0.4.18;

import "./role_creator.sol";


/// @notice Split to a new file: role_auth.sol
contract RoleManagement {

    address roleCreatorAddress = 0xe9E2593C7D1Db5EE843c143E9cB52b8d996b2380;
    RoleCreator roleCreator = RoleCreator(roleCreatorAddress);

    address internal roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;
    address internal permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    address internal authorizationAddr = 0x00000000000000000000000000000000013241b4;

    PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
    Authorization authContract = Authorization(authorizationAddr);

    mapping(address => address[]) internal accounts;
    mapping(address => address[]) internal roles;

    event RoleSetted(address indexed _account, address indexed _role);
    event RoleCanceled(address indexed _account, address indexed _role);
    event RoleCleared(address indexed _account);

    function newRole(bytes32 _name, address[] _permissions)
        public
        returns (address roleid)
    {
        return roleCreator.createRole(_name, _permissions);
    }

    function deleteRole(address _roleid)
        public
        returns (bool)
    {
        // Cancel the role of the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++)
            assert(cancelRole(accounts[_roleid][i], _roleid));

        Role roleContract = Role(_roleid);
        require(roleContract.deleteRole());

        return true;
    }

    function updateRoleName(address _roleid, bytes32 _name)
        public
        returns (bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.updateName(_name);
    }

    function addPermissions(address _roleid, address[] _permissions)
        public
        returns (bool)
    {
        // Set the authorization of all the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++) {
            for (uint j = 0; j < _permissions.length; j++)
                require(pmContract.setAuthorization(accounts[_roleid][i], _permissions[j]));
        }

        Role roleContract = Role(_roleid);
        require(roleContract.addPermissions(_permissions));
        return true;
    }

    function deletePermissions(address _roleid, address[] _permissions)
        public
        returns (bool)
    {
        // Cancel the authorization of all the account's which has the role
        for (uint i = 0; i < accounts[_roleid].length; i++) {
            for (uint j = 0; j < _permissions.length; j++)
                require(pmContract.cancelAuthorization(accounts[_roleid][i], _permissions[j]));
        }

        Role roleContract = Role(_roleid);
        require(roleContract.deletePermissions(_permissions));
        return true;
    }

    function setRole(address _account, address _role)
        public
        returns (bool)
    {
        accounts[_role].push(_account);
        roles[_account].push(_role);

        // Apply role permissions to account.
        Role roleContract = Role(_role);
        require(roleContract.applyRolePermissionsOf(_account));

        RoleSetted(_account, _role);
        return true;
    }

    function cancelRole(address _account, address _role)
        public
        returns (bool)
    {
        // Cancel role permissions of account.
        Role roleContract = Role(_role);
        require(roleContract.cancelRolePermissionsOf(_account));

        assert(addressDelete(_account, accounts[_role]));
        assert(addressDelete(_role, roles[_account]));

        RoleCanceled(_account, _role);
        return true;
    }

    function clearRole(address _account)
        public
        returns (bool)
    {
        // clear account and roles
        var _roles = roles[_account];
        for (uint i = 0; i < _roles.length; i++) {
            // Clear account auth
            Role roleContract = Role(_roles[i]);
            require(roleContract.cancelRolePermissionsOf(_account));
            // clear _account in all roles array.
            var _accounts = accounts[_roles[i]];
            assert(addressDelete(_account, _accounts));
        }

        // clear all roles associate with _account
        delete roles[_account];
        RoleCleared(_account);

        return true;
    }

    function queryRoles(address _account)
        public
        view
        returns (address[])
    {
        return roles[_account];
    }

    function queryAccounts(address _roleId)
        public
        view
        returns (address[])
    {
        return accounts[_roleId];
    }

    /// private functions
    function itemIndexOf(address item, address[] storage items)
        private
        view
        returns (uint i)
    {
        for (i = 0; i < items.length; i++) {
            if (item == items[i]) {
                return i;
            }
        }
    }

    function addressDelete(address item, address[] storage items)
        private
        returns (bool)
    {
        var index = itemIndexOf(item, items);

        if (index >= items.length)
            return false;

        // Remove the gap
        for (uint i = index; i < items.length - 1; i++)
            items[i] = items[i + 1];

        // Also delete the last element
        delete items[items.length - 1];
        items.length--;

        return true;
    }
}
