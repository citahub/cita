pragma solidity ^0.4.18;

import "./role.sol";


contract RoleCreator {
    event RoleCreated(address indexed _id, bytes32 indexed _name, address[] indexed _permissions);

    function createRole(bytes32 _name, address[] _permissions) 
        public
        returns (Role roleAddress)
    {
        return _createRole(_name, _permissions);
    }

    function _createRole(bytes32 _name, address[] _permissions)
        private
        returns (Role roleAddress)
    {
        Role role = new Role(_name, _permissions);
        RoleCreated(role, _name, _permissions);
        return role;
    }
}
