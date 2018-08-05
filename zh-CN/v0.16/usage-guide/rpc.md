# CITA JSON-RPC

## JSON-RPC

* net_peerCount
* cita_blockNumber
* cita_sendTransaction
* cita_getBlockByHash
* cita_getBlockByNumber
* eth_getTransactionReceipt
* eth_getLogs
* eth_call
* cita_getTransaction
* eth_getTransactionCount
* eth_getCode
* eth_getAbi
* eth_getBalance
* eth_newFilter
* eth_newBlockFilter
* eth_uninstallFilter
* eth_getFilterChanges
* eth_getFilterLogs
* cita_getTransactionProof
* cita_getMetaData

***

### net_peerCount

当前的节点连接数。

* Parameters

None

* Returns

QUANTITY - integer of the number of connected peers.

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74}'

// Result
{
    "id": 74,
    "jsonrpc": "2.0",
    "result": "0x3"
}
```

***

### cita_blockNumber

返回当前块高度。

* Parameters

None

* Returns

`QUANTITY` - integer of current block height of CITA.

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":83}'

// Result
{
    "id": 83,
    "jsonrpc": "2.0",
    "result": "0x1d10"
}
```

***

### cita_sendTransaction

通过序列化交易调用区块链接口。

* Parameters

    1. `DATA`, The signed transaction data.

```js
const signed_data = "0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901"
params: [signed_data]
```

#### 生成签名数据的过程

#### 构造protobuf数据结构

```js
// Transaction
syntax = "proto3";
enum Crypto {
    SECP = 0;
    SM2 = 1;
}

message Transaction {
    string to = 1;
    string nonce = 2;
    uint64 quota = 3; // gas
    uint64 valid_until_block = 4;
    bytes data = 5;
    uint64 value = 6;
    uint32 chain_id = 7;
    uint32 version = 8;
}

message UnverifiedTransaction {
    Transaction transaction = 1;
    bytes signature = 2;
    Crypto crypto = 3;
}
```

#### 一些交易字段的说明

`to` 交易要发送到的地址。

调用合约时即为被调用合约的地址，部署合约时不填写该字段。

注意地址形式为40个十六进制字符(160位)，前导0不能省略，必须补全。

`nonce` 交易的填充字段。

区块链为了防止重放攻击，会拒绝接受重复的交易。

如果交易中仅包含有效的交易数据，会存在两个正常的交易完全一样的情况。

比如两次转账，如果转账人，收款人和金额都一样，那么两个交易就完全一样。

因此需要用户在交易中填充一些内容，使得两个交易不一样。

填充内容的形式为字符串，最大长度128，具体内容用户自己定义。

`quota` 交易的配额。

合约的能力是图灵完备的，具备强大功能的同时，也意味着交易执行过程中可能出现死循环等无法终止的情况。

因此，每个交易都要填写一个配额，指定交易最大执行时间，使得交易执行一定可以终止。

如果是solidity编写的合约，有配套工具可以估算调用合约中的函数需要的配额数量。

`valid_until_block` 交易上链的最大区块高度。

区块链的发送交易接口是异步的，交易进入交易池接口即返回。后面需要用户轮询交易什么时候真正上链得到确认。

视系统的拥堵情况，等待时间并没有一个确定的值。甚至有可能在后续环节发生错误，最终没有上链。

因此用户轮询一段时间之后，发现交易还没有上链，这时无法确定交易的状态(失败or拥堵)。

发送交易操作没有幂等性，因此无法通过重复发送交易来解决。

所以需要一个类似超时的机制，保证等待一段时间之后，交易的状态就确定是失败的。

该字段就是起这个作用，表示用户愿意等待交易上链的最大区块高度。

在区块链达到该高度之后，交易就确定不会再上链了，用户可以放心地重新发送交易，或者进行其他的后续处理。

实际使用中，可选的值在 当前区块高度 到 当前区块高度+100 之间。

#### 获得合约对应的bytecode

