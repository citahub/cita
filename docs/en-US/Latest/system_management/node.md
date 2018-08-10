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
    $ ls test-chain/
      0  1  2  3  template
    ```
     The current nodes' public key address are saved in file `template/authorities.list` and the block information of genesis is saved in file `template/configs/genesis.json`. We have four nodes currently.

2. Generate new nodes：

    ```bash
    $ ./scripts/create_cita_config.py append --node "127.0.0.1:4004"
    $ ls test-chain/
      0  1  2  3  4  template
    ```

    - append：add new node with specified IP
    - The script will generate a new node（No.4）automatically and insert the new node's ip and port configuration into `test-chain/*/network.toml`

3. Start new nodes：

    Just start the new node in normal process. It can connect to the network and start to synchronize the block data on the chain automatically. **Note that the new node here is only an ordinary node and does not participate in the consensus process, which means it can only synchronize data and receive Jsonrpc Request**。

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
    <td>approveNode(address)</td>
    <td>Administrator</td>
    <td>New consensus node address</td>
    <td>Bool (indicating whether this operaiton is sucessful )</td>
    <td>After the newNode(address) operation is successful, you can call this interface to make a approvement that the node are allowed a consensus node, The node status shows start in here.</td>
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
    <td>Acquire consensus nodes list in which all nodes are in start status</td>
  </tr>
  <tr>
    <td>getStatus(address)</td>
    <td>Ordinary (read only)</td>
    <td>Node address</td>
    <td>
      node status (uint8):
      <ul>
        <li>0: close</li>
        <li>1: start</li>
      </ul>
    </td>
    <td>Get the status of nodes</td>
  </tr>
</table>

### Add consensus nodes

Only after a node is added as ordianry nodes, it can make the request to become a consensus node. Then, it is necessary to approve the request by administrator. After all of these operations, a consensus node can be added sucessfully. If a ordinary node want to be updated to a consensus node, detailed steps are as follows：

Let's illustrate how a ordinary node become a consensus node with an example. We will use `approveNode(address)` in the process.

Consensus nodes management contract is system contract and written into genesis by default. Below are function signatures of management contract：

```
contract address: 0xffffffffffffffffffffffffffffffffff020001
Function signatures:
    dd4c97a0: approveNode(address)
    2d4ede93: deleteNode(address)
    30ccebb5: getStatus(address)
    609df32f: listNode()
    645b8b1b: status(address)
```

#### First, get the consensus nodes list：

```bash
$ curl -X POST --data '{"jsonrpc":"2.0","method":"call", "params":[{"to":"0xffffffffffffffffffffffffffffffffff020001", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

{"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a1"}

```

- to: consensus nodes management contract address
- data: function signature of listNode()

The return value is the list of current consensus nodes address.

Now we need to upgrade the new ordinary node to a consensus node by constructing transactions. In the demo, the public key address of the new node is `59a316df602568957f47973332f1f85ae1e2e75e`.

#### Construct and send transactions

The standard of calling contract follows [ABI](https://solidity.readthedocs.io/en/develop/abi-spec.html), we privide a transaction tool `make_tx.py`：

1. Construct approveNode transaction inforamtion

    ```bash
    $ python3 make_tx.py --to "ffffffffffffffffffffffffffffffffff020001" --code "dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```

    - privkey: private key，used to verify the transaction information. The system's default private keys can be viewed in[systerm contract](./chain/config_tool)
    - The first 8 bits are the function hash value and the next 64 bits are node address (less than 64 bits are filled with 0).

2. Send transaction

    ```bash
    $ python3 send_tx.py
    --> {"params": ["0a5b0a283030303030303030303030303030303030303030303030303030303030303030303133323431613212013118fface20420ef012a24dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e124177a025eaafcda1f28f4b2eedd1c8ecb0d339b141e452a3bd8736cd9abc45e7387af7ab41045df5646aa411e7cac1b3a8b78e7efc81b356877afcf4a2080c06d500"], "jsonrpc": "2.0", "method": "sendRawTransaction", "id": 1}
    <-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4","status":"OK"}} (200 OK)
    ```

3. Get receipt

    ```bash
    $ python3 get_receipt.py
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
          "address": "0xffffffffffffffffffffffffffffffffff020001",
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

4. View the current number of consensus nodes

    ```bash
    $ curl -X POST --data '{"jsonrpc":"2.0","method":"call", "params":[{"to":"0xffffffffffffffffffffffffffffffffff020001", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

    {"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a100000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"}
    ```
    It can be seen that an address is added at the end of the returned result, which means, the newly added node has become a consensus node sucessfully.

### 删除共识节点

按照以上方法，普通节点可以被添加为共识节点，那么共识节点怎么删除呢？下面是删除共识节点的示例：

删除共识节点需要由管理员来完成， 具体操作需要调用 deleteNode(address) 合约方法。

#### 首先，获取当前链上的共识节点列表：

```bash
curl -X POST --data '{"jsonrpc":"2.0","method":"call", "params":[{"to":"0xffffffffffffffffffffffffffffffffff020001", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337
{"jsonrpc":"2.0","id":2,"result":"0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000500000000000000000000000009fab2c5c372c3013484709979b318c5c9817bb0000000000000000000000000cd5c86c841af6ebe19c6a0734630bb885152bc83000000000000000000000000e8efbbe34f4f439b73549bc162ecc0fbc06b789b000000000000000000000000fc92fbdafb5edcfe4773080ee2cec6123958f49a00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"}
```

- to 为共识节点管理合约地址
- data 为listNode（）的Function signature

返回值为目前的共识节点地址列表（可以看到新添加的共识节点地址 `59a316df602568957f47973332f1f85ae1e2e75e`）

#### 构造交易格式删除共识节点

调用合约遵循 [ABI](https://solidity.readthedocs.io/en/develop/abi-spec.html), 提供工具 `make_tx.py`：

1. 构造 deleteNode 交易信息

    ```bash
    $ cd script/txtool/txtool

    $ python3 make_tx.py --to "ffffffffffffffffffffffffffffffffff020001" --code "2d4ede9300000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```

    - privkey 是私钥，用来签证，确认交易信息合法，系统默认的几个私钥可以看 [系统合约相关](../chain/config_tool)
    - code 前 8 位是函数 hash 值，即 deleteNode 对应的 hash，后面 64 位是函数的参数 address 的值，即节点地址，不足 64 位用 0 补齐。

    生成的交易信息存放在 `../output/transaction/deploycode` 中

2. 发送交易

    ```bash
    $ python3 send_tx.py 
    --> {"jsonrpc": "2.0", "method": "sendRawTransaction", "params": ["0x0a83010a2866666666666666666666666666666666666666666666666666666666666666666666303230303031120656415a39553718c0843d20e7032a242d4ede9300000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e3220000000000000000000000000000000000000000000000000000000000000000038011241958dc34c4524e928193902732e9b9445eb585577b5b1d78267b75423951580f31964bcd8b979a25d6cfa1578f012a3f69df91fabd0ce83289fbe9f65cbffaf4a01"], "id": 1}
    ```

3. 获取回执

    ```bash
    $ python3 get_receipt.py 
    {
      "transactionHash": "0x80fb09bb710a1d742e19fb5195ea95faa4c192c9f1802055ecaf2d687b21cecd",
      "transactionIndex": "0x0",
      "blockHash": "0x8b962b7af8332819bcfa54285c9c8553004fe2757362474c1e5dba200094aed8",
      "blockNumber": "0x195",
      "cumulativeGasUsed": "0x4fc6",
      "gasUsed": "0x4fc6",
      "contractAddress": null,
      "logs": [
          {
            "address": "0xffffffffffffffffffffffffffffffffff020001",
            "topics": [
            "0x74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d5",
            "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
            ],
            "data": "0x",
            "blockHash": "0x8b962b7af8332819bcfa54285c9c8553004fe2757362474c1e5dba200094aed8",
            "blockNumber": "0x195",
            "transactionHash": "0x80fb09bb710a1d742e19fb5195ea95faa4c192c9f1802055ecaf2d687b21cecd",
            "transactionIndex": "0x0",
            "logIndex": "0x0",
            "transactionLogIndex": "0x0"
          }
      ],
      "root": null,
      "logsBloom": "0x00000000000000020040000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000100000000000000000000000000000000000000000000000000000000000800000000000000002000000000000000000000000400000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "errorMessage": null
    }

    ```
4. 查看当前的共识节点数

    ```bash
    $  curl -X POST --data '{"jsonrpc":"2.0","method":"call", "params":[{"to":"0xffffffffffffffffffffffffffffffffff020001", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337
    {"jsonrpc":"2.0","id":2,"result":"0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000400000000000000000000000009fab2c5c372c3013484709979b318c5c9817bb0000000000000000000000000cd5c86c841af6ebe19c6a0734630bb885152bc83000000000000000000000000e8efbbe34f4f439b73549bc162ecc0fbc06b789b000000000000000000000000fc92fbdafb5edcfe4773080ee2cec6123958f49a"}
    ```
    返回值为目前的共识节点地址列表（可以看到新添加的共识节点地址 59a316df602568957f47973332f1f85ae1e2e75e 已经删除了）

> The return value of the above code has been partially deleted, so the actual operation will be slightly different.

