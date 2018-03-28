pragma solidity ^0.4.18;

import "./role_creator.sol";


/// @notice TODO Split to a new file: role_auth.sol
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

    function newRole(bytes32 _name, address[] _permissions)
        external 
        returns (address roleid)
    {
        return roleCreator.createRole(_name, _permissions);
    }

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

    function updateRoleName(address _roleid, bytes32 _name)
        external 
        returns (bool)
    {
        Role roleContract = Role(_roleid);
        return roleContract.updateName(_name);
    }

    function addPermissions(address _roleid, address[] _permissions)
        external 
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
        external 
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
        external 
        returns (bool)
    {

        if (!inAddressArray(_role, roles[_account]))
            roles[_account].push(_role);
        if (!inAddressArray(_account, accounts[_role]))
            accounts[_role].push(_account);

        // Set role permissions to account.
        require(_setPermissions(_account, _role));

        RoleSetted(_account, _role);
        return true;
    }

    function cancelRole(address _account, address _role)
        external 
        returns (bool)
    {
        return _cancelRole(_account, _role);
    }

    function clearRole(address _account)
        external 
        returns (bool)
    {
        // clear account and roles
        for (uint i = 0; i < roles[_account].length; i++) {
            // Clear account auth
            require(_cancelPermissions(_account, roles[_account][i]));
            // clear _account in all roles array.
            assert(addressDelete(_account, accounts[roles[_account][i]]));
        }

        // clear all roles associate with _account
        delete roles[_account];
        RoleCleared(_account);

        return true;
    }

    /// @dev Query the permissions of the role
    function queryPermissions(address _role)
        public 
        returns (address[])
    {
        require(isContract(_role));
        Role roleContract = Role(_role);
        uint len = roleContract.lengthOfPermissions();
        address[] memory permissions = new address[](len);

        uint tmp;
        uint result;
        bytes4 queryPermissionsHash = 0x46f02832;
        
        // permissions = roleContract.querypermissions();
        assembly {
            // free memory pointer
            let ptr := mload(0x40)
            // function signature 
            mstore(ptr, queryPermissionsHash)
            result := call(sub(gas, 10000), _role, 0, ptr, 0x4, ptr, mul(add(len, 0x2), 0x20))
            // TODO why not work: remix not support returndatacopy
            // returndatacopy(permissions, 0, returndatasize)
            if eq(result, 0) { revert(ptr, 0) }
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

    /// @dev Check the duplicate address
    function inAddressArray(address _value, address[] storage _array)
        private
        view
        returns (bool)
    {
        // Have found the value in array
        for (uint i = 0; i < _array.length; i++) {
            if (_value == _array[i])
                return true;
        }
        // Not in
        return false;
    }

    /// @dev Private: cancelRole
    function _cancelRole(address _account, address _role)
        private 
        returns (bool)
    {
        assert(addressDelete(_account, accounts[_role]));
        assert(addressDelete(_role, roles[_account]));

        // Cancel role permissions of account.
        require(_cancelPermissions(_account, _role));

        RoleCanceled(_account, _role);
        return true;
    }

    /// @dev Private: cancel permissions of role
    function _cancelPermissions(address _account, address _role)
        private
        returns (bool)
    {
        address[] memory permissions = queryPermissions(_role);
        for (uint i = 0; i<permissions.length; i++) {
            // Cancel this permission when account has not it in any of his other roles
            if (!hasPermission(_account, permissions[i]))
                require(pmContract.cancelAuthorization(_account, permissions[i]));
        }
        return true;
    }

    /// @dev Private: account has permission in one of his roles
    function hasPermission(address _account, address _permission)
        private
        returns (bool)
    {
        for (uint i = 0; i < roles[_account].length; i++) {
            Role roleContract = Role(roles[_account][i]);
            if (roleContract.inPermissions(_permission))
                return true;
        }
    }

    /// @dev Private: set role permissions of account
    function _setPermissions(address _account, address _role)
        private
        returns (bool)
    {
        address[] memory permissions = queryPermissions(_role);

        for (uint i = 0; i<permissions.length; i++) {
            require(pmContract.setAuthorization(_account, permissions[i]));
        }

        return true;
    }

    /// @dev Check an address is contract address
    /// @notice TODO be a library and used by other contract
    function isContract(address _target)
        private
        view
        returns (bool)
    {
        uint size;
        assembly { size := extcodesize(_target) }
        return size > 0;
    }
}
