pragma solidity ^0.4.18;

import "./permission_data.sol";

/// @title The authentication of the permission system
contract PermissionCheck is PermissionData {

    /// @dev User in group
    modifier userInGroup(address _user, bytes32 _group) {
        if (_user != superAdmin)
            require(Util.addressInArray(_user, groups.groups[_group].users));
        _;
    }

    /// @dev Permission in role
    modifier permissionInRole(bytes32 _permission, bytes32 _role) {
        require(Util.bytes32InArray(_permission, roles.roles[_role].permissions));
        _;
    }

    /// @dev Permissions in role
    modifier permissionsInRole(bytes32[] _permissions, bytes32 _role) {
        if (bytes32(0x0) != _role)
            require(Util.bytes32SubSet(_permissions, roles.roles[_role].permissions));
        _;
    }

    /// @dev Group has role
    modifier groupHasRole(bytes32 _group, bytes32 _role) {
        require(Util.bytes32InArray(_role, auth.group_roles[_group]));
        _;
    }

    /// @dev Resource group in zone of the user group
    /// @notice For scope. To check resourceGroup is userGroup's subGroup for now
    ///         TODO: A scope struct
    modifier resourceInZone(bytes32 _resourceGroup, bytes32 _userGroup) {
        // Need scope switch on
        require(groups.groups[_userGroup].subSwitch);
        require(Util.bytes32InArray(_resourceGroup, groups.groups[_userGroup].subGroups));
        _;
    }

    /// @dev Resource group belong user group.
    modifier resourceBelongGroup(bytes32 _resourceGroup, bytes32 _userGroup) {
        if (groups.groups[_userGroup].subSwitch)
            // Scope switch is on
            require(Util.bytes32InArray(_resourceGroup, groups.groups[_userGroup].subGroups));
        else
            // Scope switch is off
            require(_resourceGroup == _userGroup);
        _;
    }

    /// @dev Name does not exist
    modifier nameNotExist(bytes32 _name, bytes32[] _names) {
        require(!Util.bytes32InArray(_name, _names));
        _;
    }

    /// @dev Name exists
    modifier nameExist(bytes32 _name, bytes32[] _names) {
        require(Util.bytes32InArray(_name, _names));
        _;
    }

    /// @dev Leaf node
    modifier leafNode(bytes32 _group) {
        require(Util.bytes32ArrayNul(groups.groups[_group].subGroups));
        _;
    }

    /// @dev Only superAdmin
    modifier onlySuperAdmin {
        require(msg.sender == superAdmin);
        _;
    }

    /// @dev Group is null
    modifier groupNul(bytes32 _group) {
        require(bytes32(0x0) == groups.groups[_group].name);
        require(Util.addressArrayNul(groups.groups[_group].users));
        require(Util.bytes32ArrayNul(groups.groups[_group].subGroups));
        require(false == groups.groups[_group].subSwitch);
        require(Util.bytes32ArrayNul(auth.group_roles[_group]));
        _;
    }

    /// @dev Role is null
    modifier roleNul(bytes32 _role) {
        require(bytes32(0x0) == roles.roles[_role].name);
        require(Util.bytes32ArrayNul(roles.roles[_role].permissions));
        require(Util.bytes32ArrayNul(auth.role_groups[_role]));
        _;
    }

    /// @dev Authorization is null
    modifier authNul(bytes32 _group, bytes32 _role) {
        require(Util.bytes32ArrayNul(auth.role_groups[_role]));
        require(Util.bytes32ArrayNul(auth.group_roles[_group]));
        _;
    }

    /// @dev Can do
    modifier can(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    ) {
        if (_user != superAdmin)
            require(check(_user, _userGroup, _resourceGroup, _role, _permission));
        _;
    }

    /// @dev Check user group has the permission with the scope of the resource group
    /// @notice For external function calls
    function check(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    )
        view
        private
        userInGroup(_user, _userGroup)
        groupHasRole(_userGroup, _role)
        permissionInRole(_permission, _role)
        resourceBelongGroup(_resourceGroup, _userGroup)
        returns (bool)
    {
        return true;
    }
}