以下代码片段为示例代码，具体获取contract bytecode的方法参考[文档](https://ethereum.stackexchange.com/questions/8115/how-to-get-the-bytecode-of-a-transaction-using-the-solidity-browser)

[solidity](https://solidity.readthedocs.io/en/develop/)相关文档

```solidity
pragma solidity ^0.4.18;

contract SimpleStorage {
    uint storedData;

    function set(uint x) public {
        storedData = x;
    }

    function get() view public returns (uint) {
        return storedData;
    }
}

```

#### 构造签名

1. 构造Transaction对象tx，填充to, nonce, valid_until_block, quota, data, verion 6个字段。
2. tx对象protobuf序列化后 sha3 -> hash
3. 对 hash 进行签名 -> signature
4. 构造UnverifiedTransaction, 使用hash, signature, SECP填充UnverifiedTransaction  -> unverify_tx
5. unverify_tx对象protobuf序列化

伪代码描述:

```shell
let tx = Transaction::new();
// contract bytecode
let data = "6060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a723058207fbd8b51e2ecdeb2425f642d6602a4ff030351102fd7afbed80318e61fa462670029".from_hex();
tx.setdata(data);
if not depoly_contract {
    tx.setTo(address);
}

// current_block_height 可以通过CITA JSON-RPC接口 cita_blockNumber 获取
let valid_util_block = current_block_height + 88;
tx.set_valid_until_block(valid_util_block);

tx.set_nonce(nonce);

// 如果是solidity合约，可以通过solc --gas获取估算gas值，在此基础上加50%，或者在remix中运行获取的实际消耗gas上加一点
tx.set_quota(quota);

// 当前version 默认为0
tx.set_version(verison);

// language_depend_method和sign 分别是相应的语言或库中处理私钥和签名的方法
let privkey = language_depend_method("966fc50326cf6e2b30b06d8214737fcc2cda5bdce84eb23e14b6dbf3540d3f84");
let signature = sign(privkey, tx.protobuf_serialize().hash());

let unverify_tx = UnverifiedTransaction::new();
unverify_tx.transaction = tx;
unverify_tx.signature = signature;
unverify_tx.crypto = SECP;

params = unverify_tx.protobuf_serialize().to_hex_string();
```

#### 签名后的交易

```shell
0a910212013218fface20420a0492a8302606060405234156100105760006000fd5b610015565b60e0806100236000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604b5780636d4ce63c14606c576045565b60006000fd5b341560565760006000fd5b606a60048080359060200190919050506093565b005b341560775760006000fd5b607d60a3565b6040518082815260200191505060405180910390f35b8060006000508190909055505b50565b6000600060005054905060b1565b905600a165627a7a72305820942223976c6dd48a3aa1d4749f45ad270915cfacd9c0bf3583c018d4c86f9da200291241edd3fb02bc1e844e1a6743e8986a61e1d8a584aac26db5fa1ce5b32700eba5d16ba4c754731f43692f3f5299e85176627e55b9f61f5fe3e43572ec8c535b0d9201
```

* Returns

`DATA`, 32 Bytes - 交易hash

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0a910212013218fface20420a0492a8302606060405234156100105760006000fd5b610015565b60e0806100236000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604b5780636d4ce63c14606c576045565b60006000fd5b341560565760006000fd5b606a60048080359060200190919050506093565b005b341560775760006000fd5b607d60a3565b6040518082815260200191505060405180910390f35b8060006000508190909055505b50565b6000600060005054905060b1565b905600a165627a7a72305820942223976c6dd48a3aa1d4749f45ad270915cfacd9c0bf3583c018d4c86f9da200291241edd3fb02bc1e844e1a6743e8986a61e1d8a584aac26db5fa1ce5b32700eba5d16ba4c754731f43692f3f5299e85176627e55b9f61f5fe3e43572ec8c535b0d9201"],"id":1}'

// Result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0x4b5d67c1debdd5899fc7b5cd77e71987b8a2d174b361ca2dd4d713434b4ff037",
    "status": "OK"
  }
}

// 如果是近期发送的重复交易，则会提示重复交易

{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "Dup"//重复交易
  }
}


```

***

### cita_getBlockByHash

根据块hash查询块的信息。

* Parameters

    1. DATA, 32 Bytes - Hash of a block.
    2. Boolean - 是否返回交易信息(True: 返回详细交易列表| False: 只返回交易hash).

    ```shell
    params: [
        '0x296474ecb4c2c8c92b0ba7800a01530b70a6f2b6e76e5c2ed2f89356429ef329',
        true
    ]
    ```

* Returns

    1. Object - A block object, or null when no block was found:

* Example

```shell
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByHash","params":["0x296474ecb4c2c8c92b0ba7800a01530b70a6f2b6e76e5c2ed2f89356429ef329", true],"id":1}'

