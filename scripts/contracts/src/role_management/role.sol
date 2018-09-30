pragma solidity ^0.4.24;

import "../lib/address_array.sol";
import "../common/address.sol";


/// @title Role contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: Created by roleCreator
///         The interface can be called: Only query type
contract Role is ReservedAddress {

    event NameUpdated(bytes32 indexed _oldName, bytes32 indexed _newName);
    event PermissionsAdded(address[] _permissions);
    event PermissionsDeleted(address[] _permissions);
    event RoleCreated(bytes32 indexed _name, address[] _permissions);

    bytes32 name;
    address[] permissions;

    modifier onlyRoleManagement {
        require(roleManagementAddr == msg.sender, "permission denied.");
        _;
    }

    /// @notice Constructor
    constructor(bytes32 _name, address[] _permissions)
        public
    {
        name = _name;
        permissions = _permissions;
        emit RoleCreated(_name, _permissions);
    }

    /// @notice Delete the role
    /// @return true if successed, otherwise false
    function deleteRole()
        public
        onlyRoleManagement
    {
        close();
    }

    /// @notice Update role's name
    /// @param _name The new name of role
    /// @return true if successed, otherwise false
    function updateName(bytes32 _name)
        public
        onlyRoleManagement
        returns (bool)
    {
        emit NameUpdated(name, _name);
        name = _name;
        return true;
    }

    /// @notice Add permissions of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function addPermissions(address[] _permissions)
        public
        onlyRoleManagement
        returns (bool)
    {
        for (uint index = 0; index < _permissions.length; index++) {
            if (!inPermissions(_permissions[index]))
                permissions.push(_permissions[index]);
        }

        emit PermissionsAdded(_permissions);
        return true;
    }

    /// @notice Delete permissions of role
    /// @dev TODO Check permissions in role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function deletePermissions(address[] _permissions)
        public
        onlyRoleManagement
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            assert(AddressArray.remove(_permissions[i], permissions));
        }

        emit PermissionsDeleted(_permissions);
        return true;
    }

    /// @notice Query the information of the role
    /// @return The information of role: name and permissions
    function queryRole()
        public
        view
        returns (bytes32, address[])
    {
        return (name, permissions);
    }

    /// @notice Query the name of the role
    /// @return The name of role
    function queryName()
        public
        view
        returns (bytes32)
    {
        return name;
    }

    /// @notice Query the permissions of the role
    /// @return The permissions of role
    function queryPermissions()
        public
        view
        returns (address[])
    {
        return permissions;
    }

    /// @notice Query the length of the permissions
    /// @return The number of permission
    function lengthOfPermissions()
        public
        view
        returns (uint)
    {
        return permissions.length;
    }

    /// @notice Check the duplicate permission
    /// @return true if in permissions, otherwise false
    function inPermissions(address _permission)
        public
        view
        returns (bool)
    {
        return AddressArray.exist(_permission, permissions);
    }

    /// @notice Private selfdestruct
    function close() private onlyRoleManagement
    {
        selfdestruct(msg.sender);
    }
}
