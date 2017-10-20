#CITA说明

## JSON-RPC

* net_peerCount
* cita_blockNumber
* cita_sendTransaction
* cita_getBlockByHash
* cita_getBlockByNumber
* cita_getTransaction
* eth_getTransactionCount
* eth_getCode
* eth_getTransactionReceipt
* eth_call

***
#### net_peerCount

当前的节点连接数.

##### Parameters
none

##### Returns
QUANTITY - integer of the number of connected peers.

##### Example
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

#### cita_blockNumber

返回当前块高度.

##### Parameters
none

##### Returns

`QUANTITY` - integer of current block height of CITA.

##### Example
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
#### cita_sendTransaction

通过序列化交易调用区块链接口.

##### Parameters

1. `DATA`, The signed transaction data.
```js
const signed_data = "0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901"
params: [signed_data]
```

### 生成签名数据的过程
#### 构造protobuf数据结构
```js
// Transaction
syntax = "proto3";
enum Crypto {
    SECP = 0;
    SM2 = 1;
}

//nonce 标识交易的唯一性
message Transaction {
    string to = 1;
    string nonce = 2;
    uint64 quota = 3; // gas
    uint64 valid_until_block = 4;
    bytes data = 5;
}

message UnverifiedTransaction {
    Transaction transaction = 1;
    bytes signature = 2;
    Crypto crypto = 3;
}
```
#### 获得合约对应的bytecode

以下代码片段为示例代码，具体获取contract bytecode的方法参考[文档](https://ethereum.stackexchange.com/questions/8115/how-to-get-the-bytecode-of-a-transaction-using-the-solidity-browser)

[solidity](https://solidity.readthedocs.io/en/develop/)相关文档
```
pragma solidity ^0.4.15;

contract SimpleStorage {
    uint storedData;
    event Init(address, uint);
    event Set(address, uint);

    function SimpleStorage() {
        storedData = 100;
        Init(msg.sender, 100);
    }

    event Stored(uint);

    function set(uint x)  {
        Stored(x);
        storedData = x;
        Set(msg.sender, x);
    }

    function get() constant returns (uint) {
        return storedData;
    }
}

```
#### 构造签名

1. 构造Transaction对象tx，填充to, nonce, valid_until_block, data 4个字段。
2. tx对象protobuf序列化后 sha3 -> hash
3. 对 hash 进行签名 -> signature
4. 构造UnverifiedTransaction, 使用hash, signature, SECP填充UnverifiedTransaction  -> unverify_tx
5. unverify_tx对象protobuf序列化

伪代码描述:

```
let tx = Transaction::new();
// contract bytecode
let data = "6060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a723058207fbd8b51e2ecdeb2425f642d6602a4ff030351102fd7afbed80318e61fa462670029".from_hex();
tx.setdata(data);
if not depoly_contract {
    tx.setTo(address);
}
tx.set_valid_until_block(9999999);
tx.set_nonce(nonce);

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

```
0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901
```
##### Returns

`DATA`, 32 Bytes - 交易hash

##### Example
```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901"],"id":1}'

// Result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236",
    "status": "Ok"
  }
}

// 如果是近期发送的重复交易，则会提示重复交易

{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": 6,
    "message": "TxResponse {hash: 019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236, status: \"Dup\" }"
  }
}


```
***


#### cita_getBlockByHash

根据块hash查询块的信息

##### Parameters

DATA, 32 Bytes - Hash of a block.
Boolean - 是否返回交易信息(True: 返回详细交易列表| False: 只返回交易hash).
```
params: [
   '0x296474ecb4c2c8c92b0ba7800a01530b70a6f2b6e76e5c2ed2f89356429ef329',
   true
]
```
##### Returns

Object - A block object, or null when no block was found:

##### Example

```
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
      "gasUsed": "0x0"
      "height": "0x387"
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
#### cita_getBlockByNumber

根据块高度查询块信息.

##### Parameters

1. `QUANTITY` - integer of a block height.
2. `Boolean` - 是否返回交易信息(True: 返回详细交易列表| False: 只返回交易hash).

```js
params: [
   0x1da3,
   true
]
```

##### Returns

