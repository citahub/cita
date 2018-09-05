pragma solidity ^0.4.24;

import "./role_creator.sol";
import "./role_auth.sol";
import "../common/address.sol";


/// @title Role management contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020007
///         The interface the can be called: All
contract RoleManagement is ReservedAddress {

    RoleCreator roleCreator = RoleCreator(roleCreatorAddr);
    RoleAuth auth = RoleAuth(roleAuthAddr);

    mapping(address => address[]) internal accounts;
    mapping(address => address[]) internal roles;

    modifier checkPermission(address _permission) {
        require(auth.hasPermission(msg.sender, _permission), "permission denied.");
        _;
    }

    /// @notice Create a new role
    /// @param _name The name of role
    /// @param _permissions The permissions of role
    /// @return New role's address
    function newRole(bytes32 _name, address[] _permissions)
        external
        checkPermission(builtInPermissions[5])
        returns (address roleid)
    {
        return roleCreator.createRole(_name, _permissions);
    }

    /// @notice Delete the role
    /// @param _role The address of role
    /// @return true if successed, otherwise false
    function deleteRole(address _role)
        external
        checkPermission(builtInPermissions[6])
        returns (bool)
    {
        // Cancel the role of the account's which has the role
        require(auth.clearAuthOfRole(_role), "clearAuthOfRole failed.");

        Role roleContract = Role(_role);
        roleContract.deleteRole();

        return true;
    }

    /// @notice Update role's name
    /// @param _role The address of role
    /// @param _name The new name of role
    /// @return true if successed, otherwise false
    function updateRoleName(address _role, bytes32 _name)
        external
        checkPermission(builtInPermissions[7])
        returns (bool)
    {
        Role roleContract = Role(_role);
        return roleContract.updateName(_name);
    }

    /// @notice Add permissions of role
    /// @param _role The address of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function addPermissions(address _role, address[] _permissions)
        external
        checkPermission(builtInPermissions[7])
        returns (bool)
    {
        // Set the authorization of all the account's which has the role
        require(auth.setPermissionsOfRole(_role, _permissions), "setPermissionsOfRole failed.");

        Role roleContract = Role(_role);
        require(roleContract.addPermissions(_permissions), "addPermissions failed.");
        return true;
    }

    /// @notice Delete permissions of role
    /// @param _role The address of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function deletePermissions(address _role, address[] _permissions)
        external
        checkPermission(builtInPermissions[7])
        returns (bool)
    {
        Role roleContract = Role(_role);
        require(roleContract.deletePermissions(_permissions), "deletePermissions failed.");

        // Cancel the authorization of all the account's which has the role
        require(auth.cancelPermissionsOfRole(_role, _permissions), "cancelPermissionsOfRole failed.");
        return true;
    }

    /// @notice Set the role to the account
    /// @param _account The account to be setted
    /// @param _role The role to be setted
    /// @return true if successed, otherwise false
    function setRole(address _account, address _role)
        external
        checkPermission(builtInPermissions[8])
        returns (bool)
    {
        require(auth.setRole(_account, _role), "setRole failed.");
        return true;
    }

    /// @notice Cancel the account's role
    /// @param _account The account to be canceled
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function cancelRole(address _account, address _role)
        external
        checkPermission(builtInPermissions[9])
        returns (bool)
    {
        require(auth.cancelRole(_account, _role), "cancelRole failed.");
        return true;
    }

    /// @notice Clear the account's role
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearRole(address _account)
        external
        checkPermission(builtInPermissions[9])
        returns (bool)
    {
        require(auth.clearRole(_account), "clearRole failed.");
        return true;
    }
}
