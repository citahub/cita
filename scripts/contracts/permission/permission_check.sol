pragma solidity ^0.4.14;

import "./permission_data.sol";

/// @title The modifier of the permission system
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
        // Why not work??
        // require(Util.bytes32InArray(_resourceGroup, GroupManager.querySubGroups(groups, _userGroup)));
        _; 
    }

    /// @dev Resource group equal user group.
    modifier resourceBelongGroup(bytes32 _resourceGroup, bytes32 _userGroup) {
        if (groups.groups[_userGroup].subSwitch)
            // Scope switch is on
            require(Util.bytes32InArray(_resourceGroup, groups.groups[_userGroup].subGroups));
        else
            // Scope switch is off
            require(_resourceGroup == _userGroup);
        _; 
    }

    modifier nameNotExist(bytes32 _name, bytes32[] _names) {
        require(!Util.bytes32InArray(_name, _names)); 
        _;
    }

    modifier nameExist(bytes32 _name, bytes32[] _names) {
        require(Util.bytes32InArray(_name, _names)); 
        _;
    }

    modifier leafNode(bytes32 _name) {
        require(Util.bytes32ArrayNul(groups.groups[_name].subGroups));
        _;
    }

    modifier onlySuperAdmin {
        require(msg.sender == superAdmin);
        _;
    }

    modifier groupNul(bytes32 _name) {
        require(bytes32(0x0) == groups.groups[_name].name); 
        require(Util.addressArrayNul(groups.groups[_name].users));
        require(Util.bytes32ArrayNul(groups.groups[_name].subGroups));
        require(false == groups.groups[_name].subSwitch); 
        require(Util.bytes32ArrayNul(auth.group_roles[_name]));
        _;
    }

    modifier roleNul(bytes32 _name) {
        require(bytes32(0x0) == roles.roles[_name].name); 
        require(Util.bytes32ArrayNul(roles.roles[_name].permissions));
        require(Util.bytes32ArrayNul(auth.role_groups[_name]));
        _; 
    }

    modifier authNul(bytes32 _group, bytes32 _role) {
        require(Util.bytes32ArrayNul(auth.role_groups[_role])); 
        require(Util.bytes32ArrayNul(auth.group_roles[_group])); 
        _;
    }

    modifier can(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    ) {
        require(check(_user, _userGroup, _resourceGroup, _role , _permission)); 
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
        constant 
        userInGroup(_user, _userGroup)
        groupHasRole(_userGroup, _role)
        permissionInRole(_permission, _role)
        resourceBelongGroup(_resourceGroup, _userGroup)
        returns (bool) 
    {
        return true;
    }
}