// Result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "version": 0,
    "hash": "0x5038c222d460c32fd06df36d58bb7cf5c368a55e207a46ecb18695451bfe4069",
    "header": {
      "timestamp": 1499756200950,
      "prevHash": "0xb28ec1911d375350664b9673a61d952e9a748f3e63606f1440f313c4911fde58",
      "proof": {
        "proposal": "0x0f25d396361c7d54bb16389f6a14bf95207915f91d180d382093e19adfc4133b",
        "height": 902,
        "round": 0,
        "commits": {
          "0x2b027dacd33a41ddb09e21805778f19951776ed5": "0x1532c58faedf9e103dd84aa6aacbd2121aa3a8102faa506e7e152fb10e45bafd31b1c3d372cf5d42f8b27a8bfea112ae194de76d99206f73837ad8c30267e6a501",
          "0x2d74a106464fbdf94e47bb28605a1aa244ab7788": "0x2ec53371cee732d59d23a58cf6cf53d818fb906fdeb5b0521a3a4cdbb75cf29658a1ff5fa95e4dc71563cbed10070c68e2eec0f812fa3be8e019b6df6e9ea66201",
          "0x3efd4959af72e1214ab83caa0f04a0cc3e54d383": "0xb051f0cc41bc3caed472d3c7a35e06d805e8f6d15ccb3efc257d71ee96932c5877a8e52fc29cb3bef73e0edbad62c617c4dd16763709b2604ab8b1db2d87736301",
          "0x5223818f7096520bfad68ce3d5ac959267dbc45f": "0x1cf6f8dc9654d461a317db199de0ed0d2d008762833b3358e269ceb3c412b60b3f1a2bd08f969e0dc1c9ebe1a0710002f853438a6ef3ea048de9b4e67387827400"
        }
      },
      "stateRoot": "0xe29266e5574bc0c848b513d36403d4da71f99f328d3324e8d3134809c33d4fb4",
      "transactionsRoot": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
      "receiptsRoot": "0x9646cf2572734b4b13fe1616446ab2658e208cfdbaf25e47ebea9b6327e10c5b",
      "gasUsed": "0x0",
      "number": "0x387",
      "proposer":"0xe6d430a2d830236d3774d148cbee72bbf26cd481",
    },
    "body": {
      "transactions": [
        {
          "hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
          "content": "0x0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010"
        }
      ]
    }
  }
}

```

***

### cita_getBlockByNumber

根据块高度查询块信息。

* Parameters

    1. `QUANTITY` - integer of a block height.
    2. `Boolean` - 是否返回交易信息(True: 返回详细交易列表| False: 只返回交易hash).

```js
params: [
   0x1da3,
   true
]
```

* Returns

See [cita_getBlockByHash](#cita_getblockbyhash)

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0xF9", true],"id":1}'
```

* Invalid Params

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0XF9", true],"id":1}'
#或者
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":[249, true],"id":1}'
```

高度参数可以用0x开头的十六进制。0X开头或者十进制整数都是错误的参数格式。

同 [cita_getBlockByHash](#cita_getblockbyhash)

***

### eth_getTransactionReceipt

根据交易hash获取交易回执。

* Parameters

    1. `DATA`, 32 Bytes - hash of a transaction

    ```js
    params: [
        "0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af"
    ]
    ```

* Returns

    Object - A receipt object:

    * transactionHash: DATA, 32 Bytes - hash of the transaction.
    * transactionIndex: QUANTITY - transaction index.
    * blockHash: DATA, 32 Bytes - hash of the block where this transaction was in. null when its not in block.
    * blockNumber: QUANTITY - block number where this transaction was in. null when its not in block.
    * cumulativeGasUsed: QUANTITY - The total amount of gas used when this transaction was executed in the block.
    * gasUsed: QUANTITY - The amount of gas used by this specific transaction alone.
    * contractAddress: DATA, 20 Bytes - The contract address created, if the transaction was a contract creation, otherwise null.
    * logs: Array - Array of log objects, which this transaction generated.
    * root: DATA 32 bytes of post-transaction stateroot.
    * errorMessage: String, execution error message.

    Receipt error messages:

    * No transaction permission.
    * No contract permission.
    * Not enough base gas.
    * Block gas limit reached.
    * Account gas limit reached.
    * Out of gas.
    * Jump position wasn't marked with JUMPDEST instruction.
    * Instruction is not supported.
    * Not enough stack elements to execute instruction.
    * Execution would exceed defined Stack Limit.
    * EVM internal error.
    * Mutable call in static context.
    * Out of bounds.
    * Reverted.

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":["0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236"],"id":1}'

// Result
{
    "jsonrpc":"2.0",
    "id":1,
    "result":{
        "transactionHash":"0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af",
        "transactionIndex":"0x0",
        "blockHash":"0xe068cf7299450b78fe97ed370fd9ebe09ecbd6786968e474fae862ccbd5c5020",
        "blockNumber":"0xa",
        "cumulativeGasUsed":"0x17a0f",
        "gasUsed":"0x17a0f",
        "contractAddress":"0xea4f6bc98b456ef085da5c424db710489848cab5",
        "logs":[
            {
                "address":"0xea4f6bc98b456ef085da5c424db710489848cab5",
                "topics":[
                    "0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"
                ],
                "data":"0x0000000000000000000000005b073e9233944b5e729e46d618f0d8edf3d9c34a0000000000000000000000000000000000000000000000000000000000000064",
                "blockHash":"0xe068cf7299450b78fe97ed370fd9ebe09ecbd6786968e474fae862ccbd5c5020",
                "blockNumber":"0xa",
                "transactionHash":"0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af",
                "transactionIndex":"0x0",
                "logIndex":"0x0",
                "transactionLogIndex":"0x0"
            }
        ],
        "root":"0xe702d654a292a8d074fd5ba3769b3dead8095d2a8f2207b3a69bd49c91a178af",
        "logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100040000000010000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    }
}

error：
{
  "contractAddress": "0xbb6f8266fae605da373c2526c386fe07542b4957",
  "cumulativeGasUsed": "0x0",
  "logs": [],
  "blockHash": "0x296474ecb4c2c8c92b0ba7800a01530b70a6f2b6e76e5c2ed2f89356429ef329",
  "transactionHash": "0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236",
  "root": null,
  "errorMessage": "No contract permission.",
  "blockNumber": "0x1da3",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0x0"
}
```

