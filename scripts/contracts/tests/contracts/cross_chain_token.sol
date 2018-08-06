pragma solidity ^0.4.24;

import "../../src/native/CrossChain.sol";

contract MyToken is CrossChain {
    /* This creates an array with all balances */
    mapping (address => uint256) public balanceOf;

    /* Initializes contract */
    function MyToken(uint256 _balance) public {
        balanceOf[msg.sender] = _balance;
    }

    /* Send coins */
    function transfer(address _to, uint256 _value) public {
        require(balanceOf[msg.sender] >= _value);
        // Check if the sender has enough
        require(balanceOf[_to] + _value >= balanceOf[_to]);
        // Check for overflows
        balanceOf[msg.sender] -= _value;
        // Subtract from the sender
        balanceOf[_to] += _value;
        // Add the same to the recipient
    }

    function getBalance(address addr) public view returns (uint256) {
        return balanceOf[addr];
    }

    uint256 txDataSize = 0x20;

    function sendToSideChain(
        uint32 toChainId,
        address destContract,
        bytes txData
    ) public {
        require(txData.length == txDataSize);
        uint256 value;
        assembly {
            value := mload(add(txData, 0x20))
        }
        require(balanceOf[msg.sender] >= value);
        bytes4 destFuncHasher = bytes4(keccak256("recvFromSideChain(bytes)"));
        sendTransaction(toChainId, destContract, destFuncHasher);
        balanceOf[msg.sender] -= value;
    }

    // verify_proof need:
    // check from_chain_id in ChainManager.sideChains
    // check to_chain_id == my chain_id
    // check dest_contract == this
    // check hasher == RECV_FUNC_HASHER
    // check cross_chain_nonce == cross_chain_nonce
    // extract origin tx sender and origin tx data
    function recvFromSideChain(bytes txProof) public {
        address sender;
        bytes memory txData;
        (sender, txData) = verifyTransaction(txProof, txDataSize);
        uint256 value;
        assembly {
            value := mload(add(txData, 0x20))
        }
        balanceOf[sender] += value;
    }
}
