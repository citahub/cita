pragma solidity ^0.4.18;

import "./role.sol";


contract RoleCreator {
    function createRole(bytes32 _name, address[] _permissions) 
        public
        returns (Role roleAddress)
    {
        return new Role(_name, _permissions);
    }
}
