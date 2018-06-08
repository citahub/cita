pragma solidity ^0.4.24;

contract MyToken {
    /* This creates an array with all balances */
    mapping (address => uint256) public balanceOf;

    /* Initializes contract */
    constructor(uint256 _balance) public {
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

    function get_balance(address _to) public view returns (uint256) {
        return balanceOf[_to];
    }

    event cross_chain(uint32 from_chain_id, uint32 to_chain_id, address dest_contract, uint256 hasher, uint256 nonce);
    event recv_cross_chain(uint sender, bytes data);

    uint256 crosschain_send_nonce;
    uint256 crosschain_recv_nonce;
    uint256 RECV_FUNC_HASHER = 0x9b8a78eb;
    // size as agrs[2..] of send_to_side_chain
    uint256 DATA_SIZE = 0x20;  // for this demo is _value

    function get_cross_chain_nonce() public view returns (uint256) {
        return crosschain_recv_nonce;
    }

    function send_to_side_chain(uint32 to_chain_id, address dest_contract, uint256 _value) public {
        require(balanceOf[msg.sender] >= _value);
        balanceOf[msg.sender] -= _value;
        uint32 from_chain_id = get_from_chain_id();
        emit cross_chain(from_chain_id, to_chain_id, dest_contract, RECV_FUNC_HASHER, crosschain_send_nonce);
        crosschain_send_nonce = crosschain_send_nonce + 1;
    }

    function get_from_chain_id() public view returns (uint32) {
        // ChainManager: Contract
        address chainManagerAddr = 0x00000000000000000000000000000000000000CE;
        // getChainId() function
        bytes4 getChainIdHash = bytes4(keccak256("getChainId()"));

        uint256 result;
        uint256 cid;

        assembly {
            let ptr := mload(0x40)
            mstore(ptr, getChainIdHash)
            result := call(20000, chainManagerAddr, 0, ptr, 0x4, ptr, 0x20)
            if eq(result, 0) { revert(ptr, 0) }
            cid := mload(ptr)
        }
        return uint32(cid);
    }

    // verify_proof need:
    // check from_chain_id in ChainManager.sideChains
    // check to_chain_id == my chain_id
    // check dest_contract == this
    // check hasher == RECV_FUNC_HASHER
    // check cross_chain_nonce == cross_chain_nonce
    // extract origin tx sender and origin tx data
    function recv_from_side_chain(bytes tx_proof) public {
        uint hasher = RECV_FUNC_HASHER;
        uint nonce = crosschain_recv_nonce;
        uint len = tx_proof.length;
        uint sender;
        uint data_size = DATA_SIZE;
        bytes memory data = new bytes(data_size);

        // (origin_tx_sender, origin_tx_data) = CrossChainVerify(this, RECV_FUNC_HASHER, crosschain_recv_nonce, proof_len, tx_proof));
        assembly {
            let _calldata := mload(0x40)   //Find empty storage location using "free memory pointer"
            mstore(_calldata, 0)           //Place signature at begining of empty storage
            mstore(add(_calldata, 0x04), address)         //Place first argument directly next to signature
            mstore(add(_calldata, 0x24), hasher)          //Place second argument next to first, padded to 32 bytes
            mstore(add(_calldata, 0x44), nonce)
            mstore(add(_calldata, 0x64), len)
            calldatacopy(add(_calldata, 0x84), 0x44, sub(calldatasize, 0x44)) // skip hasher and first arg

            switch call(                     //This is the critical change (Pop the top stack value)
                    100000,                  //100k gas
                    0x1301,                  //To addr
                    0,                       //No value
                    _calldata,               //Inputs are stored at location _calldata
                    add(calldatasize, 0x40), //Inputs are xx bytes long
                    _calldata,               //Store output over input (saves space)
                    add(data_size, 0x20))    //Outputs less than calldatasize
            case 0 { revert(0, 0) }
            default {
                sender := mload(_calldata)
                returndatacopy(data, 0x20, data_size)
                mstore(0x40, _calldata)
            }
        }
        crosschain_recv_nonce = crosschain_recv_nonce + 1;

        emit recv_cross_chain(sender, data);

        address origin_tx_sender = address(sender);
        uint256 value;
        assembly {
            value := mload(data)
        }

        balanceOf[origin_tx_sender] += value;
    }
}

