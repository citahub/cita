pragma solidity ^0.4.18;

import "./permission.sol";


/// @title Permission factory contract to create permission contract
/// @notice Only permission_management contract can call except query function
contract PermissionCreator {

    address permissionManagementAddr = 0x00000000000000000000000000000000013241b2;

    event PermissionCreated(address indexed _id, bytes32 indexed _name, address[] _conts, bytes4[] _funcs);

    modifier onlyPermissionManagement {
        require(permissionManagementAddr == msg.sender);
        _;
    }

    /// @dev Create a new permission contract
    function createPermission(bytes32 _name, address[] _conts, bytes4[] _funcs) 
        public 
        onlyPermissionManagement
        returns (Permission permissionAddress)
    {
        return _createPermission(_name, _conts, _funcs);
    }

    /// @dev Private: Create a new permission contract
    function _createPermission(bytes32 _name, address[] _conts, bytes4[] _funcs) 
        private 
        returns (Permission permissionAddress)
    {
        Permission perm = new Permission(_name, _conts, _funcs); 
        PermissionCreated(perm, _name, _conts, _funcs);
        return perm;
    }
}
