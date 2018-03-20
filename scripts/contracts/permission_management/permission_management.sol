pragma solidity ^0.4.18;

import "./permission_creator.sol";
import "./authorization.sol";


/// @title Permission Management
/// @notice Not include the query interface
contract PermissionManagement {

    address permissionCreatorAddr = 0x00000000000000000000000000000000013241b3;
    PermissionCreator permissionCreator = PermissionCreator(permissionCreatorAddr);

    address authorizationAddr = 0x00000000000000000000000000000000013241b4;
    Authorization auth = Authorization(authorizationAddr);

    address[12] builtInPermissions = [
        0x00000000000000000000000000000000013241b5,
        0x00000000000000000000000000000000023241b5,
        0x00000000000000000000000000000000033241B5,
        0x00000000000000000000000000000000043241b5,
        0x00000000000000000000000000000000053241b5,
        0x00000000000000000000000000000000063241b5,
        0x00000000000000000000000000000000073241b5,
        0x00000000000000000000000000000000083241B5,
        0x00000000000000000000000000000000093241B5,
        0x000000000000000000000000000000000A3241b5,
        0x0000000000000000000000000000000000000001,
        0x0000000000000000000000000000000000000002
    ];

    event PermissionDeleted(address _permission);

    modifier sameLength(address[] _one, bytes4[] _other) {
        require(_one.length > 0);
        require(_one.length == _other.length);
        _;
    }

    modifier notBuiltInPermission(address _permission) {
        for (uint i = 0; i<builtInPermissions.length; i++)
            require(_permission != builtInPermissions[i]); 
        _;
    }

    /// @dev Create a new permission
    function newPermission(bytes32 _name, address[] _conts, bytes4[] _funcs)
        external 
        sameLength(_conts, _funcs)
        returns (address id)
    {
        return permissionCreator.createPermission(_name, _conts, _funcs);
    }

    /// @dev Delete the permission
    function deletePermission(address _permission)
        external 
        notBuiltInPermission(_permission)
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.close());
        // Cancel the auth of the accounts who have the permission
        require(auth.clearAuthOfPermission(_permission));
        PermissionDeleted(_permission);
        return true;
    }

    /// @dev Update the permission name
    function updatePermissionName(address _permission, bytes32 _name)
        external 
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.updateName(_name));
        return true;
    }

    /// @dev Add the resources of permission
    function addResources(address _permission, address[] _conts, bytes4[] _funcs)
        external 
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.addResources(_conts, _funcs));
        return true;
    }

    /// @dev Delete the resources of permission
    function deleteResources(address _permission, address[] _conts, bytes4[] _funcs)
        external 
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.deleteResources(_conts, _funcs));
        return true;
    }

    /// @dev Set authorizations
    function setAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(auth.setAuth(_account, _permissions[i]));

        return true;
    }

    /// @dev Set authorization
    function setAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        require(auth.setAuth(_account, _permission));
        return true;
    }

    /// @dev Cancel authorizations
    function cancelAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(auth.cancelAuth(_account, _permissions[i]));

        return true;
    }

    /// @dev Cancel authorization
    function cancelAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        require(auth.cancelAuth(_account, _permission));
        return true;
    }

    /// @dev Clear the account's permissions
    function clearAuthorization(address _account)
        public
        returns (bool)
    {
        require(auth.clearAuth(_account));
        return true;
    }
}
