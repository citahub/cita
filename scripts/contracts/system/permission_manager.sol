pragma solidity ^0.4.14;

import "./strings.sol";
import "./permission_interface.sol";

contract PermissionManager is PermissionInterface {

    using strings for *;

    // The users having the send tx permission
    address[] senders;
    // The users having the create contract permission
    address[] creators;

    // The permission
    enum UserPermission { None, Send, Create }
    // Record the permission of the user
    mapping(address => UserPermission) public user_permission;

    modifier hasOverPermission(address _user, uint8 _permission) {
        require(uint8(user_permission[_user]) >= _permission);
        _;
    }

    modifier hasPermission(address _user, uint8 _permission) {
        require(uint8(user_permission[_user]) == _permission);
        _;
    }

    modifier noPermission(address _user, uint8 _permission) {
        require(uint8(user_permission[_user]) < _permission);
        _;
    }

    modifier notSelf(address _user) {
        require(_user!= msg.sender);
        _;
    }

    /// @dev Setup
    function PermissionManager(address[] _senders, address[] _creators) {
        // Init the senders
        for(uint i = 0; i < _senders.length; i++) {
            senders.push(_senders[i]);
            user_permission[_senders[i]] = UserPermission.Send;
        }
        // Init the creators
        for(uint j = 0; j < _creators.length; j++) {
            creators.push(_creators[j]);
            user_permission[_creators[j]] = UserPermission.Create;
        }
    }

    function grantPermission(address _user, uint8 _permission)
        public 
        hasPermission(msg.sender, _permission)
        noPermission(_user, _permission)
        returns (bool) 
    {
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

    function revokePermission(address _user, uint8 _permission)
        public 
        hasOverPermission(msg.sender, _permission)
        hasPermission(_user, _permission)
        notSelf(_user)
        returns (bool) 
    {
        user_permission[_user] = UserPermission.None;

        if (_permission == uint8(UserPermission.Send))
            deleteUser(_user, senders);

        if (_permission == uint8(UserPermission.Create))
            deleteUser(_user, creators);

        RevokePermission(_user, _permission);
        return true;
    }

    /// @dev Cancat
    function concatUser(address[] _users) internal returns (string userList) {
        if (_users.length > 0)
            userList = toString(_users[0]);

        for (uint i = 1; i < _users.length; i++)
            userList = userList.toSlice().concat(toString(_users[i]).toSlice());
    }

    /// @dev Delete the user of the users
    function deleteUser(address _user, address[] storage _users) internal returns (bool) {
        var index = indexUser(_user,  _users);
        // Not found
        if (index >= _users.length)
            return false;

        // Remove the gap
        for (uint i = index; i < _users.length - 1; i++) {
            _users[i] = _users[i + 1];
        }
        // Also delete the last element
        delete _users[_users.length - 1];
        _users.length--;
        return true;
    }

    /// @dev Get the index in the nodes_of_start array
    function indexUser(address _user, address[] _users) internal returns (uint) {
        // Find the index of the member
        for (uint i = 0; i < _users.length; i++) {
            if (_user == _users[i])
                return i;
        }
        // If i == length, means not find
        return i;
    }

    /// @dev Address to string
    /// @return The returned string is ABI encoded
    function toString(address x) internal returns (string) {
        bytes memory b = new bytes(20);

        for (uint i = 0; i < 20; i++)
            b[i] = byte(uint8(uint(x) / (2**(8*(19 - i)))));

        return string(b);
    }

    function queryUsersOfPermission(uint8 _permission) constant returns (string) {
        if (_permission == uint8(UserPermission.Send))
            return concatUser(senders);
        if (_permission == uint8(UserPermission.Create))
            return concatUser(creators);
    }

    function queryPermission(address _user) constant returns (uint8) {
        return uint8(user_permission[_user]);
    }
}
