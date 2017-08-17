RPC 调用
===============

**RPC模块服务命令详细介绍**

RPC是一个单独的可执行模块，可以通过以下命令选项，优化服务资源。

 - --help

   查看帮助文档信息。

 - --jsonrpc-port

   服务运行时监听的端口号，默认为本机的1337端口。

 - --thread-num

   服务运行开启的线程数量，默认开启200个线程。

 - --sleep-duration

   循环等待应答时每一次轮询的等待时间，默认为3000(ns)。

 - --prof-start

   用于性能分析，服务运行延时设定的的时间后，开始生成性能分析文件。默认是0，即不开启。

 - --prof-duration

   性能分析开启后的结束时间，默认是0，即不开启。需要同时开启--prof-start选项，并且需要借助性能分析工具google-perftools，快速帮助完成性能分析报告。

**JSON-RPC 接口**

目前，RPC模块提供以下接口，具体详细内容，可见下文介绍。

 - net_peerCount　
 - cita_blockNumber　
 - cita_sendTransaction　
 - cita_getBlockByHash　
 - cita_getBlockByNumber　
 - cita_getTransaction　
 - eth_getTransactionCount　
 - eth_getCode　
 - eth_getTransactionReceipt　
 - eth_call　

值得注意的是，以eth开头的RPC接口是为了兼容以太坊而设计的，具体接口详情可以参考 `以太坊 <https://github.com/ethereum/wiki/wiki/JSON-RPC>`__ 接口说明。


**JSON-RPC 接口详细介绍**

**net_peerCount**
当前节点连接数

params
 - 无

return
 - count: 节点连接数量

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74}' 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 74,
      "result": "0x3"
    }

**cita_blockNumber**
返回当前块高度

params
 - 无

return
 - height: 当前高度值

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":83}' 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 83,
      "result": "0x8"
    }

**cita_sendTransaction**
发送交易

params
 - data: 签名的交易

return
 - state: 交易的状态
 - hash: 交易的哈希值

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010"],"id":1}' 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": {
        "hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
        "status": "4:OK"
      }
    }

**cita_getTransaction**
获取交易信息

params
 - hash: 交易哈希值

return
 - hash: 交易哈希值
 - content: 交易内容　
 - blockNumber: 块高度　
 - blockHash: 块哈希　
 - index: 交易在块中的索引　

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getTransaction","params":["f31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b"],"id":1}' 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": {
        "hash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
        "content": "0x0a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a80040aba030a28356230373365393233333934346235653732396534366436313866306438656466336439633334611a87030a013010a08d0622fd026060604052341561000c57fe5b5b7f4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc6633604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b60fd806100806000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a04651900292080808080101241d51ca7a0171113478f47357a71c240bd0431f52639741a6721725de276a88d2e723b12f4bbeb1cdddea63f947ddb9db6e2667f08a03af1577c42d3c1a3dc5a7c01208080808010",
        "blockNumber": "0x5b",
        "blockHash": "0xc68eb999432bcc0712d1b4a1d03c6eb10a27ea0fe34e8f60cb3e02d8ccbcda8d",
        "index": "0x0"
      }
    }

**cita_getBlockByHash**
根据块hash查询块的信息

params
 - hash: 32 bytes, 块的哈希值
 - boolean: 是否返回交易信息(true: 返回详细交易列表，false: 只返回交易hash)

return
 - object: 块中的具体信息

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByHash","params":["2f3853b7d3bb3d6bc4bb2103b645aae2b8145125340018209184c7709e04dbc3", true],"id":1}' 127.0.0.1:1337 | jq

    // Response
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
          "commit": {
            "stateRoot": "0xe29266e5574bc0c848b513d36403d4da71f99f328d3324e8d3134809c33d4fb4",
            "transactionsRoot": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
            "receiptsRoot": "0x9646cf2572734b4b13fe1616446ab2658e208cfdbaf25e47ebea9b6327e10c5b",
            "gasUsed": "0x0"
          },
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

**cita_getBlockByNumber**
根据块高度查询块信息

params
 - quantity: 块高度
 - boolean: 是否返回交易信息(true: 返回详细交易列表 false: 只返回交易hash)

return
 - object: 块中的具体信息

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["0xF9", true],"id":1}' 127.0.0.1:1337 | jq
    或者
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["249", true],"id":1}' 127.0.0.1:1337 | jq
    或者
    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":[249, true],"id":1}' 127.0.0.1:1337 | jq

**eth_getTransactionReceipt**
获取交易凭证

params
 - hash: 交易哈希值

