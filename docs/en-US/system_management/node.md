# Nodes Management

The nodes in CITA are divided into consensus nodes and ordinary nodes. The transactions are sorted and packed into blocks by consensus nodes, and then broadcast to other nodes. Ordinary nodes do not participate in consensus process and only synchronize all the original data on the chain.

The public blockchain don't have any permission management, which means that any node can join the blockchain network and synchronize the transaction data. They can also participate in consensus under certain conditions. As a enterprise-level blockchain platform, CITA implement permission management for both consensus nodes and ordinary nodes. For a node with failed authentication, even if this node can communicate with other nodes at the network layer, the other nodes will still refuse to establish communication session with it, so as to avoid information leakage.

## Ordinary Nodes Management  (White-list)

Currently, CITA adopts white-list for ordinary nodes permission management. Each node saves the node White-list configuration file locally, which records the information of nodes that are allowed to connect for p2p communication and data synchronization, including its public key, IP address, port, and corresponding identity information.
The white-list is generated and distributed by the management organization, and the operation and maintenance personnel can maintain and manage it. Depending on nodeself，it is permitted to connect and configure several additional nodes to perform data analysis and other tasks.

### Operations

The management of ordinary nodes includes adding and deleting. Let's illustrate it with examples:

#### Add ordinary nodes

1. Assume that the current working directory is under  `../cita/target/install/` :

    ```bash
    $ pwd
    ../cita/target/install
    $ ls test-chain/
      0  1  2  3  template
    ```
     The current nodes' public key address are saved in file `template/authorities.list` and the block information of genesis is saved in file `template/configs/genesis.json`. We have four nodes currently.

2. Generate new nodes:

    ```bash
    $ ./scripts/create_cita_config.py append --chain_name test-chain --node "127.0.0.1:4004"
    $ ls test-chain/
      0  1  2  3  4  template
    ```

    - append: add new node with specified IP
    - The script will generate a new node（No.4）automatically and insert the new node's ip and port configuration into `test-chain/*/network.toml`

3. Start new nodes:

    Just start the new node in normal process. It can connect to the network and start to synchronize the block data on the chain automatically. **Note that the new node here is only an ordinary node and does not participate in the consensus process, which means it can only synchronize data and receive JSON-RPC Request**。

    ```bash
    $ ./bin/cita setup test-chain/4
    $ ./bin/cita start test-chain/4
    ```

    For the original node, if it is running, after network.toml is modified, they will automatically reload the p2p network configuration and try to find new nodes.

#### Delete ordinary nodes

Go to the corresponding node directory, find `network.toml`, delete the corresponding `peers` entry.

## Consensus Nodes Management

As a blockchain framework for enterprise-level applications, CITA needs to ensure that supervisors can get related permission to manage consensus nodes, including adding and deleting consensus nodes and other operations. For the consensus microservice, it is necessary to provide supervisors with an interface for reading the consensus node list in real time.
Compared with centralized management which cannot guarantee the security and consistency of the consensus node list of each node, CITA adopts contract method to implement the consensus nodes management which can effectively guarantee the security and consistency.

When initializing genesis block, an administrator address needs to be initialized first. Then both the administrator address and consensus nodes management contract address need to be written into the genesis block file of each node. The contents of genesis block cannot be modified after initialization. After the blockchain starts, the management contract will be written into the genesis block. Out-of-chain operators can manage consensus nodes by calling the RPC interface.

The management of consensus nodes includes adding, deleting, and getting consensus nodes list:

* Adding operation can only be performed by administrator;
* Deleting operation can only be performed by administrator;
* Get consensus nodes list by calling interface

### Add consensus nodes

Only after a node is added as ordianry nodes, it can make the request to become a consensus node. Then, it is necessary to approve the request by administrator. After all of these operations, a consensus node can be added sucessfully. If a ordinary node want to be updated to a consensus node, detailed steps are as follows:

Let's illustrate how a ordinary node become a consensus node with an example. We will use `approveNode(address)` in the process.

Consensus nodes management contract is system contract and written into genesis by default. Below are function signatures of management contract:

```
contract address: 0xffffffffffffffffffffffffffffffffff020001
Function signatures:
    dd4c97a0: approveNode(address)
    2d4ede93: deleteNode(address)
    30ccebb5: getStatus(address)
    609df32f: listNode()
    645b8b1b: status(address)
```

*First, it needs to start a chain with four nodes. check the [getting_started](../chain/getting_started)*

we will use [cita-cli](https://github.com/cryptape/cita-cli) command mode to show the presentation.

#### Get the consensus nodes list:

```bash
$ cita-cli scm NodeManager listNode --url http://127.0.0.1:1337
```

output：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000004000000000000000000000000d0f05f536ffc6a5d27b17cd14a544418b0500e92000000000000000000000000cccda2959225fc79f61f99bed213bd1172a7ea830000000000000000000000000014e2a75b4b5399f09732ecb6ed1a5b389c9e700000000000000000000000003e91911ba91b10dfa41f0a34d4a3c5a4f838eace"
}
```

The return value is the list of current consensus nodes address.

Now we need to upgrade the new ordinary node to a consensus node by constructing transactions. In the demo, the public key address of the new node is `0x59a316df602568957f47973332f1f85ae1e2e75e`.

#### Approve the node


* Send transaction

```bash
$ cita-cli scm NodeManager approveNode \
    --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

