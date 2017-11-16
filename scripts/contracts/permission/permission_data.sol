pragma solidity ^0.4.14;

import "./group_manager.sol";
import "./role_manager.sol";
import "./auth_manager.sol";
import "./util.sol";

/// @title The data of permission system
contract PermissionData {

    using GroupManager for *;
    using RoleManager for *;
    using AuthorizationManager for *;
    using Util for *;

    GroupManager.Groups groups;
    RoleManager.Roles roles;
    AuthorizationManager.Authorization auth;
    // Cache for the user appling to join the group
    mapping(bytes32 => address[]) group_applicants;
    mapping(bytes32 => address[]) group_resignations;
    mapping(address => bytes32[]) user_groups;
    bytes32[] group_names;
    bytes32[] role_names;
    address superAdmin;

    // Read???
    bytes32[21] permissions = [
        // About transaction. SendTx???LaunchTx???. Should the basic permission of the others
        bytes32("LaunchTx"),
        // Include: contract permission: DealTx.CreateContract
        bytes32("DealTx"),

        // About contract
        bytes32("CreateContract"),
        bytes32("UpdateContract"),
        bytes32("RunContract"),

        // About group
        bytes32("CreateGroup"),
        bytes32("UpdateGroup"),
        bytes32("DeleteGroup"),
        bytes32("ReadGroup"),

        // About role
        bytes32("CreateRole"),
        bytes32("UpdateRole"),
        bytes32("DeleteRole"),
        bytes32("ReadRole"),

        // About authorization
        bytes32("CreateAuth"),
        bytes32("UpdateAuth"),
        bytes32("DeleteAuth"),
        bytes32("ReadAuth"),

        // About key
        bytes32("AddKey"),
        bytes32("Freezekey"),
        bytes32("ActiveKey"),
        bytes32("ResetKey")
    ];

    enum Permissions {
        // About transaction
        // About transaction. SendTx???LaunchTx???. Should be the basic permission of others
        LaunchTx,
        // Include: contract permission: DealTx.CreateContract
        DealTx,

        // About contract
        CreateContract,
        UpdateContract,
        RunContract,

        // About group
        CreateGroup,
        UpdateGroup,
        DeleteGroup,
        ReadGroup,

        // About role
        CreateRole,
        UpdateRole,
        DeleteRole,
        ReadRole,

        // About authorization
        CreateAuth,
        UpdateAuth,
        DeleteAuth,
        ReadAuth,

        // About key
        AddKey,
        Freezekey,
        ActiveKey,
        ResetKey
    }
}
