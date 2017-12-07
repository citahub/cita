pragma solidity ^0.4.18;

interface PermissionInterface {

    event GrantPermission(address _user, uint8 _permission);
    event RevokePermission(address _user, uint8 _permission);

    /// @dev Grant the permission to a user
    function grantPermission(address _user, uint8 _permission) public returns (bool);
    /// @dev Revoke the permission of a user
    function revokePermission(address _user, uint8 _permission) public returns (bool);
    /// @dev Query users of the permission
    function queryUsersOfPermission(uint8 _permission) view public returns (string);
    /*
     * @dev Query the user's permission:
     * @return 0: "None" - no pemission
     * @return 1: "Create" - create contract
     * @return 2: "Send" - send tx
     */
    function queryPermission(address _user) view public returns (uint8);
}
