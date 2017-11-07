pragma solidity ^0.4.14;

import "./set_operate.sol";
import "./authorization_manager.sol";

/// @notice TODO. Only from router's address
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

    /// @dev New a group
    /// @param _name The group name of the caller
    /// @return The new group name
    function newGroup(
        bytes32 _name,
        bytes32 _newName,
        address[] _newUser,
        bool _newSubSwitch,
        SetOperate.SetOp _op
    )
        public
        returns (bytes32);

    /// @dev Modify the name
    /// @notice TODO. Need to change authorization too
    function modifyName(bytes32 _oldName, bytes32 _newName) public returns (bool);

    /// @dev Modify the sub_switch
    function modifySubSwitch(bytes32 _name, bool _newSubSwitch) public returns (bool);

    /// @dev Add users 
    function addUsers(bytes32 _name, address[] _users) public returns (bool);

    /// @dev Delete users 
    function deleteUsers(bytes32 _name, address[] _users) public returns (bool);

    /// @dev Delete group
    /// @notice Delete a tree's node. Need to discuss. Only leaf node?
    function deleteGroup(bytes32 _name) public returns (bool);

    /// @notice Should check the sub_switch
    /// @dev Query the users
    function queryUsers(bytes32 _name) public returns (bool);
}
