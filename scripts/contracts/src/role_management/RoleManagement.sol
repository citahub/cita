pragma solidity 0.4.24;

import "./RoleCreator.sol";
import "../common/Check.sol";
import "../../interaction/interface/IRoleAuth.sol";
import "../../interaction/interface/IRoleManagement.sol";

/// @title Role management contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @notice The address: 0xffffffffffffffffffffffffffffffffff020007
///         The interface the can be called: All
contract RoleManagement is IRoleManagement, Check {

    RoleCreator roleCreator = RoleCreator(roleCreatorAddr);
    IRoleAuth roleAuth = IRoleAuth(roleAuthAddr);

    mapping(address => address[]) internal accounts;
    mapping(address => address[]) internal roles;

    /// @notice Create a new role
    /// @param _name The name of role
    /// @param _permissions The permissions of role
    /// @return New role's address
    function newRole(bytes32 _name, address[] _permissions)
        external
        hasPermission(builtInPermissions[5])
        returns (address roleid)
    {
        return roleCreator.createRole(_name, _permissions);
    }

    /// @notice Delete the role
    /// @param _role The address of role
    /// @return true if successed, otherwise false
    function deleteRole(address _role)
        external
        hasPermission(builtInPermissions[6])
        returns (bool)
    {
        // Cancel the role of the account's which has the role
        require(roleAuth.clearAuthOfRole(_role), "clearAuthOfRole failed.");

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
        hasPermission(builtInPermissions[7])
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
        hasPermission(builtInPermissions[7])
        returns (bool)
    {
        // Set the authorization of all the account's which has the role
        require(
            roleAuth.setPermissionsOfRole(_role, _permissions),
            "setPermissionsOfRole failed."
        );

        Role roleContract = Role(_role);
        require(
            roleContract.addPermissions(_permissions),
            "addPermissions failed."
        );
        return true;
    }

    /// @notice Delete permissions of role
    /// @param _role The address of role
    /// @param _permissions The permissions of role
    /// @return true if successed, otherwise false
    function deletePermissions(address _role, address[] _permissions)
        external
        hasPermission(builtInPermissions[7])
        returns (bool)
    {
        Role roleContract = Role(_role);
        require(
            roleContract.deletePermissions(_permissions),
            "deletePermissions failed."
        );

        // Cancel the authorization of all the account's which has the role
        require(
            roleAuth.cancelPermissionsOfRole(_role, _permissions),
            "cancelPermissionsOfRole failed."
        );
        return true;
    }

    /// @notice Set the role to the account
    /// @param _account The account to be setted
    /// @param _role The role to be setted
    /// @return true if successed, otherwise false
    function setRole(address _account, address _role)
        external
        hasPermission(builtInPermissions[8])
        returns (bool)
    {
        require(roleAuth.setRole(_account, _role), "setRole failed.");
        return true;
    }

    /// @notice Cancel the account's role
    /// @param _account The account to be canceled
    /// @param _role The role to be canceled
    /// @return true if successed, otherwise false
    function cancelRole(address _account, address _role)
        external
        hasPermission(builtInPermissions[9])
        returns (bool)
    {
        require(roleAuth.cancelRole(_account, _role), "cancelRole failed.");
        return true;
    }

    /// @notice Clear the account's role
    /// @param _account The account to be cleared
    /// @return true if successed, otherwise false
    function clearRole(address _account)
        external
        hasPermission(builtInPermissions[9])
        returns (bool)
    {
        require(roleAuth.clearRole(_account), "clearRole failed.");
        return true;
    }
}