`admin-privkey`: private key，used to verify the transaction information. The system's default private keys can be viewed in[systerm contract](./chain/config_tool)

output:

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x286402ed9e27a11dbbcf5fc3b8296c36f66cb39068a3c468c632ee370e81bdb2",
    "status": "OK"
  }
}
```

* Get receipt

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x286402ed9e27a11dbbcf5fc3b8296c36f66cb39068a3c468c632ee370e81bdb2 \
    --url http://127.0.0.1:1337
```

output:

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xe7bb245d4ee718703746241c8cf3352063c7761b789b79a74a991d993f6d48e1",
    "blockNumber": "0xba",
    "contractAddress": null,
    "cumulativeGasUsed": "0x11660",
    "errorMessage": null,
    "gasUsed": "0x11660",
    "logs": [
      {
        "address": "0xffffffffffffffffffffffffffffffffff020001",
        "blockHash": "0xe7bb245d4ee718703746241c8cf3352063c7761b789b79a74a991d993f6d48e1",
        "blockNumber": "0xba",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0x5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb",
          "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
        ],
        "transactionHash": "0x286402ed9e27a11dbbcf5fc3b8296c36f66cb39068a3c468c632ee370e81bdb2",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      }
    ],
    "logsBloom": "0x00000000000000020040000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000010000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000020000000000800000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0x286402ed9e27a11dbbcf5fc3b8296c36f66cb39068a3c468c632ee370e81bdb2",
    "transactionIndex": "0x0"
  }
}
```

We can get some related information form the `log` field.

#### Check the current number of consensus nodes

```
$ cita-cli scm NodeManager listNode --url http://127.0.0.1:1337
```
output:
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000d0f05f536ffc6a5d27b17cd14a544418b0500e92000000000000000000000000cccda2959225fc79f61f99bed213bd1172a7ea830000000000000000000000000014e2a75b4b5399f09732ecb6ed1a5b389c9e700000000000000000000000003e91911ba91b10dfa41f0a34d4a3c5a4f838eace00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
}
```

It can be seen that an address is added at the end of the returned result, which means, the newly added node has become a consensus node sucessfully.

### Delete the consensus node

How to delete a consensus node? Here is an example.

It's operated by the admin address with calling the `deleteNode(address)` interface.

#### Get the current consensus nodelist

```bash
$ cita-cli scm NodeManager listNode --url http://127.0.0.1:1337
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000d0f05f536ffc6a5d27b17cd14a544418b0500e92000000000000000000000000cccda2959225fc79f61f99bed213bd1172a7ea830000000000000000000000000014e2a75b4b5399f09732ecb6ed1a5b389c9e700000000000000000000000003e91911ba91b10dfa41f0a34d4a3c5a4f838eace00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
}
```

(There are five consensus nodes because of previous action.)

#### Delete the consensus node

* send Transaction

```bash
$ cita-cli scm NodeManager deleteNode \
    --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

`--admin-privkey` meading private key of admin.

Check the [config_tool](./chain/config_tool)。

output:

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x01a4eac643589780090d5ed9fa1ac56d139776dd79ebc74a6414594d4d607393",
    "status": "OK"
  }
}
```

* get the receipt

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x01a4eac643589780090d5ed9fa1ac56d139776dd79ebc74a6414594d4d607393 \
    --url http://127.0.0.1:1337
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xc57c25447a24f7bd2b0d5699dfa151ba42456309d9da70101cfb3f599ec77c8d",
    "blockNumber": "0x1db",
    "contractAddress": null,
    "cumulativeGasUsed": "0x558c",
    "errorMessage": null,
    "gasUsed": "0x558c",
    "logs": [
      {
        "address": "0xffffffffffffffffffffffffffffffffff020001",
        "blockHash": "0xc57c25447a24f7bd2b0d5699dfa151ba42456309d9da70101cfb3f599ec77c8d",
        "blockNumber": "0x1db",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0x74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d5",
          "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
        ],
        "transactionHash": "0x01a4eac643589780090d5ed9fa1ac56d139776dd79ebc74a6414594d4d607393",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      }
    ],
    "logsBloom": "0x00000000000000020040000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000100000000000000000000000000000000000000000000000000000000000800000000000000002000000000000000000000000400000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0x01a4eac643589780090d5ed9fa1ac56d139776dd79ebc74a6414594d4d607393",
    "transactionIndex": "0x0"
  }
}
```

We can get some related information form the `log` field.

#### Check the current consensus nodes lists

```bash
$ cita-cli scm NodeManager listNode --url http://127.0.0.1:1337
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000004000000000000000000000000d0f05f536ffc6a5d27b17cd14a544418b0500e92000000000000000000000000cccda2959225fc79f61f99bed213bd1172a7ea830000000000000000000000000014e2a75b4b5399f09732ecb6ed1a5b389c9e700000000000000000000000003e91911ba91b10dfa41f0a34d4a3c5a4f838eace"
}
```

The return value is the list of consensus nodes.
(We can know that the consensus node `0x59a316df602568957f47973332f1f85ae1e2e75e` is already deleted.)

> There will be slightly different during the operation.
