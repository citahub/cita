pragma solidity ^0.4.11;

interface PermissionInterface {
    // grant the permission to a user
    // permission: "send": sent tx  
    // permission: "create": create contract
    function grant_permission(string permission, string user); 
    // revoke the permission of a user
    // permission: "send": sent tx  
    // permission: "create": create contract
    function revoke_permission(string permission, string user);
    // query users of the permission
    // permission: "send": sent tx  
    // permission: "create": create contract
    function query_users_of_permission(string permission) returns (string);
    // query the user's permission 
    // "create": create contract
    // "send": send tx
    // "create send": both
    function query_permission() returns (string);
}
