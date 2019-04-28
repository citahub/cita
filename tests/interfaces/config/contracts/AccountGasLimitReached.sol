pragma solidity ^0.4.24;

contract AccountGasLimitReached {
    bytes32[] balance;

    constructor() public {
        for (uint i = 0; i < 99999999999; i++) {
            balance.push(keccak256(abi.encodePacked(i)));
        }
    }
}
