pragma solidity ^0.4.14;

/// @title The interface of the permission system
interface PermissionSysInterface {

    event GroupInited(bytes32 indexed _root, address[] _adamEve, bool indexed _subSwitch);
    event RoleInited(bytes32 indexed _basic, bytes32[] _permissions);
    event AuthorizationInited(bytes32 indexed _group, bytes32 indexed _role);
    event GroupApplied(address indexed _user, bytes32 indexed _group);
    event ApplyApproved(address indexed _user, bytes32 indexed _group, bytes32 indexed _resource, bytes32 _role);
    event RoleGranted(bytes32 indexed _group, bytes32 indexed _resource, bytes32 indexed _role, address[] _users);
    event RoleRevoked(bytes32 indexed _group, bytes32 indexed _resource, bytes32 indexed _role, address[] _users);
    event GroupQuitted(address indexed _user, bytes32 indexed _group);
    event QuitApproved(address indexed _user, bytes32 indexed _group, bytes32 indexed _resource, bytes32 _role);

    function applyGroup(bytes32 _group) public returns (bool);

    function approveApply(bytes32 _group, bytes32 _resource, bytes32 _role) public returns (bool);

    function grantRole(bytes32 _group, bytes32 _resource, bytes32 _role, address[] _users) public returns (bool);

    function revokeRole(bytes32 _group, bytes32 _resource, bytes32 _role, address[] _users) public returns (bool);
    
    function quitGroup(bytes32 _group) public returns (bool);

    function approveQuit(bytes32 _group, bytes32 _resource, bytes32 _role) public returns (bool);

    function newGroup(
        bytes32 _name,
        bytes32 _newName,
        address[] _newUsers,
        bool _newSubSwitch,
        uint8 _op,
        bytes32 _role
    ) public returns (bool);

    function deleteGroup(bytes32 _name, bytes32 _resource, bytes32 _role) public returns (bool);
    
    function modifyGroupName(bytes32 _oldName, bytes32 _newName, bytes32 _resource, bytes32 _role) public returns (bool);

    function modifySubSwitch(bytes32 _group, bytes32 _resource, bytes32 _role, bool _newSubSwitch) public returns (bool);

    function newRole(
        bytes32 _name,
        bytes32 _newName,
        bytes32 _role,
        bytes32[] _newPermissions,
        uint8 _op
    ) public returns (bool);

    function deleteRole(bytes32 _role, bytes32 _group, bytes32 _resource) public returns (bool);

    function modifyRoleName(bytes32 _oldName, bytes32 _newName, bytes32 _group, bytes32 _resource) public returns (bool);

    function addPermissions(bytes32 _name, bytes32[] _permissions, bytes32 _group, bytes32 _resource) public returns (bool);

    function deletePermissions(bytes32 _name, bytes32[] _permissions, bytes32 _group, bytes32 _resource) public returns (bool);

    function setAuthorization(bytes32 _group, bytes32 _role, bytes32 _resource) public returns(bool);

    function cancelAuthorization(bytes32 _group, bytes32 _role, bytes32 _resource) public returns(bool);

    function check(
        address _user,
        bytes32 _userGroup,
        bytes32 _resourceGroup,
        bytes32 _role,
        bytes32 _permission
    ) constant returns (bool);

    function queryRole(bytes32 _group) constant returns (bytes32[]);

    function queryPermissions(bytes32 _role) constant returns (bytes32[]);

    function queryGroup(address _user) constant returns (bytes32[]);

    function queryUsers(bytes32 _group) constant returns (address[]);

    function querySubGroups(bytes32 _group) constant returns (bytes32[]);

    function querySubSwitch(bytes32 _group) constant returns (bool);
}
