pragma solidity ^0.4.11;

interface PermissionInterface {
    /// 0: "None" - no pemission
    /// 1: "Create" - create contract
    /// 2: "Send" - send tx

    // grant the permission to a user
    function grantPermission(address _user, uint8 _permission) returns (bool); 
    // revoke the permission of a user
    function revokePermission(address _user, uint8 _permission) returns (bool);
    // query users of the permission
    function queryUsersOfPermission(uint8 _permission) constant returns (string);
    // query the user's permission 
    function queryPermission(address _user) returns (uint8);
}
