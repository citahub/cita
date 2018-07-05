pragma solidity ^0.4.18;

import "./permission_creator.sol";
import "./authorization.sol";


/// @title Permission management contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020004
///         The interface the can be called: All
/// @dev TODO check address is contract
contract PermissionManagement {

    address permissionCreatorAddr = 0xffffffffffffffffffffffffffffffffff020005;
    PermissionCreator permissionCreator = PermissionCreator(permissionCreatorAddr);

    address authorizationAddr = 0xffffffffffffffffffffffffffffffffff020006;
    Authorization auth = Authorization(authorizationAddr);

    address[15] builtInPermissions = [
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
        0x000000000000000000000000000000000b3241b5,
        0x000000000000000000000000000000000C3241B5,
        0x000000000000000000000000000000000D3241b5,
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
        for (uint i = 0; i < builtInPermissions.length; i++)
            require(_permission != builtInPermissions[i]);
        _;
    }

    /// @notice Create a new permission
    /// @dev TODO Check the funcs belong the conts
    /// @param _name  The name of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of the resource
    /// @return New permission's address
    function newPermission(bytes32 _name, address[] _conts, bytes4[] _funcs)
        external
        sameLength(_conts, _funcs)
        returns (address id)
    {
        return permissionCreator.createPermission(_name, _conts, _funcs);
    }

    /// @notice Delete the permission
    /// @param _permission The address of permission
    /// @return true if successed, otherwise false
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

    /// @notice Update the permission name
    /// @param _permission The address of permission
    /// @param _name The new name
    /// @return true if successed, otherwise false
    function updatePermissionName(address _permission, bytes32 _name)
        external
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.updateName(_name));
        return true;
    }

    /// @notice Add the resources of permission
    /// @param _permission The address of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of resource
    /// @return true if successed, otherwise false
    function addResources(address _permission, address[] _conts, bytes4[] _funcs)
        external
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.addResources(_conts, _funcs));
        return true;
    }

    /// @notice Delete the resources of permission
    /// @param _permission The address of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of resource
    /// @return true if successed, otherwise false
    function deleteResources(address _permission, address[] _conts, bytes4[] _funcs)
        external
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.deleteResources(_conts, _funcs));
        return true;
    }

    /// @notice Set multiple permissions to the account
    /// @param _account The account to be setted
    /// @param _permissions The multiple permissions to be setted
    /// @return true if successed, otherwise false
    function setAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(auth.setAuth(_account, _permissions[i]));

        return true;
    }

    /// @notice Set permission to the account
    /// @param _account The account to be setted
    /// @param _permission The permission to be setted
    /// @return true if successed, otherwise false
    function setAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        require(auth.setAuth(_account, _permission));
        return true;
    }

    /// @notice Cancel the account's muliple permissions
    /// @param _account The account to be canceled
    /// @param _permissions The multiple permissions to be canceled
    /// @return true if successed, otherwise false
    function cancelAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(auth.cancelAuth(_account, _permissions[i]));

        return true;
    }

    /// @notice Cancel the account's permission
    /// @param _account The account to be canceled
    /// @param _permission The permission to be canceled
    /// @return true if successed, otherwise false
    function cancelAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        require(auth.cancelAuth(_account, _permission));
        return true;
    }

    /// @notice Clear the account's permissions
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearAuthorization(address _account)
        public
        returns (bool)
    {
        require(auth.clearAuth(_account));
        return true;
    }
}
