pragma solidity ^0.4.14;

contract HelloWorld {
  uint balance;

  function update(uint amount) public returns (address, uint) {
    balance += amount;
    return (msg.sender, balance);
  }
}
