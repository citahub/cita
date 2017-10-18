pragma solidity ^0.4.14;

contract SimpleStorage {
    uint storedData;

    event Set(uint);

    function set(uint x) {
        uint num = 100000;

        for (uint i = 0; i < num; i++) {
            storedData = i;
        }

        Set(x);
    }

    function get() constant returns (uint) {
        return storedData;
    }
}
