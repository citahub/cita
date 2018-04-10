pragma solidity ^0.4.18;

import "./group_creator.sol";
import "./address_array.sol";


/// @title User management using group struct
/// @notice _origin: One of sender's groups
/// @notice _target: The target group that will be operated
contract GroupManagement {

    address rootGroupAddr = 0x00000000000000000000000000000000013241b6;
    address groupCreatorAddr = 0x00000000000000000000000000000000013241c3;
    GroupCreator groupCreator = GroupCreator(groupCreatorAddr);

    address[] groups;

    event GroupDeleted(address _group);

    modifier onlyLeafNode(address _group) {
        Group group = Group(_group);
        require(group.queryChildLength() == 0);
        _;
    }

    modifier inGroup(address _group) {
        Group group = Group(_group);
        require(group.inGroup(msg.sender));
        _;
    }

    /// @dev Constructor
    function GroupManagement() public {
        // Root
        groups.push(rootGroupAddr);
    }

    /// @dev Create a new group
    function newGroup(address _parent, bytes32 _name, address[] _accounts)
        external
        returns (address new_group)
    {
        new_group = groupCreator.createGroup(_parent, _name, _accounts);
        require(addChild(_parent, new_group));
        groups.push(new_group);
    }

    /// @dev Delete the group
    function deleteGroup(address _origin, address _target)
        external
        inGroup(_origin)
        onlyLeafNode(_target)
        returns (bool)
    {
        require(checkScope(_origin, _target));
        Group group = Group(_target);
        // Delete it from the parent group
        require(deleteChild(group.queryParent(), _target));
        // Selfdestruct
        require(group.close());
        // Remove it from the groups
        AddressArray.remove(_target, groups);
        GroupDeleted(_target);
        return true;
    }

    /// @dev Update the group name
    function updateGroupName(address _origin, address _target, bytes32 _name)
        external
        inGroup(_origin)
        returns (bool)
    {
        require(checkScope(_origin, _target));
        Group group = Group(_target);
        require(group.updateName(_name));
        return true;
    }

    /// @dev Add accounts
    function addAccounts(address _origin, address _target, address[] _accounts)
        external
        inGroup(_origin)
        returns (bool)
    {
        require(checkScope(_origin, _target));
        Group group = Group(_target);
        require(group.addAccounts(_accounts));
        return true;
    }

    /// @dev Delete accounts
    function deleteAccounts(address _origin, address _target, address[] _accounts)
        external
        inGroup(_origin)
        returns (bool)
    {
        require(checkScope(_origin, _target));
        Group group = Group(_target);
        require(group.deleteAccounts(_accounts));
        return true;
    }

    /// @dev Check the target group in the scope of the origin group
    /// @notice Scope: the origin group is the ancestor of the target group
    function checkScope(address _origin, address _target)
        public
        view
        returns (bool)
    {
        address parent = _target;

        // Until the root group
        while (parent != 0x0) {
            if (_origin == parent)
                return true;
            Group group = Group(parent);
            parent = group.queryParent();
        }
    }

    /// @dev Query all groups
    function queryGroups()
        public
        view
        returns (address[])
    {
        return groups;
    }

    /// @dev Delete the child group
    function deleteChild(address _group, address _child)
        private
        returns (bool)
    {
        Group group = Group(_group);
        require(group.deleteChild(_child));
        return true;
    }

    /// @dev Add a child group
    function addChild(address _group, address _child)
        private
        returns (bool)
    {
        Group group = Group(_group);
        require(group.addChild(_child));
        return true;
    }
}
