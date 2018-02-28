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

    event PermissionDeleted(address _permission);

    modifier sameLength(address[] _one, bytes4[] _other) {
        require(_one.length > 0);
        require(_one.length == _other.length);
        _;
    }

    /// @dev Create a new permission
    function newPermission(bytes32 _name, address[] _conts, bytes4[] _funcs)
        public
        sameLength(_conts, _funcs)
        returns (address id)
    {
        return permissionCreator.createPermission(_name, _conts, _funcs);
    }

    /// @dev Delete the permission
    function deletePermission(address _permission)
        public
        returns (bool)
    {
        Permission perm = Permission(_permission);
        perm.close();
        // Cancel the auth of the accounts who have the permission
        auth.clearAuthOfPermission(_permission);
        PermissionDeleted(_permission);
        return true;
    }

    /// @dev Update the permission name
    function updatePermissionName(address _permission, bytes32 _name)
        public
        returns (bool)
    {
        Permission perm = Permission(_permission);
        return perm.updateName(_name);
    }

    /// @dev Add the resources of permission
    function addResources(address _permission, address[] _conts, bytes4[] _funcs)
        public
        returns (bool)
    {
        Permission perm = Permission(_permission);
        return perm.addResources(_conts, _funcs);
    }

    /// @dev Delete the resources of permission
    function deleteResources(address _permission, address[] _conts, bytes4[] _funcs)
        public
        returns (bool)
    {
        Permission perm = Permission(_permission);
        return perm.deleteResources(_conts, _funcs);
    }

    /// @dev Set authorizations
    function setAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            if (!auth.setAuth(_account, _permissions[i]))
                return false;
        }

        return true;
    }

    /// @dev Set authorization
    function setAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        return auth.setAuth(_account, _permission);
    }

    /// @dev Cancel authorizations
    function cancelAuthorizations(address _account, address[] _permissions)
        public
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++) {
            if (!auth.cancelAuth(_account, _permissions[i]))
                return false;
        }

        return true;
    }

    /// @dev Cancel authorization
    function cancelAuthorization(address _account, address _permission)
        public
        returns (bool)
    {
        return auth.cancelAuth(_account, _permission);
    }

    /// @dev Clear the account's permissions
    function clearAuthorization(address _account)
        public
        returns (bool)
    {
        return auth.clearAuth(_account);
    }
 }