如果出现**Timeout，errorcode 99**,请查看可能的解决方法[Can't assign requested Address](https://vincent.bernat.im/en/blog/2014-tcp-time-wait-state-linux)

***

### eth_getLogs

根据Topic查询logs。

A note on specifying topic filters:

Topics are order-dependent. A transaction with a log with topics [A, B] will be matched by the following topic filters:

* `[]` "anything"
* `[A]` "A in first position (and anything after)"
* `[null, B]` "anything in first position AND B in second position (and anything after)"
* `[A, B]` "A in first position AND B in second position (and anything after)"
* `[[A, B], [A, B]]` "(A OR B) in first position AND (A OR B) in second position (and anything after)"

* Parameters

    1. `Object` - The filter object:
        * `fromBlock`: `QUANTITY|TAG` - (optional, default: `"latest"`) Integer block number(Hex string), or `"latest"` or `"earliest"`.
        * `toBlock`: `QUANTITY|TAG` - (optional, default: `"latest"`) Integer block number(Hex string), or `"latest"` or `"earliest"`.
        * `address`: `DATA|Array`, 20 Bytes - (optional) Contract address or a list of addresses from which logs should originate.
        * `topics`: `Array of DATA`,  - (optional) Array of 32 Bytes `DATA` topics. Topics are order-dependent. Each topic can also be an array of DATA with "or" options.

* Returns

`Array` - Array of log objects, or an empty array if no logs

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getLogs","params":[{"topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"],"fromBlock": "0x0"}],"id":74}'

// Result
{
    "jsonrpc":"2.0",
    "id":74,
    "result":[
        {
            "address":"0xea4f6bc98b456ef085da5c424db710489848cab5",
            "topics":[
                "0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"
            ],
            "data":"0x0000000000000000000000005b073e9233944b5e729e46d618f0d8edf3d9c34a0000000000000000000000000000000000000000000000000000000000000064",
            "blockHash":"0x3e83b74560860344f4c48d7b8089a18173aecd96b6b2148653c61b5d3f559764",
            "blockNumber":"0x4",
            "transactionHash":"0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af",
            "transactionIndex":"0x0",
            "logIndex":"0x0",
            "transactionLogIndex":"0x0"
        }
    ]
}

```

***

### eth_call

合约接口调用。

* Parameters

    1. Object - The transaction call object
        * from: DATA, 20 Bytes - (optional) The address the transaction is sent from.
        * to:   DATA, 20 Bytes - The address the transaction is directed to.
        * data: DATA - (optional) Hash of the method signature and encoded parameters. For details see [Ethereum Contract ABI](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI)
    2. QUANTITY - block parameter
        * HEX String - an integer block number
        * String "earliest" for the earliest/genesis block
        * String "latest" - for the latest mined block

```js
params: [{"from":"0xca35b7d915458ef540ade6068dfe2f44e8fa733c","to":"0xea4f6bc98b456ef085da5c424db710489848cab5","data":"0x6d4ce63c"}, "0x1d23"]
```

* Returns

`DATA`, 32 Bytes - the transaction hash.

* Example

contract中get方法Hash和编码后的数据

```shell
0x6d4ce63c
```

发送和获取数据

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call",
"params":[{"from":"0xca35b7d915458ef540ade6068dfe2f44e8fa733c","to":"0xea4f6bc98b456ef085da5c424db710489848cab5",
"data":"0x6d4ce63c"}, "0x6"],"id":2}'

// Result
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": "0x0000000000000000000000000000000000000000000000000000000000000064"
}

```

***

### cita_getTransaction

根据交易hash查询交易。

* Parameters

    1. `DATA`, 32 Bytes - hash of a transaction

    ```js
    params: [
        "0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236"
    ]
    ```

* Returns

    Object - A transaction object, or null when no transaction was found:

    * hash: DATA, 32 Bytes - hash of the transaction.
    * content: DATA, 交易内容.
    * blockHash: DATA, 32 Bytes - hash of the block where this transaction was in. null when its not in block.
    * blockNumber: QUANTITY - block number where this transaction was in. null when its not in block.
    * index: QUANTITY - integer of the transactions index position in the block. null when its not in block.

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getTransaction","params":["0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236"],"id":1}'

// Result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236",
    "content": "0x0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901",
    "blockNumber": "0x1da3",
    "blockHash": "0x296474ecb4c2c8c92b0ba7800a01530b70a6f2b6e76e5c2ed2f89356429ef329",
    "index": "0x0"
  }
}
```

***

### eth_getTransactionCount

获取账户发送交易的数量。

* Parameters

    1. `DATA`, 20 Bytes - address.
    2. `QUANTITY|TAG` - integer block number(Hex string), or the string "latest", "earliest"

* Returns

`QUANTITY` - integer of the number of transactions send from this address.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params":["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a","0x1F"],"id":1}'

// Result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x1"
}

```

