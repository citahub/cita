pragma solidity ^0.4.15;

contract SimpleStorage {
    uint storedData;
    event Init(address, uint);
    event Set(address, uint);

    function SimpleStorage() public {
        storedData = 100;
        Init(msg.sender, 100);
    }

    event Stored(uint);

    function set(uint x) public {
        Stored(x);
        storedData = x;
        Set(msg.sender, x);
    }

    function get() public constant returns (uint) {
        return storedData;
    }
}
