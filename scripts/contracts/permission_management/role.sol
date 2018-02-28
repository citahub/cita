pragma solidity ^0.4.18;

import "./permission_management.sol";


contract Role {

    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event PermissionsAdded(address[] _permissions);
    event PermissionsDeleted(address[] _permissions);
    event RoleCreated(bytes32 indexed _name, address[] _permissions);

    bytes32 name;
    address[] permissions;
    address internal permissionManagementAddr = 0x00000000000000000000000000000000013241b2;
    address internal roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;

    modifier onlyRoleManagement {
        require(roleManagementAddr == msg.sender);
        _;
    }

    modifier notSame(bytes32 _name) {
        require(name != _name);
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
        notSame(_name)
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
            removePermission(_permissions[i]);
        }

        PermissionsDeleted(_permissions);
        return true;
    }

    function applyRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        for (uint i = 0; i < permissions.length; i++) {
            pmContract.setAuthorization(_account, permissions[i]);
        }

        return true;
    }

    function cancelRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        for (uint i = 0; i < permissions.length; i++) {
            pmContract.cancelAuthorization(_account, permissions[i]);
        }

        return true;
    }

    function clearRolePermissionsOf(address _account)
        public
        onlyRoleManagement
        returns (bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        return pmContract.clearAuthorization(_account);
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
}
