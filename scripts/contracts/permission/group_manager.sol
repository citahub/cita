pragma solidity ^0.4.14;

import "./set_operate.sol";
import "./authorization_manager.sol";

/// @notice TODO: Only from router's address. Need an address of the router contract
/// @title Manager the group
contract GroupManager {

    using SetOperate for *;

    mapping(bytes32 => Group) groups;

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
        bytes32 _name,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        SetOperate.SetOp _op
    )
        public
        returns (bytes32)
    {
        Group memory group;
        group.name = _newName;
        group.subSwitch = _newSubSwitch;

        if (SetOperate.SetOp.None == _op) {
            for (uint i = 0; i < _newUsers.length; i++)
                group.users[i] = _newUsers[i];
        } else {
            address[] memory one = SetOperate.setOpAddress(groups[_name].users, _newUsers, _op);
            for (uint j = 0; j < one.length; j++)
                group.users[j] = one[j];
        }

        GroupNewed(_newName);
        return group.name;
    }

    /// @dev Modify the name
    /// @notice TODO. Need to change authorization too
    function modifyName(bytes32 _oldName, bytes32 _newName) public returns (bool) {
        Group memory group = groups[_oldName];
        group.name = _newName;
        groups[_newName] = group;
        delete groups[_oldName];
        // Also change authorization
        AuthorizationManager auth = new AuthorizationManager();
        auth.replaceGroup(_oldName, _newName); 
        NameModified(_oldName, _newName);
        return true;
    }

    /// @dev Modify the sub_switch
    function modifySubSwitch(bytes32 _name, bool _newSubSwitch) public returns (bool) {
        SubSwitchModified(groups[_name].subSwitch, _newSubSwitch);
        groups[_name].subSwitch = _newSubSwitch;
        return true;
    }

    /// @dev Add users 
    function addUsers(bytes32 _name, address[] _users) public returns (bool) {
        address[] memory result = SetOperate.opUnionAddress(groups[_name].users, _users);

        for (uint i = 0; i < result.length; i++)
            groups[_name].users[i] = result[i];

        UsersAdded(_name, _users);
        return true;

    }

    /// @dev Delete users 
    function deleteUsers(bytes32 _name, address[] _users) public returns (bool) {
        address[] memory result = SetOperate.opDiffAddress(groups[_name].users, _users);

        for (uint i = 0; i < result.length; i++)
            groups[_name].users[i] = result[i];

        UsersDeleted(_name, _users);
        return true;
    }

    /// @dev Delete group
    /// @notice Delete a tree's node. Need to discuss. Only leaf node?
    function deleteGroup(bytes32 _name) public returns (bool) {
        delete groups[_name];
        AuthorizationManager auth = new AuthorizationManager();
        // Also delete the authorization
        auth.deleteGroup(_name);
        GroupDeleted(_name);
        return true;
    }

    /// @notice Should check the sub_switch
    /// @dev Query the users
    function queryUsers(bytes32 _name) public returns (address[]) {
        return groups[_name].users;
    }
}
