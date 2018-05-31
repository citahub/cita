# Nodes Management

The nodes in CITA are divided into consensus nodes and ordinary nodes. The transactions are sorted and packed into blocks by consensus nodes, and then broadcast to other nodes. Ordinary nodes do not participate in consensus process and only synchronize all the original data on the chain.

The public blockchain don't have any permission management, which means that any node can join the blockchain network and synchronize the transaction data. They can also participate in consensus under certain conditions. As a enterprise-level blockchain platform, CITA implement permission management for both consensus nodes and ordinary nodes. For a node with failed authentication, even if this node can communicate with other nodes at the network layer, the other nodes will still refuse to establish communication session with it, so as to avoid information leakage.

## Ordinary Nodes Management  (White-list)

Currently, CITA adopts white-list for ordinary nodes permission management. Each node saves the node White-list configuration file locally, which records the information of nodes that are allowed to connect for p2p communication and data synchronization, including its public key, IP address, port, and corresponding identity information. 
The white-list is generated and distributed by the management organization, and the operation and maintenance personnel can maintain and manage it. Depending on nodeself，it is permitted to connect and configure several additional nodes to perform data analysis and other tasks.

### Operations

The management of ordinary nodes includes adding and deleting. Let's illustrate it with examples:

#### Add ordinary nodes

1. Assume that the current working directory is under  `../cita/target/install/` ：

    ```bash
    $ pwd
    ../cita/target/install
    $ ls node/
      0  1  2  3  template
    ```
     The current nodes' public key address are saved in file `template/authorities.list` and the block information of genesis is saved in file `template/configs/genesis.json`. We have four nodes currently. 

2. Generate new nodes：

    ```bash
    $ ./scripts/create_cita_config.py append --node "127.0.0.1:4004"
    $ ls node/
      0  1  2  3  4  template
    ```
    
    - append：add new node with specified IP
    - The script will generate a new node（No.4）automatically and insert the new node's ip and port configuration into `node/*/network.toml`

3. Start new nodes：

    Just start the new node in normal process. It can connect to the network and start to synchronize the block data on the chain automatically. **Note that the new node here is only an ordinary node and does not participate in the consensus process, which means it can only synchronize data and receive Jsonrpc Request**。

    ```bash
    $ ./bin/cita setup node4
    $ ./bin/cita start node4
    ```

    For the original node, if it is running, after network.toml is modified, they will automatically reload the p2p network configuration and try to find new nodes.

#### Delete ordinary nodes

Go to the corresponding node directory, find `network.toml`, delete the corresponding `peers` entry.

## Consensus Nodes Management 

As a blockchain framework for enterprise-level applications, CITA needs to ensure that supervisors can get related permission to manage consensus nodes, including adding and deleting consensus nodes and other operations. For the consensus microservice, it is necessary to provide supervisors with an interface for reading the consensus node list in real time. 
Compared with centralized management which cannot guarantee the security and consistency of the consensus node list of each node, CITA adopts contract method to implement the consensus nodes management which can effectively guarantee the security and consistency.

When initializing genesis block, an administrator address needs to be initialized first. Then both the administrator address and consensus nodes management contract address need to be written into the genesis block file of each node. The contents of genesis block cannot be modified after initialization. After the blockchain starts, the management contract will be written into the genesis block. Out-of-chain operators can manage consensus nodes by calling the RPC interface.

The management of consensus nodes includes adding, deleting, and getting consensus nodes list:

* The adding operation is divided into initiation and confirmation. The node first initiates a request to becomes a consensus node. After confirmation by administrator (the account with the administrator role)，the node is added successfully;
* Deleting operation can only be performed by administrator;
* Get consensus nodes list by calling interface

### Consensus nodes management interface

