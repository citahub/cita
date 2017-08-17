pragma solidity ^0.4.15;

contract SimpleStorage {
    uint storedData;
    event Init(address, uint);
    event Set(address, uint);
    
    function SimpleStorage() {
        storedData = 100;
        Init(msg.sender, 100);
    }
    
    event Stored(uint);

    function set(uint x)  {
        Stored(x);
        storedData = x;
        Set(msg.sender, x);
    }

    function get() constant returns (uint) {
        return storedData;
    }
}
