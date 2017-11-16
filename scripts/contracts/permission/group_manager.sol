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

    event GroupNewed(bytes32 _name);
    event NameModified(bytes32 _oldName, bytes32 _newName);
    event SubSwitchModified(bool _oldSubSwitch, bool _newSubSwitch);
    event UsersAdded(bytes32 _name, address[] _users);
    event UsersDeleted(bytes32 _name, address[] _users);
    event GroupDeleted(bytes32 _name);

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
        returns (bytes32)
    {
        Group memory group;
        group.name = _newName;
        group.subSwitch = _newSubSwitch;

        if (Util.SetOp.None == _op) {
            for (uint i = 0; i < _newUsers.length; i++)
                group.users[i] = _newUsers[i];
        } else {
            address[] memory one = Util.setOpAddress(self.groups[_name].users, _newUsers, _op);
            for (uint j = 0; j < one.length; j++)
                group.users[j] = one[j];
        }

        GroupNewed(_newName);
        return group.name;
    }

    /// @dev Modify the name
    /// @notice TODO. Need to change authorization too
    function modifyName(Groups storage self, bytes32 _oldName, bytes32 _newName) internal returns (bool) {
        Group memory group = self.groups[_oldName];
        group.name = _newName;
        self.groups[_newName] = group;
        delete self.groups[_oldName];
        NameModified(_oldName, _newName);
        return true;
    }

    /// @dev Modify the sub_switch
    function modifySubSwitch(Groups storage self, bytes32 _name, bool _newSubSwitch) internal returns (bool) {
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

    /// @notice Should check the sub_switch
    /// @dev Query the users
    function queryUsers(Groups storage self, bytes32 _name) constant returns (address[]) {
        return self.groups[_name].users;
    }
}
