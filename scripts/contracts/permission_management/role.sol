pragma solidity ^0.4.18;

import "./address_array.sol";


/// @notice TODO Move the interface about calling the permission management to the role management
///         TODO Add util library of operation about address
contract Role {

    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event PermissionsAdded(address[] _permissions);
    event PermissionsDeleted(address[] _permissions);
    event RoleCreated(bytes32 indexed _name, address[] _permissions);

    bytes32 name;
    address[] permissions;
    address internal roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;

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
        onlyRoleManagement
        returns (bool)
    {
        close();
        return true;
    }

    function updateName(bytes32 _name)
        public
        onlyRoleManagement
        returns (bool)
    {
        NameUpdated(name, _name);
        name = _name;
        return true;
    }

    function addPermissions(address[] _permissions)
        public
        onlyRoleManagement
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
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            assert(AddressArray.remove(_permissions[i], permissions));
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

    /// @dev Query the length of the permissions
    function lengthOfPermissions()
        public
        view
        returns (uint)
    {
        return permissions.length; 
    }

    /// @dev Check the duplicate permission
    function inPermissions(address _permission)
        public 
        view
        returns (bool)
    {
        return AddressArray.exist(_permission, permissions);
    }

    /// @notice private
    function close() private onlyRoleManagement
    {
        selfdestruct(msg.sender);
    }
}
