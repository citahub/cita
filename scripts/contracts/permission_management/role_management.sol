pragma solidity ^0.4.18;

import "./role.sol";
import "./role_creator.sol";


contract RoleManagement {
    event RoleSetted(address indexed _account, address indexed _role);
    event RoleCanceled(address indexed _account, address indexed _role);
    event RoleCleared(address indexed _account);

    // TODO: replace role creator address with deployed address.
    address internal roleCreatorAddress = 0x619f9Ab1672EED2628BFec65AA392fD48994A430;

    mapping(address => address[]) internal roleAssignedToAccounts;
    mapping(address => address[]) internal accountHadRoles;

    function newRole(bytes32 _name, address[] _permissions) 
        public
        returns (address roleid) 
    {
        RoleCreator roleCreator = RoleCreator(roleCreatorAddress);
        address role = roleCreator.createRole(_name, _permissions);

        return role;
    }

    function deleteRole(address _roleid)
        public
        returns (bool) 
    {
        Role roleContract = Role(_roleid);
        roleContract.deleteRole();

        return true;
    }

    function updateRoleName(address _roleid, bytes32 _name)
        public
        returns(bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.updateName(_name);
    }

    function addPermissions(address _roleid, address[] _permissions)
        public
        returns(bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.addPermissions(_permissions);
    }

    function deletePermissions(address _roleid, address[] _permissions)
        public
        returns(bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.deletePermissions(_permissions);
    }

    function setRole(address _account, address _role)
        public
        returns(bool)
    {
        roleAssignedToAccounts[_role].push(_account);
        accountHadRoles[_account].push(_role);

        // Apply role permissions to account.
        Role roleContract = Role(_role);
        roleContract.applyRolePermissionsOf(_account);

        RoleSetted(_account, _role);
        return true;
    }

    function cancelRole(address _account, address _role)
        public
        returns(bool)
    {
        // Cancel role permissions of account.
        Role roleContract = Role(_role);
        roleContract.cancelRolePermissionsOf(_account);

        removeAddressInArray(_account, roleAssignedToAccounts[_role]);
        removeAddressInArray(_role, accountHadRoles[_account]);

        RoleCanceled(_account, _role);
        return true;
    }

    function clearRole(address _account)
        public
        returns(bool)
    {
        // clear account and roles
        var roles = accountHadRoles[_account];
        for (uint i = 0; i < roles.length; i++) {
            // Clear account auth
            Role roleContract = Role(roles[i]);
            roleContract.cancelRolePermissionsOf(_account);
            // clear _account in all roles array.
            var accounts = roleAssignedToAccounts[roles[i]];
            removeAddressInArray(_account, accounts);
        }
 
        // clear all roles associate with _account
        delete accountHadRoles[_account];
        RoleCleared(_account);

        return true;
    }

    /// private functions
    function itemIndexOf(address item, address[] storage items)
        private
        view
        returns(uint i)
    {
        for (i = 0; i < items.length; i++) {
            if (item == items[i]) {
                return i;
            }
        }
    }

    function removeAddressInArray(address item, address[] storage items)
        private
        returns(bool)
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
