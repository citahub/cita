pragma solidity ^0.4.18;

import "./role.sol";


/// @title Role factory contract to create role contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xe9e2593c7d1db5ee843c143e9cb52b8d996b2380
///         The interface: None
contract RoleCreator {

    address roleManagementAddr = 0xe3b5DDB80AdDb513b5c981e27Bb030A86A8821eE;

    event RoleCreated(address indexed _id, bytes32 indexed _name, address[] indexed _permissions);

    /// @notice Create a new role contract
    /// @param _name  The name of role
    /// @param _permissions The permissions of role
    /// @return New role's address
    function createRole(bytes32 _name, address[] _permissions)
        public
        returns (Role roleAddress)
    {
        require(roleManagementAddr == msg.sender);

        roleAddress = new Role(_name, _permissions);
        RoleCreated(roleAddress, _name, _permissions);
    }
}
