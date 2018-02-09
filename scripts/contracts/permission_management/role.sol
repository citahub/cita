pragma solidity ^0.4.18;
import "./permission_management.sol";


contract Role {

    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event PermissionsAdded(address[] _permissions);
    event PermissionsDeleted(address[] _permissions);

    bytes32 internal name;
    address[] internal permissions;

    // TODO: repalce test address with deployed address.
    address internal permissionManagementAddr = 0x619F9ab1672eeD2628bFeC65AA392FD48994A433;

    modifier onlyPermissionManagement {
        require(permissionManagementAddr == msg.sender);
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
        returns(bool)
    {
        NameUpdated(name, _name);
        name = _name;
        return true;
    }

    function addPermissions(address[] _permissions)
        public
        returns(bool)
    {
        for (uint index = 0; index < _permissions.length; index++) {
            permissions.push(_permissions[index]);
        }

        PermissionsAdded(_permissions);
        return true;
    }

    function deletePermissions(address[] _permissions)
        public
        returns(bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            removePermission(_permissions[i]);
        }

        PermissionsDeleted(_permissions);
        return true;
    }

    function queryRole()
        public
        view
        returns (bytes32, address[]) 
    {
        return (name, permissions);
    } 

    function applyRolePermissionsOf(address _account)
        public
        onlyPermissionManagement
        returns(bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        for (uint i = 0; i < permissions.length; i++) {
            pmContract.setAuthorization(_account, permissions[i]);
        }

        return true;
    }

    function cancelRolePermissionsOf(address _account)
        public
        onlyPermissionManagement
        returns(bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        for (uint i = 0; i < permissions.length; i++) {
            pmContract.cancelAuthorization(_account, permissions[i]);
        }

        return true;
    }

    function clearRolePermissionsOf(address _account)
        public
        onlyPermissionManagement
        returns (bool)
    {
        PermissionManagement pmContract = PermissionManagement(permissionManagementAddr);
        for (uint i = 0; i < permissions.length; i++) {
            pmContract.clearAuthorization(_account);
        }

        return true;
    }

    /// private
    function close() private onlyPermissionManagement
    {
        selfdestruct(msg.sender);
    }

    function indexOf(address permission)
        private
        view
        returns(uint i)
    {
        for (i = 0; i < permissions.length; i++) {
            if (permission == permissions[i]) {
                return i;
            }
        }
    }

    function removePermission(address permission)
        private
        returns(bool)
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
