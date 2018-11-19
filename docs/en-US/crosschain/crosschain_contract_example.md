# Cross-chain Contract Writing and Operation Guide

## How to write cross-chain contract ?

Let's illustrate it with a [contract sample](https://github.com/cryptape/cita/blob/develop/scripts/contracts/tests/contracts/MyToken.sol).

This is a token contract used to transfer tokens cross chains and need to be deployed in both sidechain and mainchain.

Compared with ordinary token contract, two functions, `send_to_side_chain` and `recv_from_side_chain`, are added which can be used in cross-chain token transfer.

`send_to_side_chain` is designed to deduct tokens from one chain.

After the transaction is executed, use JsonRPC `cita_getTransactionProof` interface to get the transaction proof.

Send this proof to function `recv_from_side_chain` of another chain. After verification, this proof can be parsed out original transaction information,  which is transfer amount in here. In the end, add the same number of tokens to the same user.

Now we have completed the token transfer cross chains.

### Notices

1. `RECV_FUNC_HASHER` is the function signature of `recv_from_side_chain`. You can use following command to view the detail：

  ```shell
  solc --hashes cross_chain_token.sol
  ```

2. `DATA_SIZE` is the size of the data transfered cross the chain, that is, the total size of all other arguments except for the first two arguments (fixed arguments) in function  `send_to_side_chain`.

3. `nonce`  is designed to protect cross-chain transaction from replay attacks,  same function as `nonce` in `CITA`.

   The cross-chain transaction data must be transfered strictly in the order of transaction execution，so the count of `crosschain_nonce` is designed as autoincrement. 

   Before sending the proof to another chain, call `get_cross_chain_nonce` to get current `nonce`. Then, there are tools also used to parse the proof and extract the `nonce`. Comparing two nonce values for equality, Only if they are equal, the transaction can be sent successfully. Otherwise, the proof may need to be discarded since the proof has been sent before, or the transaction needs to wait until preorder transaction was sent successfully.

4. `event cross_chain(uint256 from_chain_id, uint256 to_chain_id, address dest_contract, uint256 hasher, uint256 nonce)`in `send_to_side_chain` privide necessary information for cross-chain transaction. 

   Do not modify or add other `event` to this function.

5. Users need to parse the data into the original type following the signature of `send_to_side_chain`.

## Operations

### Create, register，and launch sidechains

Currently, sidechains are managed using the [ChainManager](https://github.com/cryptape/cita/blob/develop/scripts/contracts/system/chain_manager.sol).

* Generate the private key and address of sidechain verification node, then call `newSideChain` in mainchain to create a sidechain ID using the sidechain address. 
* Call `enableSideChain` in mainchain to enable the sidechain with specified Id.
* When enable sidechain contract `ChainManager` inside genesis block on sidechain, use the sidechain Id , mainchain id and mainchain verification node applied for in the previous step as aruguments.
* Launch the sidechain now.

### Deploy crosschain contract

Deploy crosschain contract in both sidechain and mainchain and obtain these two contract address.

### Send cross-chain transaction

Call `send_to_side_chain` of sender chain,  using the ID and contract address of the other chain, and the token transfer amount as the arguments, to send the cross chain token transaction，and get the transaction hash.
No distination is made between mainchain and sidechain in operations.

### Use the relayer tool to send transactions to receiver chain

Use the cross-chain token transaction hash, the ID of the sender chain, and a configuration file as input arguments to invoke the tool:

```shell
cita-relayer-parser -c SEND_CHAIN_ID -t TX_HASH -f relayer-parser.json
```

In current，the configuration file `relayer-parser.json` has two parameters:

* The private key used by the tool
* The Jsonrpc network address of all related chains, using chain ID as index

For example:

```json
{
    "private_key": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "chains": [
        {
            "id": 1,
            "servers": [
                { "url": "http://127.0.0.1:11337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        },
        {
            "id": 2,
            "servers": [
                { "url": "http://127.0.0.1:21337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        }
    ]
}
```

Main task of this tool：

* Based on the input parameters, check the transaction proof of the sender chain.
* According to the transaction proof, get the ID of the receiver chain.
* Send the proof to the receiver chain.

### Verify if the cross-chain token transfer is successful

Query the number of tokens for current user using the query interface ( `get_balance` in contract sample) in both sender chain and receiver chain.