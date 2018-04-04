pragma solidity ^0.4.19;

contract Reverted {
    uint t;

    function Reverted() public {
        assert(1 > 2);
        t = 100;
    }
}