<table>
  <tr>
    <th>Interface Name</th>
    <th>Permission Needed</th>
    <th>Incoming Parameters</th>
    <th>Return Value</th>
    <th>Detailed Discription</th>
  </tr>
  <tr>
    <td>newNode(address)</td>
    <td>Ordinary</td>
    <td>New node address</td>
    <td>Bool (indicating whether this operaiton is sucessful )</td>
    <td>If this operation is sucessful, the new node would be recoreded in the consensus nodes list and ready to become a consensus node. The node status shows new in here.</td>
  </tr>
  <tr>
    <td>approveNode(address)</td>
    <td>Administrator</td>
    <td>New consensus node address</td>
    <td>Bool (indicating whether this operaiton is sucessful )</td>
    <td>After the newNode(address) operation is successful, you can call this interface to make a approvement that the node are allowed a consensus node, The node status shows consensus in here.</td>
  </tr>
  <tr>
    <td>deleteNode(address)</td>
    <td>Administrator</td>
    <td>Node address</td>
    <td>Bool (indicating whether this operaiton is sucessful )</td>
    <td>If this operation is sucessful, the node would be deleted in the consensus nodes list. The node status shows close in here.</td>
  </tr>
  <tr>
    <td>listNode()</td>
    <td>Ordinary (read only)</td>
    <td>Null</td>
    <td>Address list(address[])</td>
    <td>Acquire consensus nodes list in which all nodes are in consensus status</td>
  </tr>
  <tr>
    <td>getStatus(address)</td>
    <td>Ordinary (read only)</td>
    <td>Node address</td>
    <td>
      node status (uint8):
      <ul>
        <li>0: close</li>
        <li>1: new</li>
        <li>2: consensus</li>
      </ul>
    </td>
    <td>Get the status of nodes</td>
  </tr>
</table>

### Add consensus nodes

Only after a node is added as ordianry nodes, it can make the request to become a consensus node. Then, it is necessary to approve the request by administrator. After all of these operations, a consensus node can be added sucessfully. If a ordnary want to be updated to a consensus, detailed steps are as follows：

* Submit the account address to the administrator;
* The node initiates a contract that records it as a consensus node, and the administrator completes the confirmation; 
* Modify the local white-list together with other nodes;
* Participate in consensus process after completing the block data synchronization.

Let's illustrate it with an example. We will use `newNode(address)`/`approveNode(address)` in here.

Consensus nodes management contract is system contract and written into genesis by default. Below are function signatures of management contract：

```
contract address: 0x00000000000000000000000000000000013241a2
Function signatures:
    dd4c97a0: approveNode(address)
    2d4ede93: deleteNode(address)
    30ccebb5: getStatus(address)
    609df32f: listNode()
    ddad2ffe: newNode(address)
    645b8b1b: status(address)
```

#### First, get the consensus nodes list：

```bash
$ curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

{"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a1"}

```

- to: consensus node management contract address
- data: function signature of listNode()

The return value is the list of current consensus nodes address.

Now we need to upgrade the new node to a consensus node by constructing a transaction. In the demo, the public key address of the new node is `59a316df602568957f47973332f1f85ae1e2e75e`.

#### Construct and send transaction 