***

### eth_getCode

获取合约代码。

* Parameters

    1. `DATA`, 20 Bytes - address.
    2. `QUANTITY|TAG` - integer block number(Hex string), or the string "latest", "earliest"

* Returns

`DATA` - the code from the given address.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab5", "0x1F"],"id":1}'

// Result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x60606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a0465190029"
}
```

***

### eth_getAbi

获取合约ABI

* Parameters

    1. `DATA`, 20 Bytes - address.
    2. `QUANTITY|TAG` - integer block number(Hex string), or the string "latest", "earliest"

* Returns

`DATA` - the abi from the given address.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getAbi","params":["73552bc4e960a1d53013b40074569ea05b950b4d", "latest"],"id":1}'

// Result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x4ed3885e000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001275b7b22636f6e7374616e74223a66616c73652c22696e70757473223a5b7b226e616d65223a2278222c2274797065223a2275696e74323536227d5d2c226e616d65223a22736574222c226f757470757473223a5b5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a226e6f6e70617961626c65222c2274797065223a2266756e6374696f6e227d2c7b22636f6e7374616e74223a747275652c22696e70757473223a5b5d2c226e616d65223a22676574222c226f757470757473223a5b7b226e616d65223a22222c2274797065223a2275696e74323536227d5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a2276696577222c2274797065223a2266756e6374696f6e227d5d00000000000000000000000000000000000000000000000000"
}

```

***

### eth_getBalance

获取合约余额。

* Parameters

    1. `DATA`, 20 Bytes - address.
    2. `QUANTITY|TAG` - integer block number(Hex string), or the string "latest", "earliest"

* Returns

`DATA` - the balance from the given address.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0xea4f6bc98b456ef085da5c424db710489848cab5", "0x1F"],"id":1}'

// Result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0"
}
```

***

### eth_newFilter

Creates a filter object, based on filter options, to notify when the state changes (logs). To check if the state has changed, call eth_getFilterChanges.

* Parameters

    1. `Object` - The filter object, see [eth_getLogs](#eth_getLogs)

* Returns

`QUANTITY` - A filter id.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_newFilter","params":[{"topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"]}],"id":1}'

// Result
{
  "id":1,
  "jsonrpc": "2.0",
  "result": "0x1"
}
```

***

### eth_newBlockFilter

