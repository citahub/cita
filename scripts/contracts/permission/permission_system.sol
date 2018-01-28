pragma solidity ^0.4.18;

import "./permission_sys_interface.sol";
import "./permission_check.sol";

/// @notice TODO: Multiple routers
///             : More modifier
///             : Mark resource
///             : Role tree(TBD)
///             : Change bytes32 to uint8 of the permissions using enum and array
///             : Check the adding permissions is valid
///             : Add owner field in the group, role and authorization
///             : Make the scope be a separate structure
///        DOING: Use id as the key of the map(TBD)
/// @title Permission system of CITA
contract PermissionSystem is PermissionSysInterface, PermissionCheck {

    /// @dev Setup
    /// @param _superAdmin The address of superAdmin
    /// @param _adamEve Address as Adam and Eve of the permission system world.
    ///                 These address will have the basic permission
    function PermissionSystem(address _superAdmin, address[] _adamEve) public {
        // User can give up it by setting address 0x0
        if (address(0x0) != _superAdmin)
            superAdmin = _superAdmin;
        else if (!Util.addressArrayNul(_adamEve))
            // Call the genesis
            genesis(_adamEve);
        else
            revert();
    }

    /// @dev Init group only by superAdmin
    function initGroup(bytes32 _root, address[] _adamEve, bool _subSwitch)
        public
        onlySuperAdmin
        groupNul(_root)
        returns (bool)
    {
        if (GroupManager.initGroup(groups, _root, _adamEve, _subSwitch)) {
            GroupInited(_root, _adamEve, _subSwitch);
            group_names.push(_root);
            return true;
        }
    }

    /// @dev Init role only by superAdmin
    function initRole(bytes32 _basic, bytes32[] _permissions)
        public
        onlySuperAdmin
        roleNul(_basic)
        returns (bool)
    {
        if (RoleManager.initRole(roles, _basic, _permissions)) {
            RoleInited(_basic, _permissions);
            role_names.push(_basic);
            return true;
        }
    }

    /// @dev Init authorization only by superAdmin
    function initAuthorization(bytes32 _group, bytes32 _role)
        public
        onlySuperAdmin
        authNul(_group, _role)
        returns (bool)
    {
        AuthorizationInited(_group, _role);
        auth.role_groups[_role].push( _role);
        auth.group_roles[_group].push(_group);
        return true;
    }

    /// @dev Apply to join the group
    /// @notice TODO: check the SendTx permission.
    function applyGroup(bytes32 _group) public returns (bool) {
        group_applicants[_group].push(msg.sender);
        GroupApplied(msg.sender, _group);
        return true;
    }

    /// @dev Approve the application of joining the _group
    function approveApply(bytes32 _group, bytes32 _resource, bytes32 _role)
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        ApplyApproved(msg.sender, _group, _resource, _role);
        GroupManager.addUsers(groups, _group, group_applicants[_group]);
    }

    /// @dev Grant the role to users
    function grantRole(bytes32 _group, bytes32 _resource, bytes32 _role, address[] _users)
        public
        // Default resourceGroup is userGroup
        can(msg.sender, _group, _group, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        RoleGranted(_group, _resource, _role, _users);
        GroupManager.addUsers(groups, _resource, _users);
    }

    /// @dev Revoke the users's role
    function revokeRole(bytes32 _group, bytes32 _resource, bytes32 _role, address[] _users)
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        RoleRevoked(_group, _resource, _role, _users);
        GroupManager.deleteUsers(groups, _group, _users);
    }

    /// @dev Apply to quit the group
    function quitGroup(bytes32 _group)
        public
        userInGroup(msg.sender, _group)
        returns (bool)
    {
        GroupQuitted(msg.sender, _group);
        return Util.addressDelete(msg.sender, group_resignations[_group]);
    }

    /// @dev Approve the application of quitting the group
    /// @notice Msg.sender should belong to the father node
    function approveQuit(bytes32 _group, bytes32 _resource, bytes32 _role)
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        QuitApproved(msg.sender, _group, _resource, _role);
        return GroupManager.deleteUsers(groups, _group, group_resignations[_group]);
    }

    /// @dev New a group
    /// @param _group The group of the caller
    ///        _newName The name of new group
    function newGroup(
        bytes32 _group,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        uint8 _op,
        bytes32 _role,
        string _profile
    )
        public
        nameNotExist(_newName, group_names)
        can(msg.sender, _group, _newName, _role, bytes32("CreateGroup"))
        returns (bool)
    {
        GroupNewed(msg.sender, _group, _newName);
        return _newGroup(_group, _newName, _newUsers, _newSubSwitch, _op, _profile);
    }

    /// @dev Delete group
    /// @notice Delete a tree's node. Only leaf node
    /// @param _resource The resource group to be deleted
    function deleteGroup(bytes32 _group, bytes32 _resource, bytes32 _role)
        public
        nameExist(_resource, group_names)
        can(msg.sender, _group, _resource, _role, bytes32("DeleteGroup"))
        leafNode(_resource)
        returns (bool)
    {
        // Delete the name in group
        if (GroupManager.deleteGroup(groups, _resource)) {
            // Delete the name in authorization
            if (AuthorizationManager.deleteGroup(auth, _resource)) 
                // Delete the name in group_names
                return Util.bytes32Delete(_resource, group_names);
        }
    }

    /// @dev Modify the group name
    function modifyGroupName(bytes32 _oldName, bytes32 _newName, bytes32 _resource, bytes32 _role)
        public
        nameNotExist(_newName, group_names)
        can(msg.sender, _oldName, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        return _modifyGroupName(_oldName, _newName);
    }

    /// @dev Modify the subSwitch of group
    function modifySubSwitch(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        bool _newSubSwitch
    )
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        return GroupManager.modifySubSwitch(groups, _group, _newSubSwitch);
    }

    /// @dev Modify the profile of group
    function modifyProfile(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        string _newProfile
    )
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateGroup"))
        returns (bool)
    {
        return GroupManager.modifyProfile(groups, _group, _newProfile);
    }

    /// @dev New a role
    /// @param _group The group of the caller
    /// @param _role The role of the caller
    function newRole(
        bytes32 _group,
        bytes32 _newName,
        bytes32 _role,
        bytes32[] _newPermissions,
        uint8 _op
    )
        public
        nameNotExist(_newName, role_names)
        permissionsInRole(_newPermissions, _role)
        can(msg.sender, _group, _group, _role, bytes32("CreateRole"))
        returns (bool)
    {
        return _newRole(_group, _newName,  _newPermissions, _op);
    }

    /// @dev Delete role
    function deleteRole(bytes32 _role, bytes32 _group, bytes32 _resource)
        public
        nameExist(_role, role_names)
        can(msg.sender, _group, _resource, _role, bytes32("DeleteRole"))
        returns (bool)
    {
        // Delete the name in role
        if (RoleManager.deleteRole(roles, _role)) {
            // Delete the name in authorization
            if (AuthorizationManager.deleteRole(auth, _role))
                // Delete the name in role_names
                return Util.bytes32Delete(_role, role_names);
        }
    }

    /// @dev Modify the role name
    function modifyRoleName(bytes32 _oldName, bytes32 _newName, bytes32 _group, bytes32 _resource)
        public
        nameNotExist(_newName, role_names)
        can(msg.sender, _group, _resource, _oldName, bytes32("UpdateRole"))
        returns (bool)
    {
        return _modifyRoleName(_oldName, _newName);
    }

    /// @dev Add permissions of the role
    function addPermissions(bytes32 _role, bytes32[] _permissions, bytes32 _group, bytes32 _resource)
        public
        nameExist(_role, role_names)
        can(msg.sender, _group, _resource, _role, bytes32("UpdateRole"))
        returns (bool)
    {
        return RoleManager.addPermissions(roles, _role, _permissions);
    }

    /// @dev Delete permissions of the role
    function deletePermissions(bytes32 _role, bytes32[] _permissions, bytes32 _group, bytes32 _resource)
        public
        can(msg.sender, _group, _resource, _role, bytes32("UpdateRole"))
        returns (bool)
    {
        return RoleManager.deletePermissions(roles, _role, _permissions);
    }

    /// @dev Set authorization
    function setAuthorization(bytes32 _group, bytes32 _role, bytes32 _resource)
        public
        can(msg.sender, _group, _resource, _role, bytes32("CreateAuth"))
        returns(bool)
    {
        return AuthorizationManager.setAuthorization(auth, _group, _role);
    }

    /// @dev Cancel authorization
    function cancelAuthorization(bytes32 _group, bytes32 _role, bytes32 _resource)
        public
        can(msg.sender, _group, _resource, _role, bytes32("DeteleAuth"))
        returns(bool)
    {
        return AuthorizationManager.cancelAuthorization(auth, _group, _role);
    }

    /// @dev Query the roles of the group
    function queryRoles(bytes32 _group)
        view
        public
        returns (bytes32[])
    {
        return auth.group_roles[_group];
    }

    /// @dev Query the permissions of the role
    function queryPermissions(bytes32 _role)
        view
        public
        returns (bytes32[])
    {
        return roles.roles[_role].permissions;
    }

    /// @dev Query the groups of the user
    function queryGroups(address _user)
        view
        public
        returns (bytes32[])
    {
        return user_groups[_user];
    }

    /// @dev Query the information of the group
    function queryGroupInfo(bytes32 _group)
        view
        public
        returns (address[], bytes32[], bool, bytes32, string)
    {
        return (groups.groups[_group].users,
                groups.groups[_group].subGroups,
                groups.groups[_group].subSwitch,
                groups.groups[_group].parentGroup,
                groups.groups[_group].profile
               );
    }
    
    /// @dev Query the ancestors of the group
    function queryAncestors(bytes32 _group)
        view
        public
        returns (bytes32[])
    {
        bytes32[] memory ancestors;

        for (uint i=0; _group != bytes32(0x0); i++) {
            _group = groups.groups[_group].parentGroup;
            ancestors[i] = _group;
        }
        
        return ancestors;
    }

    /// @dev Query the users of the group
    function queryUsers(bytes32 _group)
        view
        public
        returns (address[])
    {
        return groups.groups[_group].users;
    }

    /// @dev Query the subGroups of the group
    function querySubGroups(bytes32 _group)
        view
        public
        returns (bytes32[])
    {
        return groups.groups[_group].subGroups;
    }

    /// @dev Query the subSwitch of the group
    function querySubSwitch(bytes32 _group)
        view
        public
        returns (bool)
    {
        return groups.groups[_group].subSwitch;
    }

    /// @dev Query the parentGroup of the group
    function queryParentGroups(bytes32 _group)
        view
        public
        returns (bytes32)
    {
        return groups.groups[_group].parentGroup;
    }

    /// @dev Query the profie of group
    function queryProfile(bytes32 _group)
        view
        public
        returns (string)
    {
        return groups.groups[_group].profile;
    }

    /// @dev Query all groups
    function queryAllGroups()
        view
        public
        returns (bytes32[])
    {
        return group_names;
    }

    /// @dev Query all roles
    function queryAllRoles()
        view
        public
        returns (bytes32[])
    {
        return role_names;
    }

    /// @dev Query superAdmin
    function querySuperAdmin()
        view
        public
        returns (address)
    {
        return superAdmin;
    }

    /// @dev Query basic permission
    function queryBasicPermission()
        view
        public
        returns (bytes32[])
    {
        return _per_basic;
    }

    /// @dev Check the permission
    function checkPermission(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    )
        view
        public
        can(_user, _userGroup, _resourceGroup, _role, _permission)
        returns (bool)
    {
        return true;
    }

    // Create the genensis: group, role and authorization when the superAdmin is nul
    function genesis(address[] _adamEve)
        private
        returns (bool)
    {
        // Init the group.
        if (_initGroup(bytes32("root"), _adamEve)) {
            // Init the role
            if (_initRole(bytes32("basic")))
                // Init the authorization
                return _initAuthorization(bytes32("root"), bytes32("basic"));
        }
    }

    /// @dev Init group: root
    function _initGroup(bytes32 _root, address[] _adamEve)
        private
        returns (bool)
    {
        // Set subSwitch false by default
        if (GroupManager.initGroup(groups, _root, _adamEve, false)) {
            group_names.push(_root);
            return true;
        }
    }

    /// @dev Init role: basic
    function _initRole(bytes32 _basic)
        private
        returns (bool)
    {
        _per_basic.push(bytes32("SendTx"));
        _per_basic.push(bytes32("CreateGroup"));
        _per_basic.push(bytes32("CreateRole"));
        _per_basic.push(bytes32("CreateAuth"));

        if (RoleManager.initRole(roles, _basic, _per_basic)) {
            role_names.push(_basic);
            return true;
        }
    }

    /// @dev Init gengesis's authorization: root group and basic role
    function _initAuthorization(bytes32 _group, bytes32 _role)
        private
        returns (bool)
    {
        auth.role_groups[_role].push( _role);
        auth.group_roles[_group].push(_group);
        return true;
    }

    /// @dev New a role
    function _newRole(
        bytes32 _group,
        bytes32 _newName,
        bytes32[] _newPermissions,
        uint8 _op
    )
        private
        returns (bool)
    {
        if (RoleManager.newRole(roles, _group, _newName, _newPermissions, Util.SetOp(_op))) {
            role_names.push(_newName);
            return true;
        }
    }

    /// @dev New a group
    function _newGroup(
        bytes32 _group,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        uint8 _op,
        string _profile
    )
        private
        returns (bool)
    {
        if (GroupManager.newGroup(groups, _group, _newName, _newUsers, _newSubSwitch, Util.SetOp(_op), _profile)) {
            for (uint i = 0; i< _newUsers.length; i++)
                user_groups[_newUsers[i]].push(_newName);

            group_names.push(_newName);
            return true;
        }
    }

    function _modifyGroupName(bytes32 _oldName, bytes32 _newName)
        private
        returns (bool)
    {
        // Change the name in user_groups
        for (uint i=0; i < groups.groups[_oldName].users.length; i++) {
            Util.bytes32Replace(_oldName, _newName, user_groups[groups.groups[_oldName].users[i]]);
        }
            
        // Change the name in group
        if (GroupManager.modifyName(groups, _oldName, _newName)) {
            // Change the name in authorization
            if (AuthorizationManager.replaceGroup(auth, _oldName, _newName))
                // Change the name in group_names
                return Util.bytes32Replace(_oldName, _newName, group_names);
        }
            
    }

    function _modifyRoleName(bytes32 _oldName, bytes32 _newName)
        private
        returns (bool)
    {
        // Change the name in role
        if (RoleManager.modifyName(roles, _oldName, _newName)) {
            // Change the name in authorization
            if (AuthorizationManager.replaceRole(auth, _oldName, _newName))
                // Change the name in role_names
                return Util.bytes32Replace(_oldName, _newName, role_names);
        }
    }
}
