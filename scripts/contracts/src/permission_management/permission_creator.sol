pragma solidity ^0.4.24;

import "./permission.sol";
import "../common/address.sol";


/// @title Permission factory contract to create permission contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address:0xffFFFffFfFFffffFffffFFfFffffFfFFFF020005
///         The interface: None
contract PermissionCreator is ReservedAddress {

    event PermissionCreated(address indexed _id, bytes32 indexed _name, address[] _conts, bytes4[] _funcs);

    /// @notice Create a new permission contract
    /// @param _name  The name of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of the resource
    /// @return New permission's address
    function createPermission(bytes32 _name, address[] _conts, bytes4[] _funcs)
        public
        returns (Permission permissionAddress)
    {
        require(permissionManagementAddr == msg.sender, "permission denied.");

        permissionAddress = new Permission(_name, _conts, _funcs);
        emit PermissionCreated(permissionAddress, _name, _conts, _funcs);
    }
}
