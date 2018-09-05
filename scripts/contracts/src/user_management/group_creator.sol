pragma solidity ^0.4.24;

import "./group.sol";
import "../common/address.sol";


/// @title Group factory contract to create group contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xfFFffFfFFFFfFFFfFfffffFFfffffffffF02000B
///         The interface: None
contract GroupCreator is ReservedAddress {

    event GroupCreated(address indexed _id, address indexed _parent, bytes32 indexed _name, address[] accounts);

    /// @notice Create a new group contract
    /// @param _parent The parent group
    /// @param _name  The name of group
    /// @return New group's accounts
    function createGroup(address _parent, bytes32 _name, address[] _accounts)
        public
        returns (Group groupAddress)
    {
        require(userManagementAddr == msg.sender, "permission denied.");

        groupAddress = new Group(_parent, _name, _accounts);
        emit GroupCreated(groupAddress, _parent, _name, _accounts);
    }
}