See [cita_getBlockByHash](#cita_getblockbyhash)

##### Example

```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0xF9", true],"id":1}'

```
##### Invalid Params

```
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0XF9", true],"id":1}'
或者
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["249", true],"id":1}'
或者
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":[249, true],"id":1}'
```
高度参数可以用0x开头的十六进制。0X开头或者十进制整数都是错误的参数格式。

同 [cita_getBlockByHash](#cita_getblockbyhash)

***

#### eth_getTransactionReceipt

根据交易hash获取交易回执。


##### Parameters

1. `DATA`, 32 Bytes - hash of a transaction

```js
params: [
   "b38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af"
]
```

##### Returns
Object - A receipt object:

* transactionHash: DATA, 32 Bytes - hash of the transaction.
* transactionIndex: QUANTITY - transaction index.
* blockHash: DATA, 32 Bytes - hash of the block where this transaction was in. null when its not in block.
* blockNumber: QUANTITY - block number where this transaction was in. null when its not in block.
* cumulativeGasUsed: QUANTITY - The total amount of gas used when this transaction was executed in the block.
* gasUsed: QUANTITY - The amount of gas used by this specific transaction alone.
* contractAddress: DATA, 20 Bytes - The contract address created, if the transaction was a contract creation, otherwise null.
* logs: Array - Array of log objects, which this transaction generated.
* root : DATA 32 bytes of post-transaction stateroot 
* errorMessage: String, execution error message.

##### receipt error messages
* No transaction permission.
* No contract permission.
* Not enough base gas.
* Block gas limit reached.
* Account gas limit reached.
* Out of gas
* Jump position wasn't marked with JUMPDEST instruction.
* Instruction is not supported.
* Not enough stack elements to execute instruction.
* Execution would exceed defined Stack Limit.
* EVM internal error.

##### Example
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
如果出现**Timeout，errorcode 99**,请查看可能的解决方法[Cann't assign requested Address](https://vincent.bernat.im/en/blog/2014-tcp-time-wait-state-linux)

***

#### eth_getLogs

根据Topic查询logs。

##### A note on specifying topic filters:
Topics are order-dependent. A transaction with a log with topics [A, B] will be matched by the following topic filters:
* `[]` "anything"
* `[A]` "A in first position (and anything after)"
* `[null, B]` "anything in first position AND B in second position (and anything after)"
* `[A, B]` "A in first position AND B in second position (and anything after)"
* `[[A, B], [A, B]]` "(A OR B) in first position AND (A OR B) in second position (and anything after)"

##### Parameters

1. `Object` - The filter options:
  - `fromBlock`: `QUANTITY|TAG` - (optional, default: `"latest"`) Integer block number, or `"latest"` or `"earliest"`.
  - `toBlock`: `QUANTITY|TAG` - (optional, default: `"latest"`) Integer block number, or `"latest"` or `"earliest"`.
  - `address`: `DATA|Array`, 20 Bytes - (optional) Contract address or a list of addresses from which logs should originate.
  - `topics`: `Array of DATA`,  - (optional) Array of 32 Bytes `DATA` topics. Topics are order-dependent. Each topic can also be an array of DATA with "or" options.

##### Example
```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getLogs","params":[{"topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"],"fromBlock": 0}],"id":74}'

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
#### eth_call

合约接口调用.

##### Parameters

1. Object - The transaction call object
 * from: DATA, 20 Bytes - (optional) The address the transaction is sent from.
 * to:   DATA, 20 Bytes - The address the transaction is directed to.
 * data: DATA - (optional) Hash of the method signature and encoded parameters. For details see [Ethereum Contract ABI](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI)
2. QUANTITY - integer block height

```js
params: [{"from":"0xca35b7d915458ef540ade6068dfe2f44e8fa733c","to":"0xea4f6bc98b456ef085da5c424db710489848cab5","data":"0x6d4ce63c"}, 0x1d23]
```

##### Returns

`DATA`, 32 Bytes - the transaction hash.

(Parameters -> 1 -> Object -> data)  example contract中get方法Hash和编码后的数据
```
0x6d4ce63c
```

发送和获取数据
```js
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call",
"params":[{"from":"0xca35b7d915458ef540ade6068dfe2f44e8fa733c","to":"0xea4f6bc98b456ef085da5c424db710489848cab5",
"data":"0x6d4ce63c"}, 6],"id":2}'

// Result
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": "0x0000000000000000000000000000000000000000000000000000000000000064"
}

```
***

#### cita_getTransaction

根据交易hash查询交易。


##### Parameters

1. `DATA`, 32 Bytes - hash of a transaction

```js
params: [
   "0x019abfa50cbb6df5b6dc41eabba47db4e7eb1787a96fd5836820d581287e0236"
]
```

##### Returns
Object - A transaction object, or null when no transaction was found:

* hash: DATA, 32 Bytes - hash of the transaction.
* content: DATA, 交易内容.
* blockHash: DATA, 32 Bytes - hash of the block where this transaction was in. null when its not in block.
* blockNumber: QUANTITY - block number where this transaction was in. null when its not in block.
* index: QUANTITY - integer of the transactions index position in the block. null when its not in block.
##### Example

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

#### eth_getTransactionCount
It is tranasction that you must have a EOA to deploy contract. eg:

keypair
```rust
let privkey = crypto::PrivKey::from(H256::from_str("966fc50326cf6e2b30b06d8214737fcc2cda5bdce84eb23e14b6dbf3540d3f84").unwrap());
let keypair = crypto::KeyPair::from_privkey(privkey.into()).unwrap();
// nonce = 0
// sender: "4bfff5a38b972bda5c54a40a7a6427514409d149"
// EOA address = "4bfff5a38b972bda5c54a40a7a6427514409d149"

```
example contract
```solidity
pragma solidity ^0.4.0;
contract SimpleStorage {
    uint storedData;

    function SimpleStorage() {
        storedData = 100;
    }

    event Stored(uint);

    function set(uint x)  {
        Stored(x);
        storedData = x;
    }

    function get() constant returns (uint) {
        return storedData;
    }
}
```
complie contract:
```
bincode = "0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010"

```
deploy contract:
```
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010"],"id":1}' 127.0.0.1:1337 | jq

```
get some infomation like eg:
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
    "status": "4:OK"
  }
}
```
ok! next:
```
curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getTransaction","params":["f31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b"],"id":1}' 127.0.0.1:1337 | jq

result:
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": {
        "transaction": {
          "hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
          "content": "0x0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010"
        },
        "block_height": 35,
        "block_hash": "0x8c4b8d43a973770e7362d3d4f72c541daa9e23cdf4690815e11b6e6cbc3e4f5b",
        "index": 0
      }
    }
```

get the transaction block height. eg:
```
"block_height": 35,
```
Now! you can test the eth_getTransactionCount interface. eg:
```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params":["5b073e9233944b5e729e46d618f0d8edf3d9c34a",35],"id":1}' 127.0.0.1:1337 | jq

reslut:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x1"
}

```
good!

keep on! test eth_getCode.
you have a contract address that is generated from eth_getTransactionReceipt result.
eg:
```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":["f31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b"],"id":1}' 127.0.0.1:1337 | jq

reuslt:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "logs": [
      {
        "transaction_hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
        "transaction_index": 0,
        "block_hash": "0x8c4b8d43a973770e7362d3d4f72c541daa9e23cdf4690815e11b6e6cbc3e4f5b",
        "block_height": 35,
        "address": "ea4f6bc98b456ef085da5c424db710489848cab5",
        "topics": [
          "0x4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc66"
        ],
        "data": "0x0000000000000000000000005b073e9233944b5e729e46d618f0d8edf3d9c34a"
      }
    ],
    "transaction_hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
    "transaction_index": 0,
    "block_hash": "0x8c4b8d43a973770e7362d3d4f72c541daa9e23cdf4690815e11b6e6cbc3e4f5b",
    "block_height": 35,
    "gas_used": 51869,
    "cumulative_gas_used": 51869,
    "contract_address": "0xea4f6bc98b456ef085da5c424db710489848cab5"
  }
}
```
right! get contract_address

keep

call eth_getCode:
```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab5",35],"id":1}' 127.0.0.1:1337 | jq

result:
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x60606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a0465190029"
}
```
end!


***

# RPC Errors 和 Null

1 Invalid Request

```
// 正确方式：POST 方法
curl -X GET -d '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[]}' 127.0.0.1:1337
```

```
{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid Request"},"id":null}
```

2 Method not found

```
// 正确方式： method应该是cita_getBlockByNumber
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_blockByHeight","params":[true],"id":3}' 127.0.0.1:1337
```

```
{"jsonrpc":"2.0", "error":{"code":-32601,"message":"Method not found"},"id":3 }
```

3 Invalid params

```
// 正确方式：params 应该是 ["0063187e6a84ae731cf9"]
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getTransaction","params":["0x0063187e6a84ae731cf9"],"id":2}' 127.0.0.1:1337
```

```
{"jsonrpc":"2.0", "error":{"code":-32602,"message":"Invalid params"},"id":2}
```

4 Method not found

```
// 正确方式： method应该是cita_getBlockByNumber
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getBlockBy","params":[99,true],"id":2}' 127.0.0.1:1337
```

```
{"jsonrpc":"2.0", "error":{"code":-32601,"message":"Method not found"},"id":2}
```

5 Null

```
// 原因：块高度未达到99999
curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":[99999,true],"id":2}' 127.0.0.1:1337
```

```
{"jsonrpc":"2.0","id":2,"result":null}
```


7 test cita_code

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab9",35],"id":1}' 127.0.0.1:1337 | jq

//address or height error
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": null
}
```

8  test cita_get_TransactionCount
```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params":["5b073e9233944b5e729e46d618f0d8edf3d9c342",2],"id":1}' 127.0.0.1:1337 | jq

//address or height error
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0"
}




```