Creates a filter in the node, to notify when a new block arrives. To check if the state has changed, call eth_getFilterChanges.

* Parameters

None

* Returns

`QUANTITY` - A filter id.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_newBlockFilter","params":[],"id":73}'

// Result
{
  "id":1,
  "jsonrpc":  "2.0",
  "result": "0x1"
}
```

***

### eth_uninstallFilter

Uninstalls a filter with given id. Should always be called when watch is no longer needed.
Additonally Filters timeout when they aren't requested with eth_getFilterChanges for a period of time.

* Parameters

    1. `QUANTITY` - The filter id.

* Returns

`Boolean` - true if the filter was successfully uninstalled, otherwise false.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_uninstallFilter","params":["0xb"],"id":73}'

// Result
{
  "id":1,
  "jsonrpc": "2.0",
  "result": true
}
```

***

### eth_getFilterChanges

Polling method for a filter, which returns an array of logs which occurred since last poll.

* Parameters

    1. `QUANTITY` - The filter id.

* Returns

`Array` - Array of log objects, or an empty array if nothing has changed since last poll.

* Example

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getFilterChanges","params":["0x16"],"id":74}'

// Result
{
    "jsonrpc":"2.0",
    "id":74,
    "result":[
        {
            "address":"0xea4f6bc98b456ef085da5c424db710489848cab5",
            "topics":[
                "0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"
            ],
            "data":"0x0000000000000000000000005b073e9233944b5e729e46d618f0d8edf3d9c34a0000000000000000000000000000000000000000000000000000000000000064",
            "blockHash":"0x3e83b74560860344f4c48d7b8089a18173aecd96b6b2148653c61b5d3f559764",
            "blockNumber":"0x4",
            "transactionHash":"0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af",
            "transactionIndex":"0x0",
            "logIndex":"0x0",
            "transactionLogIndex":"0x0"
        }
    ]
}
```

***

### eth_getFilterLogs

Returns an array of all logs matching filter with given id.

* Parameters

    1. `QUANTITY` - The filter id.

* Returns

`Array` - Array of log objects, or an empty array if nothing has changed since last poll.

* Example

see [eth_getFilterChanges](#eth_getFilterChanges)

***

## 在Account中保存ABI和获取ABI

默认将接收方地址为`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`的交易为保存合约ABI交易类型。

### 保存ABI

保存合约ABI到Account，主要步骤为：
1.需要先成功创建合约，得到合约地址；
2.使用solc编译合约代码得到ABI，以String类型编码成相应参数，详情见[Ethereum Contract ABI](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI)；
3.构造交易data，前20字节为合约地址，后面字节为abi的编码成的bytes；
4.发送交易，使用`cita_sendTransaction`接口；

* Example

以`scripts/contracts/tests/contracts/test_example.sol`这个合约为例子，正常在链上创建该合约；

- 首先可以通过solc得到合约的ABI；

```shell
solc --abi test_example.sol

