pragma solidity ^0.4.18;

import "./permission.sol";


/**
 * ======= permission_management.sol:PermissionManagement =======
 * Function signatures: 
 * fc4a089c: newPermission(bytes32,address[],bytes4[])
 * 98a05bb1: deletePermission(address)
 * f036ed56: addResources(address,address[],bytes4[])
 * 6446ebd8: deleteResources(address,address[],bytes4[])
 * 537bf9a3: updatePermissionName(address,bytes32)
 * 0f5aa9f3: setAuthorization(address,address)
 * 3482e0c9: cancelAuthorization(address,address)
 * a5925b5b: clearAuthorization(address)
 * ====================================
 * @title Permission factory contract to create permission contract
 * @notice Only permission_management contract can call except query function
 */
contract PermissionCreator {

    address permissionManagementAddr = 0x00000000000000000000000000000000013241b2;

    bytes4[8] funcs = [
        bytes4(0xf036ed56),
        bytes4(0x3482e0c9),
        bytes4(0xa5925b5b),
        bytes4(0x98a05bb1),
        bytes4(0x6446ebd8),
        bytes4(0xfc4a089c),
        bytes4(0x0f5aa9f3),
        bytes4(0x537bf9a3)
    ];

    // Save the id(contract address) of the initialized permission
    mapping(bytes32 => address) ids;

    event PermissionCreated(address indexed _id, bytes32 indexed _name, address[] _conts, bytes4[] _funcs);

    modifier onlyPermissionManagement {
        require(permissionManagementAddr == msg.sender);
        _;
    }

    /// @dev Constructor: Initialize the permssion
    function PermissionCreator() public {
        bytes4[] memory newPermission = new bytes4[](1);
        newPermission[0] = funcs[0];

        bytes4[] memory deletePermission = new bytes4[](1);
        deletePermission[0] = funcs[1];

        bytes4[] memory updatePermission = new bytes4[](3);
        address[] memory contAddr = new address[](3);
        updatePermission[0] = funcs[2];
        updatePermission[1] = funcs[3];
        updatePermission[2] = funcs[4];

        for (uint i = 0; i < 3; i++)
            contAddr[i] = permissionManagementAddr;

        bytes4[] memory setAuth = new bytes4[](1);
        setAuth[0] = funcs[5];

        bytes4[] memory cancelAuth = new bytes4[](1);
        setAuth[0] = funcs[6];

        bytes4[] memory clearAuth = new bytes4[](1);
        clearAuth[0] = funcs[7];

        address[] memory contCommon = new address[](1);
        contCommon[0] = permissionManagementAddr;

        ids[bytes32('NewPermission')] =
            _createPermission(bytes32('NewPermission'), contCommon, newPermission);
        ids[bytes32('DeletePermission')] =
            _createPermission(bytes32('DeletePermission'), contCommon, deletePermission);
        ids[bytes32('UpdatePermission')] =
            _createPermission(bytes32('UpdatePermission'), contAddr, updatePermission);
        ids[bytes32('SetAuth')] =
            _createPermission(bytes32('SetAuth'), contCommon, setAuth);
        ids[bytes32('CancelAuth')] =
            _createPermission(bytes32('CancelAuth'), contCommon, cancelAuth);
        ids[bytes32('ClearAuth')] =
            _createPermission(bytes32('ClearAuth'), contCommon, clearAuth);
    }

    /// @dev Create a new permission contract
    function createPermission(bytes32 _name, address[] _conts, bytes4[] _funcs) 
        public 
        onlyPermissionManagement
        returns (Permission permissionAddress)
    {
        return _createPermission(_name, _conts, _funcs);
    }

    /// @dev Query the permission_name's id
    function queryId(bytes32 _name)
        public
        view
        returns (address)
    {
        return ids[_name];
    }

    /// @dev Create a new permission contract
    function _createPermission(bytes32 _name, address[] _conts, bytes4[] _funcs) 
        private 
        returns (Permission permissionAddress)
    {
        Permission perm = new Permission(_name, _conts, _funcs); 
        PermissionCreated(perm, _name, _conts, _funcs);
        return perm;
    }
}
