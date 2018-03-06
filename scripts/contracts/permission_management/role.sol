pragma solidity ^0.4.18;

import "./permission_management.sol";


/// @notice TODO Move the interface about calling the permission management to the role management
///         TODO Add util library of operation about address
contract Role {

    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event PermissionsAdded(address[] _permissions);
    event PermissionsDeleted(address[] _permissions);
    event RoleCreated(bytes32 indexed _name, address[] _permissions);

    bytes32 name;
    address[] permissions;
    address internal permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    address internal roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;
    PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);

    modifier onlyRoleManagement {
        require(roleManagementAddr == msg.sender);
        _;
    }

    function Role(bytes32 _name, address[] _permissions)
        public
    {
        name = _name;
        permissions = _permissions;
        RoleCreated(_name, _permissions);
    }

    function deleteRole()
        public
        returns (bool)
    {
        close();
        return true;
    }

    function updateName(bytes32 _name)
        public
        returns (bool)
    {
        NameUpdated(name, _name);
        name = _name;
        return true;
    }

    function addPermissions(address[] _permissions)
        public
        returns (bool)
    {
        for (uint index = 0; index < _permissions.length; index++) {
            if (!inPermissions(_permissions[index]))
                permissions.push(_permissions[index]);
        }

        PermissionsAdded(_permissions);
        return true;
    }

    function deletePermissions(address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            assert(removePermission(_permissions[i]));
        }

        PermissionsDeleted(_permissions);
        return true;
    }

    function applyRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < permissions.length; i++) {
            require(pmContract.setAuthorization(_account, permissions[i]));
        }

        return true;
    }

    function cancelRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < permissions.length; i++) {
            require(pmContract.cancelAuthorization(_account, permissions[i]));
        }

        return true;
    }

    function clearRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        require(pmContract.clearAuthorization(_account));
        return true;
    }
    
    function queryRole()
        public
        view
        returns (bytes32, address[])
    {
        return (name, permissions);
    }

    function queryName()
        public
        view
        returns (bytes32)
    {
        return name;
    }

    function queryPermissions()
        public
        view
        returns (address[])
    {
        return permissions;
    }

    /// @notice private
    function close() private onlyRoleManagement
    {
        selfdestruct(msg.sender);
    }

    function indexOf(address permission)
        private
        view
        returns (uint i)
    {
        for (i = 0; i < permissions.length; i++) {
            if (permission == permissions[i]) {
                return i;
            }
        }
    }

    function removePermission(address permission)
        private
        returns (bool)
    {
        var index = indexOf(permission);

        if (index >= permissions.length)
            return false;

        // Remove the gap
        for (uint i = index; i < permissions.length - 1; i++)
            permissions[i] = permissions[i + 1];

        // Also delete the last element
        delete permissions[permissions.length - 1];
        permissions.length--;

        return true;
    }

    /// @dev Check the duplicate permission
    function inPermissions(address _permission)
        private
        view
        returns (bool)
    {
        for (uint i = 0; i < permissions.length; i++) {
            if (_permission == permissions[i])
                return true;
        }

        return false;
    }
}