The standard of calling contract follows [ABI](https://solidity.readthedocs.io/en/develop/abi-spec.html), we privide a transaction tool `make_tx.py`：

1. Construct newNode transaction information 

    ```bash
    $ cd script/txtool/txtool

    $ python make_tx.py --to "00000000000000000000000000000000013241a2" --code "ddad2ffe00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```

    - privkey: private key，used to verify the transaction information. The system's default private keys can be viewed in[systerm contract](https://cryptape.github.io/cita/en/usage-guide/smart-contract-guide/index.html#_5)
    - The first 8 bits are the function hash value and the next 64 bits are node address (less than 64 bits are filled with 0).

    Generated information is stored in `../output/transaction/deploycode`.

2. Send transaction

    ```bash
    $ python send_tx.py 
    --> {"params": ["0a5b0a283030303030303030303030303030303030303030303030303030303030303030303133323431613212013018fface20420dc012a24ddad2ffe00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e1241bc58c97ad8979f429bac343157fd8ecb193edb8255ca256ca077d352c24161e31ad634214f5443ea27ac95a3fe0b2ef2efc2a991b26c043f193325ea12033e7400"], "jsonrpc": "2.0", "method": "cita_sendTransaction", "id": 1}
    <-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323","status":"OK"}} (200 OK)
    ```

3. Get receipt

    ```bash
    $ python get_receipt.py
    {
      "contractAddress": null,
      "cumulativeGasUsed": "0x5615",
      "logs": [
        {
          "blockHash": "0xe5f58cbe8d4817adabec30c93662610fd4859cf87eecc2f3a4d483d74f9b256d",
          "transactionHash": "0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323",
          "transactionIndex": "0x0",
          "topics": [
            "0xfd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c1",
            "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
          ],
          "blockNumber": "0x89",
          "address": "0x00000000000000000000000000000000013241a2",
          "transactionLogIndex": "0x0",
          "logIndex": "0x0",
          "data": "0x"
        }
      ],
      "blockHash": "0xe5f58cbe8d4817adabec30c93662610fd4859cf87eecc2f3a4d483d74f9b256d",
      "transactionHash": "0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323",
      "root": null,
      "errorMessage": null,
      "blockNumber": "0x89",
      "logsBloom": "0x00000000000000020040000000000000000000000000000000000000000000200000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000208000000000000000000000000000",
      "transactionIndex": "0x0",
      "gasUsed": "0x5615"
    }
    ```

    If the transaction has not been processed yet, an error will occur. Try it several times to get the delivery confirmation . If `errorMassage` is null, it means normal and you can continue to the next step.

4. Construct approveNode transaction inforamtion

    ```bash
    $ python make_tx.py --to "00000000000000000000000000000000013241a2" --code "dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```
    Only function hash in code is changed.

5. Send transaction

    ```bash
    $ python send_tx.py
    --> {"params": ["0a5b0a283030303030303030303030303030303030303030303030303030303030303030303133323431613212013118fface20420ef012a24dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e124177a025eaafcda1f28f4b2eedd1c8ecb0d339b141e452a3bd8736cd9abc45e7387af7ab41045df5646aa411e7cac1b3a8b78e7efc81b356877afcf4a2080c06d500"], "jsonrpc": "2.0", "method": "cita_sendTransaction", "id": 1}
    <-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4","status":"OK"}} (200 OK)
    ```

6. Get receipt

    ```bash
    $ python get_receipt.py 
    {
      "contractAddress": null,
      "cumulativeGasUsed": "0xcf15",
      "logs": [
        {
          "blockHash": "0x17b24208a3f9af4d8ef65aa385116fc35e789477026bdc7a0fbef162047bdf98",
          "transactionHash": "0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4",
          "transactionIndex": "0x0",
          "topics": [
            "0x5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb",
            "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
          ],
          "blockNumber": "0x9b",
          "address": "0x00000000000000000000000000000000013241a2",
          "transactionLogIndex": "0x0",
          "logIndex": "0x0",
          "data": "0x"
        }
      ],
      "blockHash": "0x17b24208a3f9af4d8ef65aa385116fc35e789477026bdc7a0fbef162047bdf98",
      "transactionHash": "0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4",
      "root": null,
      "errorMessage": null,
      "blockNumber": "0x9b",
      "logsBloom": "0x00000000000000020040000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000040000000000000000000000000000000000000000000010000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000",
      "transactionIndex": "0x0",
      "gasUsed": "0xcf15"
    }
    ```

7. View the current number of consensus nodes 

    ```bash
    $ curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

    {"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a100000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"}
    ```
    It can be seen that an address is added at the end of the returned result, which means, the newly added node has become a consensus node is added sucessfully.

> The return value of the above code has been partially deleted, so the actual operation will be slightly different.

