pragma solidity ^0.4.0;
import "./strings.sol";

import "./permission_interface.sol";


contract PermissionManager is PermissionInterface {
    using strings for *;

    // the users having the send tx permission
    address[] senders;
    // the users having the create contract permission
    address[] creators;

    // the permission
    enum UserPermission { None, Send, Create }

    // record the permission fo the user
    mapping(address => UserPermission) public user_permission;

    event GrantPermission(address _user, uint8 _permission);
    event RevokePermission(address _user, uint8 _permission);

    // setup
    function PermissionManager(address[] _senders, address[] _creators) {
        // init the senders
        for(uint i = 0; i < _senders.length; i++) {
            senders.push(_senders[i]);
            user_permission[_senders[i]] = UserPermission.Send;
        }
        // init the creators
        for(uint j = 0; j < _creators.length; j++) {
            creators.push(_creators[j]);
            user_permission[_creators[j]] = UserPermission.Create;
        }
    }

    // sender grant the permission to a user
    function grantPermission(address _user, uint8 _permission) returns (bool) {
        // require sender has the permission
        if (_permission > uint8(user_permission[msg.sender]) || _permission < uint8(user_permission[_user]))
            return false;

        if (_permission == uint8(UserPermission.Send)) {
            senders.push(_user);
            user_permission[_user] = UserPermission.Send;
        }

        if (_permission == uint8(UserPermission.Create)) {
            creators.push(_user);
            user_permission[_user] = UserPermission.Create;
        }

        GrantPermission(_user, _permission);
        return true;
    }

    // revoke the role
    function revokePermission(address _user, uint8 _permission) returns (bool) {
        // require sender and user both have the permission
        if (_permission > uint8(user_permission[msg.sender]) || _permission != uint8(user_permission[_user]) || msg.sender == _user)
            return false;

        user_permission[_user] = UserPermission.None;

        if (_permission == uint8(UserPermission.Send))
            deleteUser(_user, senders);

        if (_permission == uint8(UserPermission.Create))
            deleteUser(_user, creators);

        RevokePermission(_user, _permission);
        return true;
    }

    // query users of the permission
    function queryUsersOfPermission(uint8 _permission) constant returns (string) {
        if (_permission == uint8(UserPermission.Send)) {
            return concatUser(senders);
        }
        if (_permission == uint8(UserPermission.Create)) {
            return concatUser(creators);
        }
    }

    // query the user's permission
    function queryPermission(address _user) returns (uint8) {
        return uint8(user_permission[_user]);
    }

    // cancat
    function concatUser(address[] _users) internal returns (string userList) {
        if (_users.length > 0) {
            userList = toString(_users[0]);
        }

        for (uint i = 1; i < _users.length; i++) {
            userList = userList.toSlice().concat(toString(_users[i]).toSlice());
        }
    }

    // delete the user of the users
    function deleteUser(address _user, address[] storage _users) internal returns (bool) {
        var index = indexUser(_user,  _users);
        // not found
        if (index >= _users.length) {
            return false;
        }

        // remove the gap
        for (uint i = index; i < _users.length - 1; i++) {
            _users[i] = _users[i + 1];
        }
        // also delete the last element
        delete _users[_users.length - 1];
        _users.length--;
        return true;
    }

    // interface: get the index in the nodes_of_start array
    function indexUser(address _user, address[] _users) internal returns (uint) {
        // find the index of the member
        for (uint i = 0; i < _users.length; i++) {
            if (_user == _users[i]) {
                return i;
            }
        }
        // if i == length, means not find
        return i;
    }

    // interface: address to string
    // the returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++) {
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));
        }

        return string(b);
    }
}
