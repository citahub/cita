pragma solidity ^0.4.24;

import "./group_creator.sol";
import "../lib/address_array.sol";
import "../common/address.sol";
import "../permission_management/authorization.sol";


/// @title User management using group struct
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a
///         The interface the can be called: All
///         Origin: One group choosed by sender from all his groups
///         Target: The target group to be operated
contract GroupManagement is ReservedAddress {

    GroupCreator groupCreator = GroupCreator(groupCreatorAddr);

    address[] groups;
    Authorization auth = Authorization(authorizationAddr);

    event GroupDeleted(address _group);

    modifier onlyLeafNode(address _group) {
        Group group = Group(_group);
        require(group.queryChildLength() == 0, "Only leaf node.");
        _;
    }

    modifier inGroup(address _group) {
        Group group = Group(_group);
        require(group.inGroup(msg.sender), "Not in group.");
        _;
    }

    modifier checkPermission(address _permission) {
        require(auth.checkPermission(msg.sender, _permission), "Permission denied.");
        _;
    }

    /// @notice Constructor
    constructor() public {
        // Root
        groups.push(rootGroupAddr);
    }

    /// @notice Create a new group
    /// @param _origin The sender's orgin group
    /// @param _name  The name of group
    /// @param _accounts The accounts of group
    /// @return New role's address
    function newGroup(address _origin, bytes32 _name, address[] _accounts)
        external
        checkPermission(builtInPermissions[10])
        returns (address new_group)
    {
        new_group = groupCreator.createGroup(_origin, _name, _accounts);
        require(addChild(_origin, new_group), "addChild failed.");
        groups.push(new_group);
    }

    /// @notice Delete the group
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be deleted
    /// @return True if successed, otherwise false
    function deleteGroup(address _origin, address _target)
        external
        inGroup(_origin)
        onlyLeafNode(_target)
        checkPermission(builtInPermissions[11])
        returns (bool)
    {
        require(checkScope(_origin, _target), "The target group not in origin group.");
        Group group = Group(_target);
        // Delete it from the parent group
        require(deleteChild(group.queryParent(), _target), "deleteChild failed.");
        // Selfdestruct
        group.close();
        // Remove it from the groups
        AddressArray.remove(_target, groups);
        emit GroupDeleted(_target);
        return true;
    }

    /// @notice Update the group name
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be updated
    /// @param _name  The new name to be updated
    /// @return True if successed, otherwise false
    function updateGroupName(address _origin, address _target, bytes32 _name)
        external
        inGroup(_origin)
        checkPermission(builtInPermissions[12])
        returns (bool)
    {
        require(checkScope(_origin, _target), "The target not in origin group.");
        Group group = Group(_target);
        require(group.updateName(_name), "updateName failed.");
        return true;
    }

    /// @notice Add accounts
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be added
    /// @param _accounts The accounts to be added
    /// @return True if successed, otherwise false
    function addAccounts(address _origin, address _target, address[] _accounts)
        external
        inGroup(_origin)
        checkPermission(builtInPermissions[12])
        returns (bool)
    {
        require(checkScope(_origin, _target), "The target not in origin group.");
        Group group = Group(_target);
        require(group.addAccounts(_accounts), "addAccounts failed.");
        return true;
    }

    /// @notice Delete accounts
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be deleted
    /// @param _accounts The accounts to be deleted
    /// @return True if successed, otherwise false
    function deleteAccounts(address _origin, address _target, address[] _accounts)
        external
        inGroup(_origin)
        checkPermission(builtInPermissions[12])
        returns (bool)
    {
        require(checkScope(_origin, _target), "The target not in origin group.");
        Group group = Group(_target);
        require(group.deleteAccounts(_accounts), "deleteAccounts failed.");
        return true;
    }

    /// @notice Check the target group in the scope of the origin group
    ///         Scope: the origin group is the ancestor of the target group
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be checked
    /// @return True if successed, otherwise false
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

    /// @notice Query all groups
    /// @return All groups
    function queryGroups()
        public
        view
        returns (address[])
    {
        return groups;
    }

    /// @notice Private: Delete the child group
    function deleteChild(address _group, address _child)
        private
        returns (bool)
    {
        Group group = Group(_group);
        require(group.deleteChild(_child), "deleteChild failed.");
        return true;
    }

    /// @notice Private: Add a child group
    function addChild(address _group, address _child)
        private
        returns (bool)
    {
        Group group = Group(_group);
        require(group.addChild(_child), "addChild failed.");
        return true;
    }
}
