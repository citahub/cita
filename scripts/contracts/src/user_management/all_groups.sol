pragma solidity ^0.4.24;

import "../lib/address_array.sol";
import "../common/address.sol";
import "../interfaces/all_groups.sol";

/// @title User management using group struct
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xfFFffFFFfffFfFFFfFfFFfffffffFfFfFf020012
///         The interface the can be called: All
contract AllGroups is IAllGroups, ReservedAddress {

    address[] groups;

    modifier onlyGroupManagement {
        require(userManagementAddr == msg.sender, "permission denied.");
        _;
    }

    /// @notice Constructor
    constructor() public {
        // Root
        groups.push(rootGroupAddr);
    }

    /// @notice Insert a new group
    /// @param _group  the group be added
    function insert(address _group)
        external
        onlyGroupManagement
        returns (bool)
    {
        groups.push(_group);
    }

    /// @notice Delete the group
    /// @return True if successed, otherwise false
    function drop(address _group)
        external
        onlyGroupManagement
        returns (bool)
    {
        AddressArray.remove(_group, groups);
        return true;
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
}
