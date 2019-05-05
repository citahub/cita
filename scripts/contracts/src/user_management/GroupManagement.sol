pragma solidity 0.4.24;

import "./GroupCreator.sol";
import "../lib/AddressArray.sol";
import "../common/ReservedAddrPublic.sol";
import "../../interaction/interface/IAuthorization.sol";
import "../../interaction/interface/IGroupManagement.sol";
import "../../interaction/interface/IAllGroups.sol";

/// @title User management using group struct
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a
///         The interface the can be called: All
///         Origin: One group choosed by sender from all his groups
///         Target: The target group to be operated
contract GroupManagement is IGroupManagement, ReservedAddrPublic {

    GroupCreator groupCreator = GroupCreator(groupCreatorAddr);
    /// Just for compatible
    address[] private _groups;
    IAuthorization auth = IAuthorization(authorizationAddr);
    IAllGroups constant groups = IAllGroups(allGroupsAddr);

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

    modifier checkPermission(address _permission, address _origin) {
        require(
            auth.checkPermission(msg.sender, _permission) ||
                auth.checkPermission(_origin, _permission),
            "Permission denied.");
        _;
    }

    /// @notice Constructor
    /// Just for compatible
    constructor() public {
        // Root
        _groups.push(rootGroupAddr);
    }

    /// @notice Create a new group
    /// @param _origin The sender's orgin group
    /// @param _name  The name of group
    /// @param _accounts The accounts of group
    /// @return New role's address
    /// @dev TODO Add a param: target.
    function newGroup(address _origin, bytes32 _name, address[] _accounts)
        external
        // Have to check all the permission of account's groups. but can not do it for now.
        checkPermission(builtInPermissions[10], 0x0)
        returns (address new_group)
    {
        new_group = groupCreator.createGroup(_origin, _name, _accounts);
        require(addChild(_origin, new_group), "addChild failed.");
        groups.insert(new_group);
    }

    /// @notice Delete the group
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be deleted
    /// @return True if successed, otherwise false
    function deleteGroup(address _origin, address _target)
        external
        inGroup(_origin)
        onlyLeafNode(_target)
        checkPermission(builtInPermissions[11], _origin)
        returns (bool)
    {
        require(
            checkScope(_origin, _target),
            "The target group not in origin group."
        );
        Group group = Group(_target);
        // Delete it from the parent group
        require(
            deleteChild(group.queryParent(), _target),
            "deleteChild failed."
        );
        // Selfdestruct
        group.close();
        emit GroupDeleted(_target);
        // Remove it from the groups
        groups.drop(_target);
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
        checkPermission(builtInPermissions[12], _origin)
        returns (bool)
    {
        require(
            checkScope(_origin, _target),
            "The target not in origin group."
        );
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
        checkPermission(builtInPermissions[12], _origin)
        returns (bool)
    {
        require(
            checkScope(_origin, _target),
            "The target not in origin group."
        );
        Group group = Group(_target);
        require(group.addAccounts(_accounts), "addAccounts failed.");
        return true;
    }

    /// @notice Delete accounts
    /// @param _origin The sender's orgin group
    /// @param _target The target group to be deleted
    /// @param _accounts The accounts to be deleted
    /// @return True if successed, otherwise false
    function deleteAccounts(
        address _origin,
        address _target,
        address[] _accounts
    )
        external
        inGroup(_origin)
        checkPermission(builtInPermissions[12], _origin)
        returns (bool)
    {
        require(
            checkScope(_origin, _target),
            "The target not in origin group."
        );
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
    ///         (for compatible)
    /// @return All groups
    function queryGroups()
        public
        returns (address[])
    {
        return groups.queryGroups();
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
