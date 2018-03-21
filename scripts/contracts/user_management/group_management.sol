pragma solidity ^0.4.18;

import "./group_creator.sol";

contract GroupManagement {

    address groupCreatorAddr = 0x00000000000000000000000000000000013241c3;
    GroupCreator groupCreator = GroupCreator(groupCreatorAddr);

    modifier onlyLeafNode(address _group) {
        Group group = Group(_group);
        require(group.queryChildLength() == 0);
        _;
    }

    event GroupDeleted(address _group);

    /// @dev Create a new group
    function newGroup(address _parent, bytes32 _name, address[] _accounts)
        public
        returns (address groupId)
    {
        Group parent = Group(_parent);
        var groupid = groupCreator.createGroup(_parent, _name, _accounts);
        require(parent.addChild(groupid));
        return groupid;
    }

    /// @dev Delete the group
    function deleteGroup(address _group)
        public
        onlyLeafNode(_group)
        returns (bool)
    {
        Group group = Group(_group);
        // Delete the parent group's child
        require(deleteChildGroup(group.queryParent(), _group));
        require(group.close());
        GroupDeleted(_group);
        return true;
    }

    /// @dev Update the group name
    function updateGroupName(address _group, bytes32 _name)
        public
        returns (bool)
    {
        Group group = Group(_group);
        require(group.updateName(_name));
        return true;
    }

    /// @dev Add accounts
    function addAccounts(address _group, address[] _accounts)
        public
        returns (bool)
    {
        Group group = Group(_group);
        require(group.addAccounts(_accounts));
        return true;
    }

    /// @dev Delete accounts
    function deleteAccounts(address _group, address[] _accounts)
        public
        returns (bool)
    {
        Group group = Group(_group);
        require(group.deleteAccounts(_accounts));
        return true;
    }

    /// @dev Add a child group
    function addChildGroup(address _group, address _child)
        public
        returns (bool)
    {
        Group group = Group(_group);
        require(group.addChild(_child));
        return true;
    }

    /// @dev Delete the child group
    function deleteChildGroup(address _group, address _child)
        private
        onlyLeafNode(_child)
        returns (bool)
    {
        Group group = Group(_group);
        require(group.deleteChild(_child));
        return true;
    }
}
