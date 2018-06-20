# JSON RPC 错误码

## JSON RPC标准错误码

 | 错误码              | 错误消息        | 描述              |
 | ----------------  | :------------ | :---------------|
 | -32700             | 解析错误        | 非Json格式数据     |
 | -32600             | 请求错误        | 含有错误的请求值    |
 | -32601             | 请求服务方法错误 | 调用方法不存在或错误 |
 | -32602             | 非法参数        | 调用方法参数错误    |
 | -32603             | 内部错误        | 内部错误(NotReady)           |
 | -32003             | 查询类错误      | 见示例             |
 | -32006             | 交易认证类错误   | 见示例(InvalidNonce,Dup,InvalidUntilBlock,BadSig,Buy)             |
 | -32099             | 请求超时        | 见示例(system time out,please resend)             |

## 错误示例

### 交易错误

``` json
//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "InvalidUntilBlock"//交易valid_until_block过时.
  }
}


//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "Dup"//重复交易
  }
}

//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "InvalidNonce"//非法nonce
  }
}

//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "Busy"//处理交易繁忙
  }
}

//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32006,
    "message": "BadSig"//签名错误
  }
}

//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32603,
    "message": "NotReady"//内部错误
  }
}

```

### 请求超时

``` json
//request 发送交易
curl -X POST --data '{"jsonrpc":"2.0","method":"sendRawTransaction","params":["..."],"id":1}' 127.0.0.1:1337 | jq
   //result
   {
     "jsonrpc": "2.0",
     "id": 1,
     "error": {
       "code": -32099,
       "message": "system time out,please resend"
     }
   }
```

### 请求错误

``` json
//request 应发送POST请求，而不是GET请求
curl -X GET -d '{"jsonrpc":"2.0","method":"blockNumber","params":[],"id":"1"}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": "1",
  "error": {
    "code": -32600,
    "message": "Invalid request"
  }
}
```

### 调用方法错误

``` json
//request
curl -X POST --data '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":74}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": "74",
  "error": {
    "code": -32601,
    "message": "Method not found"
  }
}
```

### 非法参数

``` json
//request 参数不能是十进制整数,需要是十六进制
curl -X POST --data '{"jsonrpc":"2.0","method":"getBlockByNumber","params":[249, true],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params: invalid type: integer `249`, expected a hex block number or 'latest', 'earliest'."
  }
}


//request 参数个数不正确
curl -X POST -d '{"jsonrpc":"2.0","method":"getTransaction","params":["0x0063187e6a84ae731cf9",true],"id":2}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32602,
    "message": "invalid JsonRpc params length"
  }
}

```

### 其他错误信息

``` json
//request 交易个数为０或者地址不正确
curl -X POST --data '{"jsonrpc":"2.0","method":"getTransactionCount","params":["5b073e9233944b5e729e46d618f0d8edf3d9c342",2],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x0"
}

//request 没有部署合约
curl -X POST --data '{"jsonrpc":"2.0","method":"getCode","params":["ea4f6bc98b456ef085da5c424db710489848cab9",35],"id":1}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": null
}

//request 高度未达到
curl -X POST -d '{"jsonrpc":"2.0","method":"getBlockByNumber","params":[99999,true],"id":2}' 127.0.0.1:1337 | jq

//result
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": null
}
```

参考文档:

1. [JSON RPC specification](http://www.jsonrpc.org/specification)

2. [Ethereum wiki/JSON RPC Error Codes Improvement Proposal](https://github.com/ethereum/wiki/wiki/JSON-RPC-Error-Codes-Improvement-Proposal)