return
 - object: 凭证信息

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":["f31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b"],"id":1}' 127.0.0.1:1337 | jq

    // reuslt
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": {
        "transactionHash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
        "transactionIndex": "0x0",
        "blockHash": "0xc68eb999432bcc0712d1b4a1d03c6eb10a27ea0fe34e8f60cb3e02d8ccbcda8d",
        "blockNumber": "0x5b",
        "cumulativeGasUsed": "0xca9d",
        "gasUsed": "0xca9d",
        "contractAddress": "0xea4f6bc98b456ef085da5c424db710489848cab5",
        "logs": [
          {
            "address": "0xea4f6bc98b456ef085da5c424db710489848cab5",
            "topics": [
              "0x4f8cfde3439a1a302c21ca51eec26086efbfd940b8c0279889fc6bb6e73ecc66"
            ],
            "data": "0x0000000000000000000000005b073e9233944b5e729e46d618f0d8edf3d9c34a",
            "blockHash": "0xc68eb999432bcc0712d1b4a1d03c6eb10a27ea0fe34e8f60cb3e02d8ccbcda8d",
            "blockNumber": "0x5b",
            "transactionHash": "0xf31e32611322f410f430ef8141c2237c19dd1034eddef8dedba692ec9851799b",
            "transactionIndex": "0x0",
            "logIndex": "0x0",
            "transactionLogIndex": "0x0"
          }
        ],
        "root": "0xe29266e5574bc0c848b513d36403d4da71f99f328d3324e8d3134809c33d4fb4",
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000010000000000001000000000000000000000000000000000000000000000010000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      }
    }

**eth_getTransactionCount**
获取交易数

param
 - address: 账户地址
 - height: quantity|tag, (可选, 默认: "latest") 高度值或者 "latest" 或者 "earliest"

return
 - count: 交易个数

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params":["5b073e9233944b5e729e46d618f0d8edf3d9c34a","0x5b"],"id":1}' 127.0.0.1:1337 | jq

    // reslut
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": "0x1"
    }

**eth_getCode**
查看合约信息

params
 - address: 合约的地址

return
 - object: 合约信息

::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab5","0x5b"],"id":1}' 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": "0x60606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460445780636d4ce63c146061575bfe5b3415604b57fe5b605f60048080359060200190919050506084565b005b3415606857fe5b606e60c6565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055505b50565b600060005490505b905600a165627a7a7230582079ba3769927f0f8cf4bec7ce02513b56823c8fc3f4047989951e042a9a0465190029"
    }

**eth_getLogs**
根据Topic查询logs。

params
 - object

  - fromBlock: quantity|tag, (可选, 默认: "latest") 高度值或者 "latest" 或者 "earliest"
  - toBlock: quantity|tag, (可选, 默认: "latest") 高度值或者 "latest" 或者 "earliest"
  - address: data|array, 20 bytes,（可选）合约地址或者合约地址列表
  - topics: array of data, (可选) 过滤条件，关于topics构造，可以参考 `以太坊 <https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_newfilter>`__

return
 - object: 日志信息

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getLogs","params":[{"topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"],"fromBlock": 0}],"id":74}' 127.0.0.1:1337 | jq

    // Response
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

**eth_call**
合约接口调用

params
 - Object: 合约调用对象

  - from: DATA, 20 Bytes 交易发送方
  - to:   DATA, 20 Bytes 交易接收方
  - data: DATA, (可选)　经过签名的数据
  - quantity: 块高度

return
 - hash: 交易哈希值

example:
::

    // Request
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call","params":[{"from":"0xca35b7d915458ef540ade6068dfe2f44e8fa733c","to":"0xea4f6bc98b456ef085da5c424db710489848cab5","data":"0x6d4ce63c"}, 6],"id":2} 127.0.0.1:1337 | jq

    // Response
    {
      "jsonrpc": "2.0",
      "id": 2,
      "result": "0x0000000000000000000000000000000000000000000000000000000000000064"
    }



**RPC错误返回码和错误介绍**

=========================  ========================
     错误码                  错误消息
=========================  ========================
   -32700                   解析错误
   -32600                   请求错误
   -32601                   请求服务方法错误
   -32602                   非法参数
   -32603                   网络错误
   -32000 to -32099         自定义服务错误
=========================  ========================

**错误示例**
::

    // 应发送POST请求，而不是GET请求
    curl -X GET -d '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[]}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": null,
      "id": null,
      "error": {
        "code": -32600,
        "message": "Invalid request"
      }
    }

    // hash值没有前缀0x
    curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getTransaction","params":["0x0063187e6a84ae731cf9"],"id":2}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": null,
      "id": null,
      "error": {
        "code": -32602,
        "message": "param is not hash"
      }
    }

    // 调用方法错误
    curl -X POST --data '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":74}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": null,
      "id": null,
      "error": {
        "code": -32601,
        "message": "Method not found"
      }
    }

    // 交易个数为０或者地址不正确
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionCount","params":["5b073e9233944b5e729e46d618f0d8edf3d9c342",2],"id":1}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": "0x0"
    }

    // 没有部署合约
    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab9",35],"id":1}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": null
    }

    // 高度未达到
    curl -X POST -d '{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":[99999,true],"id":2}' 127.0.0.1:1337 | jq

    {
      "jsonrpc": "2.0",
      "id": 2,
      "result": null
    }