// Result
[{"constant":false,"inputs":[{"name":"x","type":"uint256"}],"name":"set","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"get","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"}]
```

- 将abi作为String类型，编码结果如下：

```
0x4ed3885e0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000
00000000000000001275b7b22636f6e7374616e74223a66616c73652c22696e70757473223a5b7b226e616d65223a2278222c2274797065223a2275
696e74323536227d5d2c226e616d65223a22736574222c226f757470757473223a5b5d2c2270617961626c65223a66616c73652c2273746174654d7
5746162696c697479223a226e6f6e70617961626c65222c2274797065223a2266756e6374696f6e227d2c7b22636f6e7374616e74223a747275652c
22696e70757473223a5b5d2c226e616d65223a22676574222c226f757470757473223a5b7b226e616d65223a22222c2274797065223a2275696e743
23536227d5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a2276696577222c2274797065223a2266756e63
74696f6e227d5d00000000000000000000000000000000000000000000000000
```

- 构造并发送交易到链上

```shell
// 前面20字节`73552bc4e960a1d53013b40074569ea05b950b4d`为合约地址，后面为abi
python make_tx.py --code "73552bc4e960a1d53013b40074569ea05b950b4d4ed3885e000000000000000000000000000000000000000000000
000000000000000002000000000000000000000000000000000000000000000000000000000000001275b7b22636f6e7374616e74223a66616c7365
2c22696e70757473223a5b7b226e616d65223a2278222c2274797065223a2275696e74323536227d5d2c226e616d65223a22736574222c226f75747
0757473223a5b5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a226e6f6e70617961626c65222c22747970
65223a2266756e6374696f6e227d2c7b22636f6e7374616e74223a747275652c22696e70757473223a5b5d2c226e616d65223a22676574222c226f7
57470757473223a5b7b226e616d65223a22222c2274797065223a2275696e74323536227d5d2c2270617961626c65223a66616c73652c2273746174
654d75746162696c697479223a2276696577222c2274797065223a2266756e6374696f6e227d5d00000000000000000000000000000000000000000
000000000" --to "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"  --privkey "352416e1c910e413768c51390dfd791b414212b7b4fe6b1a
18f58007fa894214"

python send_tx.py

// Result
--> {"params": ["0ad0030a286161616161616161616161616161616161616161616161616161616161616161616161616161616112013118fface20420ba022a980373552bc4e960a1d53013b40074569ea05b950b4d4ed3885e000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001275b7b22636f6e7374616e74223a66616c73652c22696e70757473223a5b7b226e616d65223a2278222c2274797065223a2275696e74323536227d5d2c226e616d65223a22736574222c226f757470757473223a5b5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a226e6f6e70617961626c65222c2274797065223a2266756e6374696f6e227d2c7b22636f6e7374616e74223a747275652c22696e70757473223a5b5d2c226e616d65223a22676574222c226f757470757473223a5b7b226e616d65223a22222c2274797065223a2275696e74323536227d5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a2276696577222c2274797065223a2266756e6374696f6e227d5d000000000000000000000000000000000000000000000000001241a79c6b34aaa552ccbe99c1240ecf892a982fa337d26143f5485cb578f3972c426aaebd031d49b5ce32164422fd292957e183fa5933da8cb9eee426607e3314f101"], "jsonrpc": "2.0", "method": "cita_sendTransaction", "id": 1}
<-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0x5ea7cf509ef5325bd58f737ab533c7fe5683c285a2d7b71dde90519f5640ae73","status":"OK"}} (200 OK)
transaction hash 保存到../output/transaction/hash

```

### 查询abi

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getAbi","params":["73552bc4e960a1d53013b40074569ea05b950b4d", "latest"],"id":1}' 127.0.0.1:1337 | jq

// Result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x4ed3885e000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001275b7b22636f6e7374616e74223a66616c73652c22696e70757473223a5b7b226e616d65223a2278222c2274797065223a2275696e74323536227d5d2c226e616d65223a22736574222c226f757470757473223a5b5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a226e6f6e70617961626c65222c2274797065223a2266756e6374696f6e227d2c7b22636f6e7374616e74223a747275652c22696e70757473223a5b5d2c226e616d65223a22676574222c226f757470757473223a5b7b226e616d65223a22222c2274797065223a2275696e74323536227d5d2c2270617961626c65223a66616c73652c2273746174654d75746162696c697479223a2276696577222c2274797065223a2266756e6374696f6e227d5d00000000000000000000000000000000000000000000000000"
}
```
***

### cita_getTransactionProof

根据交易hash获取交易执行的证明。

* Parameters

    1. `DATA`, 32 Bytes - hash of a transaction

    ```js
    params: [
        "0x37f1261203d7b81a5a5cfc4a5c4abf15297555a47fd8686580d5a211876516c4"
    ]
    ```

* Returns

`DATA` - A proof include transaction, receipt, receipt merkle tree proof, block header. There will be a tool to verify the proof and extract some info.

* Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getTransactionProof","params":["0x37f1261203d7b81a5a5cfc4a5c4abf15297555a47fd8686580d5a211876516c4"],"id":1}'

// Result
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "0xf9070df903f2a09a9102d68052de36b825d5b1f4dc9512684a8b1a61add6337e455ef69eb304e7a044d0da81e6ebd0928c084692e5aacb6f0e56dbd0a064c9639adb7ef9f6fd3b41a037f1261203d7b81a5a5cfc4a5c4abf15297555a47fd8686580d5a211876516c4a02c533219294c9fce960e0749b017a3ce281eb45facaa138123240336fef8327db90100000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000400000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000005b88ffffffffffffffff826d7b86016242114b6f80b902530ace0442000000000000003078306139383765623937646438323539626433623063386665303832303333333762336631386536613161653735613035636261383032336135346563303564355a00000000000000000000000000000004000000000000002a000000000000003078316666333830313730633765633338613831643731636663636433343765653033386464646237304100000000000000cf81312f4118a72b53f35ecd99817cb499597f1ff7132df672e58cbb615180e8535eebe2040d6303b50c55cbe01d01de49512b6dbffc363dd950a4a919b9693b002a000000000000003078383665373133326164326535323433326466323537633630373638373962626334393962343365624100000000000000f75f5b766676560d6e1a116c29ac6176070c5448c453dc68202b4ac7d388c3cc0edf07d2e0c171710c6ec30735d06590b7fa8e3c094a9275df1bfddf026f1a94002a000000000000003078373661393566333633313532666133353338623530366362636539333764343138613164643131644100000000000000cba1c56842458c11245ba64b0fc3f93dc90682b12407623634433a6a22a741af356a2df0603ab6d300b7c8638db7c56761a716d4948da10be177b36193c058e7012a0000000000000030783362313836653138643263353530383661343230323338373239613238303761393364366163633941000000000000000727955db4413d3c7f476d94d7570ae6d6ed4177fff682fde08ee21b1e46e47073d39d22218b55bb56e1e620fb4a5f17c998d9b80892a27fd73d947756483085001002f901e5826d7bb9010000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000040000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000f8dbf8d99473552bc4e960a1d53013b40074569ea05b950b4de1a002d96cafa1b66d774136b3051a7a5675d5cb23a055bfe1410019ec924f4f93acb8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000073552bc4e960a1d53013b40074569ea05b950b4d00000000000000000000000000000000000000000000000000000000ec90a79a0000000000000000000000000000000000000000000000000000000000000000c001c0f9012c31808398967f9473552bc4e960a1d53013b40074569ea05b950b4d80b8644c6e5926000000000000000000000000000000000000000000000000000000000000000000000000000000000000000073552bc4e960a1d53013b40074569ea05b950b4d000000000000000000000000000000000000000000000000000000000000000281aeb84182e91042d0201d5f3c8def6f7b09a46a02c6d4a9890e878406498f47fc78719a6d86ee0cfb7bba33228649790c7340ea975dbe59b7fd6a0f267de972b1008dbf0180a037f1261203d7b81a5a5cfc4a5c4abf15297555a47fd8686580d5a211876516c4b840c7561cde2792e85c76bf423cf9c339bd085f6ca686e2fe5cb5092c6bff210786790cac0bf6472e6837282bef5e19416e791f5dbf6abbb33e04e4b291d6ef4e1b80"
}
```

***

### cita_getMetaData

根据高度查询链上元数据。

* Parameters

    1. `QUANTITY`, integer of a block height or "latest"

    ```py
    params: [
        "0x1da3"
    ]
    ```

* Returns
    * `chainId`, u32 - Deal with transaction replay attack
    * `chainName`, String - Chain Name
    * `operator`, String - Chain operator
    * `genesisTimestamp`, Timestamp - Genesis timestamp
    * `validators`, [Address] - Validator array
    * `blockInterval` u64 - block interval by millisecond
* Example

```shell
$ curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getMetaData","params":["0xff"],"id":1}' 127.0.0.1:1337

{"jsonrpc":"2.0","id":1,"result":{"chainId":464896313,"chainName":"test-chain","operator":"test-operator","website":"https://www.example.com","genesisTimestamp":1525313122,"validators":["0x1c82af8ac82348748c14970296386cf466271bed","0xd3e9c874eb92663337db36f570bca4c0167312be","0x56075c47c33ebac18a8f8be0c3cbaf66328027ac","0xbd5e28047ac06f31fe27cc3ab6ce014cfa05ced3"],"blockInterval":3000}}

```

***

## RPC Errors

### Invalid Request

```shell
// 正确方式：POST 方法
curl -X GET -d '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[]}' 127.0.0.1:1337
```

```shell
{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid Request"},"id":null}
```

### Method not found

```shell
// 正确方式： method应该是cita_getBlockByNumber
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_blockByHeight","params":[true],"id":3}' 127.0.0.1:1337
```

```shell
{"jsonrpc":"2.0", "error":{"code":-32601,"message":"Method not found"},"id":3 }
```

### Invalid params

```shell
// 正确方式：params 应该是 ["0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236"]
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getTransaction","params":[0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236],"id":2}' 127.0.0.1:1337
```

```shell
{"jsonrpc":"2.0", "error":{"code":-32602,"message":"Invalid params"},"id":2}
```

### Null

```shell
// 原因：块高度未达到99999
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0x1869F",true],"id":2}' 127.0.0.1:1337
```

```shell
{"jsonrpc":"2.0","id":2,"result":null}
```
