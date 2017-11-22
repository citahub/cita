pragma solidity ^0.4.14;

import "./util.sol";

/// @title Manager the group
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
    }

    event GroupNewed(bytes32 indexed _name);
    event NameModified(bytes32 indexed _oldName, bytes32 indexed _newName);
    event SubSwitchModified(bool indexed _oldSubSwitch, bool indexed _newSubSwitch);
    event UsersAdded(bytes32 indexed _name, address[] _users);
    event UsersDeleted(bytes32 indexed _name, address[] _users);
    event GroupDeleted(bytes32 indexed _name);
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
            self.groups[_root].users[i] = _adamEve[i];

        self.groups[_root].subSwitch = _subSwitch;
        GroupInited(_root, _adamEve, _subSwitch);
        return true;
    }

    /// @dev New a group
    /// @param _name The group name of the caller
    /// @return The new group name
    function newGroup(
        Groups storage self,
        bytes32 _name,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        Util.SetOp _op
    )
        internal 
        returns (bool)
    {
        self.groups[_newName].name = _newName;
        self.groups[_newName].subSwitch = _newSubSwitch;

        if (Util.SetOp.None == _op) {
            for (uint i = 0; i < _newUsers.length; i++)
                self.groups[_newName].users[i] = _newUsers[i];
        } else {
            address[] memory one = Util.setOpAddress(self.groups[_name].users, _newUsers, _op);
            for (uint j = 0; j < one.length; j++)
                self.groups[_newName].users[j] = one[j];
        }

        self.groups[_name].subGroups.push(_newName);
        GroupNewed(_newName);
        return true;
    }

    /// @dev Modify the name
    function modifyName(Groups storage self, bytes32 _oldName, bytes32 _newName) internal returns (bool) {
        Group memory group = self.groups[_oldName];
        // Will it work?
        self.groups[_newName] = group;
        self.groups[_newName].name = _newName;
        delete self.groups[_oldName];
        NameModified(_oldName, _newName);
        return true;
    }

    /// @dev Modify the sub_switch
    function modifySubSwitch(
        Groups storage self,
        bytes32 _name,
        bool _newSubSwitch
    )
        internal
        returns (bool)
    {
        SubSwitchModified(self.groups[_name].subSwitch, _newSubSwitch);
        self.groups[_name].subSwitch = _newSubSwitch;
        return true;
    }

    /// @dev Add users 
    function addUsers(Groups storage self, bytes32 _name, address[] _users) internal returns (bool) {
        address[] memory result = Util.opUnionAddress(self.groups[_name].users, _users);

        for (uint i = 0; i < result.length; i++)
            self.groups[_name].users[i] = result[i];

        UsersAdded(_name, _users);
        return true;

    }

    /// @dev Delete users 
    function deleteUsers(Groups storage self, bytes32 _name, address[] _users) internal returns (bool) {
        address[] memory result = Util.opDiffAddress(self.groups[_name].users, _users);

        for (uint i = 0; i < result.length; i++)
            self.groups[_name].users[i] = result[i];

        UsersDeleted(_name, _users);
        return true;
    }

    /// @dev Delete group
    /// @notice Delete a tree's node. Need to discuss. Only leaf node?
    function deleteGroup(Groups storage self, bytes32 _name) internal returns (bool) {
        delete self.groups[_name];
        GroupDeleted(_name);
        return true;
    }
}
