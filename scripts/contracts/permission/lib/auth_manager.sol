pragma solidity ^0.4.18;

import "./util.sol";

/// @title Manage the authorization
library AuthorizationManager {

    using Util for *;

    struct Authorization {
        mapping(bytes32 => bytes32[]) role_groups;
        mapping(bytes32 => bytes32[]) group_roles;
    }

    event AuthorizationSetted(bytes32 indexed _group, bytes32 indexed _role);
    event AuthorizationCanceled(bytes32 indexed _group, bytes32 indexed _role);
    event GroupReplaced(bytes32 indexed _oldGroup, bytes32 indexed _newGroup);
    event RoleReplaced(bytes32 indexed _oldRole, bytes32 indexed _newRole);
    event GroupDeleted(bytes32 indexed _group);
    event RoleDeleted(bytes32 indexed _role);

    /// @dev Set authorization
    function setAuthorization(Authorization storage self, bytes32 _group, bytes32 _role)
        internal
        returns(bool)
    {
        self.role_groups[_role].push(_group);
        self.group_roles[_group].push(_role);
        AuthorizationSetted(_group, _role);
        return true;
    }

    /// @dev Cancel authorization
    function cancelAuthorization(Authorization storage self, bytes32 _group, bytes32 _role)
        internal
        returns(bool)
    {
        Util.bytes32Delete(_group, self.role_groups[_role]);
        Util.bytes32Delete(_role, self.group_roles[_group]);
        AuthorizationCanceled(_group, _role);
        return true;
    }

    /// @dev Replace the group name
    function replaceGroup(Authorization storage self, bytes32 _oldGroup, bytes32 _newGroup)
        internal
        returns(bool)
    {
        bytes32[] memory roles = self.group_roles[_oldGroup];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++) {
            var index = Util.bytes32Index(_oldGroup, self.role_groups[roles[i]]);

            // Not found
            if (index >= self.role_groups[roles[i]].length)
                return false;

            self.role_groups[roles[i]][index] = _newGroup;
        }

        // Change the group_roles
        self.group_roles[_newGroup] = roles;
        delete self.group_roles[_oldGroup];
        GroupReplaced(_oldGroup, _newGroup);
        return true;
    }

    /// @dev Replace the role name
    function replaceRole(Authorization storage self, bytes32 _oldRole, bytes32 _newRole)
        internal
        returns(bool)
    {
        bytes32[] memory groups = self.group_roles[_oldRole];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++) {
            var index = Util.bytes32Index(_oldRole, self.group_roles[groups[i]]);

            // Not found
            if (index >= self.group_roles[groups[i]].length)
                return false;

            self.group_roles[groups[i]][index] = _newRole;
        }

        // Change the role_groups
        self.role_groups[_newRole] = groups;
        delete self.role_groups[_oldRole];
        RoleReplaced(_oldRole, _newRole);
        return true;
    }

    /// @dev Delete the group
    function deleteGroup(Authorization storage self, bytes32 _group)
        internal
        returns (bool)
    {
        bytes32[] memory roles = self.group_roles[_group];

        // Change the role_groups
        for (uint i = 0; i < roles.length; i++)
            Util.bytes32Delete(_group, self.role_groups[roles[i]]);

        // Change the group_roles
        self.group_roles[_group] = roles;
        delete self.group_roles[_group];
        GroupDeleted(_group);
        return true;
    }

    /// @dev Delete the role
    function deleteRole(Authorization storage self, bytes32 _role)
        internal
        returns (bool)
    {
        bytes32[] memory groups = self.role_groups[_role];

        // Change the group_roles
        for (uint i = 0; i < groups.length; i++)
            Util.bytes32Delete(_role, self.group_roles[groups[i]]);

        // Change the role_groups
        self.role_groups[_role] = groups;
        delete self.role_groups[_role];
        RoleDeleted(_role);
        return true;
    }
}
