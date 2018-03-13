pragma solidity ^0.4.18;

import "./group.sol";


/// @title Group factory contract to create group contract
contract GroupCreator {

    address userManagementAddr = 0x00000000000000000000000000000000013241C2;

    modifier onlyUserManagement {
        require(userManagementAddr == msg.sender);
        _;
    }

    event GroupCreated(address indexed _id, address indexed _parent, bytes32 indexed _name, address[] accounts);

    /// @dev Create a new group contract
    function createGroup(address _parent, bytes32 _name, address[] _accounts)
        public
        onlyUserManagement
        returns (Group groupAddress)
    {
        Group group = new Group(_parent, _name, _accounts);
        GroupCreated(group, _parent, _name, _accounts);
        return group;
    }
}
