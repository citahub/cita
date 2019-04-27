pragma solidity ^0.4.24;

contract RevertedDemo {
    constructor() public {
        assert(false);
    }
}
