pragma solidity ^0.4.18;

/*
 * contract address: 0x00000000000000000000000000000000013241a5
 * ======= permission_system.sol:PermissionSystem =======
 * Function signatures:
 * 7d4b8ef4: addPermissions(bytes32,bytes32[],bytes32,bytes32)
 * 9c61abc3: applyGroup(bytes32)
 * a4fc9eea: approveApply(bytes32,bytes32,bytes32)
 * 38d45528: approveQuit(bytes32,bytes32,bytes32)
 * 8bccbee0: cancelAuthorization(bytes32,bytes32,bytes32)
 * 64469324: checkPermission(address,bytes32,bytes32,bytes32,bytes32)
 * 7a733b63: deleteGroup(bytes32,bytes32,bytes32)
 * 81c25a73: deletePermissions(bytes32,bytes32[],bytes32,bytes32)
 * 14cfb0f0: deleteRole(bytes32,bytes32,bytes32)
 * a1596cae: grantRole(bytes32,bytes32,bytes32,address[])
 * 81126873: initAuthorization(bytes32,bytes32)
 * db70783b: initGroup(bytes32,address[],bool)
 * 59bb0539: initRole(bytes32,bytes32[])
 * 82dfcf7b: modifyGroupName(bytes32,bytes32,bytes32,bytes32)
 * 1c89463c: modifyRoleName(bytes32,bytes32,bytes32,bytes32)
 * 61fc7cd1: modifySubSwitch(bytes32,bytes32,bytes32,bool)
 * ee10a057: modifyProfile(bytes32,bytes32,bytes32,string)
 * 5848ee39: newGroup(bytes32,bytes32,address[],bool,uint8,bytes32,string)
 * dddb6902: newRole(bytes32,bytes32,bytes32,bytes32[],uint8)
 * b45b9543: queryAllGroups()
 * deb9c0d3: queryAllRoles()
 * f861d87d: queryAncestors(bytes32)
 * f5ff3f20: queryBasicPermission()
 * 152db21e: queryGroupInfo(bytes32)
 * 95b771dc: queryGroups(address)
 * 5257a658: queryParentGroup(bytes32)
 * 626b04a2: queryPermissions(bytes32)
 * 5ec62af2: queryProfile(bytes32)
 * 620bb3ca: queryRoles(bytes32)
 * f1d08f9e: querySubGroups(bytes32)
 * 32af8c0f: querySubSwitch(bytes32)
 * b8bfa5a9: querySuperAdmin()
 * 080f69f4: queryUsers(bytes32)
 * 3be6f3d7: quitGroup(bytes32)
 * 50769b88: revokeRole(bytes32,bytes32,bytes32,address[])
 * 57d51bc1: setAuthorization(bytes32,bytes32,bytes32)
 */

/// @title The interface of the permission system
interface PermissionSysInterface {

    event GroupNewed(address indexed _user, bytes32 indexed _group, bytes32 indexed _newGroup);
    event GroupInited(bytes32 indexed _root, address[] _adamEve, bool indexed _subSwitch);
    event RoleInited(bytes32 indexed _basic, bytes32[] _permissions);
    event AuthorizationInited(bytes32 indexed _group, bytes32 indexed _role);
    event GroupApplied(address indexed _user, bytes32 indexed _group);
    event ApplyApproved(address indexed _user, bytes32 indexed _group, bytes32 indexed _resource, bytes32 _role);
    event RoleGranted(bytes32 indexed _group, bytes32 indexed _resource, bytes32 indexed _role, address[] _users);
    event RoleRevoked(bytes32 indexed _group, bytes32 indexed _resource, bytes32 indexed _role, address[] _users);
    event GroupQuitted(address indexed _user, bytes32 indexed _group);
    event QuitApproved(address indexed _user, bytes32 indexed _group, bytes32 indexed _resource, bytes32 _role);

    function initGroup(
        bytes32 _root,
        address[] _adamEve,
        bool _subSwitch
    ) public returns (bool);

    function initRole(
        bytes32 _basic,
        bytes32[] _permissions
    ) public returns (bool);

    function initAuthorization(
        bytes32 _group,
        bytes32 _role
    ) public returns (bool);

    function applyGroup(
        bytes32 _group
    ) public returns (bool);

    function approveApply(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role
    ) public returns (bool);

    function grantRole(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        address[] _users
    ) public returns (bool);

    function revokeRole(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        address[] _users
    ) public returns (bool);
    
    function quitGroup(
        bytes32 _group
    ) public returns (bool);

    function approveQuit(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role
    ) public returns (bool);

    function newGroup(
        bytes32 _name,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        uint8 _op,
        bytes32 _role,
        string _profile
    ) public returns (bool);

    function deleteGroup(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role
    ) public returns (bool);
    
    function modifyGroupName(
        bytes32 _oldName,
        bytes32 _newName,
        bytes32 _resource,
        bytes32 _role
    ) public returns (bool);

    function modifySubSwitch(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        bool _newSubSwitch
    ) public returns (bool);

    function modifyProfile(
        bytes32 _group,
        bytes32 _resource,
        bytes32 _role,
        string _newProfile
    ) public returns (bool);

    function newRole(
        bytes32 _group,
        bytes32 _newName,
        bytes32 _role,
        bytes32[] _newPermissions,
        uint8 _op
    ) public returns (bool);

    function deleteRole(
        bytes32 _role,
        bytes32 _group,
        bytes32 _resource
    ) public returns (bool);

    function modifyRoleName(
        bytes32 _oldName,
        bytes32 _newName,
        bytes32 _group,
        bytes32 _resource
    ) public returns (bool);

    function addPermissions(
        bytes32 _role,
        bytes32[] _permissions,
        bytes32 _group,
        bytes32 _resource
    ) public returns (bool);

    function deletePermissions(
        bytes32 _role,
        bytes32[] _permissions,
        bytes32 _group,
        bytes32 _resource
    ) public returns (bool);

    function setAuthorization(
        bytes32 _group,
        bytes32 _role,
        bytes32 _resource
    ) public returns(bool);

    function cancelAuthorization(
        bytes32 _group,
        bytes32 _role,
        bytes32 _resource
    ) public returns(bool);

    function checkPermission(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    ) view public returns (bool);

    function queryRoles(
        bytes32 _group
    ) view public returns (bytes32[]);

    function queryPermissions(
        bytes32 _role
    ) view public returns (bytes32[]);

    function queryGroups(
        address _user
    ) view public returns (bytes32[]);

    function queryGroupInfo(
        bytes32 _group
    ) view public returns (address[], bytes32[], bool, bytes32, string);

    function queryAncestors(
            bytes32 _group
    ) view public returns (bytes32[]);

    function queryUsers(
        bytes32 _group
    ) view public returns (address[]);

    function querySubGroups(
        bytes32 _group
    ) view public returns (bytes32[]);

    function querySubSwitch(
        bytes32 _group
    ) view public returns (bool);

    function queryParentGroup(
        bytes32 _group
    ) view public returns (bytes32);

    function queryProfile(
        bytes32 _group
    ) view public returns (string);

    function queryAllGroups() view public returns (bytes32[]);

    function queryAllRoles() view public returns (bytes32[]);

    function querySuperAdmin() view public returns (address);

    function queryBasicPermission() view public returns (bytes32[]);
}
