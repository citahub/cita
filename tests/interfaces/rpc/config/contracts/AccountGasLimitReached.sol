pragma solidity ^0.4.14;

contract AccountGasLimitReached {
    bytes32[] balance;

    function AccountGasLimitReached() public {
        for (uint i = 0; i < 99999999999; i++) {
            balance.push(keccak256(i));
        }
    }
}
