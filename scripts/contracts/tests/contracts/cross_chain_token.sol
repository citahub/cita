pragma solidity ^0.4.18;

contract MyToken {
    /* This creates an array with all balances */
    mapping (address => uint256) public balanceOf;

    /* Initializes contract */
    function MyToken() {
        balanceOf[msg.sender] = 100;
    }
    
    /* Send coins */
    function transfer(address _to, uint256 _value) {
        require(balanceOf[msg.sender] >= _value);
        // Check if the sender has enough
        require(balanceOf[_to] + _value >= balanceOf[_to]);
        // Check for overflows
        balanceOf[msg.sender] -= _value;
        // Subtract from the sender
        balanceOf[_to] += _value;
        // Add the same to the recipient
    }
    
    function get_banlance(address _to) returns (uint256) {
        return balanceOf[_to];
    }
    
    event cross_chain(uint256 from_chain_id, uint256 to_chain_id, address dest_contract, uint256 hasher);
    
    // chain id must be first argument, relayer can extract chain_id
    function send_to_side_chain(uint256 to_chain_id, address dest_contract, uint256 _value) {
        require(balanceOf[msg.sender] >= _value);
        balanceOf[msg.sender] -= _value;
        // cross_chain(chainmanager.chain_id, msg.to, to_chain_id, dest_contract, RECV_FUNC_HASHER);
        cross_chain(0, to_chain_id, dest_contract, 0xf3701642);
    }
    
    function recv_from_side_chain(bytes raw_tx, bytes block_header, bytes receipt_merkle_tree_proof) {
        //require(raw_tx.dest_contract == this);
        // Check dest contract address
        //require(raw_tx.to_chain_id == chainmanager.chain_id);
        // Check chain_id
        // Check tx proof after valid check because verify_tx_proof will record tx hash to prevent duplication use the proof
        //require(verify_tx_proof(raw_tx, block_header, receipt_merkle_tree_proof));
        //balanceOf[raw_tx.sender] += 1;
    }
}

