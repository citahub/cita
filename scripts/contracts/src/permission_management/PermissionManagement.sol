pragma solidity 0.4.24;

import "./PermissionCreator.sol";
import "../common/Check.sol";
import "../../interaction/interface/IPermissionManagement.sol";
import "../../interaction/interface/IAuthorization.sol";

/// @title Permission management contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffFffFffFFffFFFFFfFfFFfFFFFfffFFff020004
///         The interface the can be called: All
/// @dev TODO check address is contract
// contract PermissionManagement is ReservedAddress {
contract PermissionManagement is IPermissionManagement, Check {

    PermissionCreator permissionCreator = PermissionCreator(
        permissionCreatorAddr
    );
    IAuthorization auth = IAuthorization(authorizationAddr);

    event PermissionDeleted(address _permission);

    modifier sameLength(address[] _one, bytes4[] _other) {
        require(_one.length > 0, "The length must large than zero.");
        require(
            _one.length == _other.length,
            "Two arrays'length not the same."
        );
        _;
    }

    modifier notBuiltInPermission(address _permission) {
        for (uint i = 0; i < builtInPermissions.length; i++)
            require(
                _permission != builtInPermissions[i],
                "not buildInPermission."
            );
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
        hasPermission(builtInPermissions[0])
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
        hasPermission(builtInPermissions[1])
        notBuiltInPermission(_permission)
        returns (bool)
    {
        Permission perm = Permission(_permission);
        perm.close();
        // Cancel the auth of the accounts who have the permission
        require(
            auth.clearAuthOfPermission(_permission),
            "deletePermission failed."
        );
        emit PermissionDeleted(_permission);
        return true;
    }

    /// @notice Update the permission name
    /// @param _permission The address of permission
    /// @param _name The new name
    /// @return true if successed, otherwise false
    function updatePermissionName(address _permission, bytes32 _name)
        external
        hasPermission(builtInPermissions[2])
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.updateName(_name), "updatePermissionName failed.");
        return true;
    }

    /// @notice Add the resources of permission
    /// @param _permission The address of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of resource
    /// @return true if successed, otherwise false
    function addResources(
        address _permission,
        address[] _conts,
        bytes4[] _funcs
    )
        external
        hasPermission(builtInPermissions[2])
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(perm.addResources(_conts, _funcs), "addResources failed.");
        return true;
    }

    /// @notice Delete the resources of permission
    /// @param _permission The address of permission
    /// @param _conts The contracts of resource
    /// @param _funcs The function signature of resource
    /// @return true if successed, otherwise false
    function deleteResources(
        address _permission,
        address[] _conts,
        bytes4[] _funcs
    )
        external
        hasPermission(builtInPermissions[2])
        returns (bool)
    {
        Permission perm = Permission(_permission);
        require(
            perm.deleteResources(_conts, _funcs),
            "deleteResources failed."
        );
        return true;
    }

    /// @notice Set multiple permissions to the account
    /// @param _account The account to be setted
    /// @param _permissions The multiple permissions to be setted
    /// @return true if successed, otherwise false
    function setAuthorizations(address _account, address[] _permissions)
        external
        hasPermission(builtInPermissions[3])
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(
                auth.setAuth(_account, _permissions[i]),
                "setAuthorizations failed."
            );

        return true;
    }

    /// @notice Set permission to the account
    /// @param _account The account to be setted
    /// @param _permission The permission to be setted
    /// @return true if successed, otherwise false
    function setAuthorization(address _account, address _permission)
        external
        hasPermission(builtInPermissions[3])
        returns (bool)
    {
        require(
            auth.setAuth(_account, _permission),
            "setAuthorization failed."
        );
        return true;
    }

    /// @notice Cancel the account's muliple permissions
    /// @param _account The account to be canceled
    /// @param _permissions The multiple permissions to be canceled
    /// @return true if successed, otherwise false
    function cancelAuthorizations(address _account, address[] _permissions)
        external
        hasPermission(builtInPermissions[4])
        returns (bool)
    {
        for (uint i = 0; i < _permissions.length; i++)
            require(
                auth.cancelAuth(_account, _permissions[i]),
                "cancelAuthorizations failed."
            );

        return true;
    }

    /// @notice Cancel the account's permission
    /// @param _account The account to be canceled
    /// @param _permission The permission to be canceled
    /// @return true if successed, otherwise false
    function cancelAuthorization(address _account, address _permission)
        external
        hasPermission(builtInPermissions[4])
        returns (bool)
    {
        require(
            auth.cancelAuth(_account, _permission),
            "cancelAuthorization failed."
        );
        return true;
    }

    /// @notice Clear the account's permissions
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearAuthorization(address _account)
        external
        hasPermission(builtInPermissions[4])
        returns (bool)
    {
        require(auth.clearAuth(_account), "clearAuthorization failed.");
        return true;
    }
}
