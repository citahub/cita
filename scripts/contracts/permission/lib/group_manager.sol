pragma solidity ^0.4.18;

import "./util.sol";

/// @title Manage the group
library GroupManager {

    using Util for *;

    struct Groups {
        mapping(bytes32 => Group) groups;
    }

    struct Group {
        bytes32 name;
        address[] users;
        bytes32[] subGroups;
        bool subSwitch;
        bytes32 parentGroup;
        // HTTP: url or IPFS: fingerprint
        string profile;
    }

    event GroupNewed(bytes32 indexed _group, bytes32 indexed _parentGroup);
    event NameModified(bytes32 indexed _oldName, bytes32 indexed _newName);
    event SubSwitchModified(bool indexed _oldSubSwitch, bool indexed _newSubSwitch);
    event ProfileModified(string _oldSubSwitch, string _newProfile);
    event UsersAdded(bytes32 indexed _group, address[] _users);
    event UsersDeleted(bytes32 indexed _group, address[] _users);
    event GroupDeleted(bytes32 indexed _group, bytes32 parentGroup);
    event GroupInited(bytes32 indexed _root, address[] _adamEve, bool indexed _subSwitch);

    /// @dev Init the root group
    /// @return The root group name
    function initGroup(
        Groups storage self,
        bytes32 _root,
        address[] _adamEve,
        bool _subSwitch
    )
        internal 
        returns (bool)
    {
        self.groups[_root].name = _root;

        for (uint i = 0; i < _adamEve.length; i++)
            self.groups[_root].users.push(_adamEve[i]);

        self.groups[_root].subSwitch = _subSwitch;
        GroupInited(_root, _adamEve, _subSwitch);
        return true;
    }

    /// @dev New a group
    /// @param _group The group of the caller
    function newGroup(
        Groups storage self,
        bytes32 _group,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        Util.SetOp _op,
        string _profile
    )
        internal 
        returns (bool)
    {
        self.groups[_newName].name = _newName;
        self.groups[_newName].subSwitch = _newSubSwitch;
        self.groups[_newName].parentGroup = _group;
        self.groups[_newName].profile = _profile;

        if (Util.SetOp.None == _op) {
            for (uint i = 0; i < _newUsers.length; i++)
                self.groups[_newName].users.push(_newUsers[i]);
        } else {
            address[] memory one = Util.setOpAddress(self.groups[_group].users, _newUsers, _op);
            for (uint j = 0; j < one.length; j++)
                self.groups[_newName].users.push(one[j]);
        }

        self.groups[_group].subGroups.push(_newName);
        GroupNewed(_newName, _group);
        return true;
    }

    /// @dev Modify the name
    function modifyName(
        Groups storage self,
        bytes32 _oldName,
        bytes32 _newName
    )
        internal
        returns (bool)
    {
        Group memory group = self.groups[_oldName];
        self.groups[_newName] = group;
        self.groups[_newName].name = _newName;
        delete self.groups[_oldName];
        NameModified(_oldName, _newName);
        return true;
    }

    /// @dev Modify the subSwitch
    function modifySubSwitch(
        Groups storage self,
        bytes32 _group,
        bool _newSubSwitch
    )
        internal
        returns (bool)
    {
        self.groups[_group].subSwitch = _newSubSwitch;
        SubSwitchModified(self.groups[_group].subSwitch, _newSubSwitch);
        return true;
    }

    /// @dev Modify the profile
    function modifyProfile(
        Groups storage self,
        bytes32 _group,
        string _newProfile
    )
        internal
        returns (bool)
    {
        self.groups[_group].profile = _newProfile;
        ProfileModified(self.groups[_group].profile, _newProfile);
        return true;
    }

    /// @dev Add users 
    function addUsers(
        Groups storage self,
        bytes32 _group,
        address[] _users
    )
        internal
        returns (bool)
    {
        address[] memory result = Util.opUnionAddress(self.groups[_group].users, _users);

        for (uint i = 0; i < result.length; i++)
            self.groups[_group].users.push(result[i]);

        UsersAdded(_group, _users);
        return true;
    }

    /// @dev Delete users 
    function deleteUsers(
        Groups storage self,
        bytes32 _group,
        address[] _users
    )
        internal
        returns (bool)
    {
        address[] memory result = Util.opDiffAddress(self.groups[_group].users, _users);

        for (uint i = 0; i < result.length; i++)
            self.groups[_group].users.push(result[i]);

        UsersDeleted(_group, _users);
        return true;
    }

    /// @dev Delete group
    /// @notice Delete a tree's node. Only leaf node
    ///         Also delete the subGroups of parentGroup
    function deleteGroup(
        Groups storage self,
        bytes32 _group
    )
        internal
        returns (bool)
    {
        Util.bytes32Delete(_group, self.groups[self.groups[_group].parentGroup].subGroups);

        delete self.groups[_group];
        GroupDeleted(_group, self.groups[_group].parentGroup);
        return true;
    }
}
