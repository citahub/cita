# 自动执行

`CITA` 自动执行合约。

## 简述

自动执行是底层的一个扩展功能，只有管理员可以注册，注册的自动执行合约需要继承 `IAutoExec.sol`，实现 autoExec 函数作为自动执行的入口。

### 合约信息

合约地址为: `0xffffffffffffffffffffffffffffffffff020013`

接口签名如下:

```
======= system/AutoExec.sol:AutoExec =======
Function signatures:
844cbc43: autoExec()
f95b72de: contAddr()
4420e486: register(address)
```

## 操作示例

*首先需要启动一条链，具体方法见快速入门部分*

其中[测试合约](https://github.com/cryptape/cita/blob/develop/scripts/contracts/tests/contracts/AutoExec.sol)函数签名如下:

```
======= contracts/AutoExec.sol:AutoExec =======
Function signatures:
844cbc43: autoExec()
0c55699c: x()
```

其中：

* `autoExec()`： 为自动执行的入口，实现对 x 加一
* `x()`： 表示获取x数值

接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式（与交互式模式的命令是一致的）进行演示。


### 部署测试合约

*使用默认私钥进行演示*

* 发送交易

```bash
$ cita-cli rpc sendRawTransaction \
    --code 0x608060405234801561001057600080fd5b5060cf8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680630c55699c14604e578063844cbc43146076575b600080fd5b348015605957600080fd5b506060608a565b6040518082815260200191505060405180910390f35b348015608157600080fd5b5060886090565b005b60005481565b60008081548092919060010191905055505600a165627a7a723058204a422f811fa5ff28e0e79adab0817a6ffe6d283011d9107b68a36bc95091abd30029 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 4,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x4c590891fdfd89d5e064c467ac74196ef4fcba5e80dc670da01800c652650913",
    "status": "OK"
}
```

* 获取交易回执

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x4c590891fdfd89d5e064c467ac74196ef4fcba5e80dc670da01800c652650913 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xae69b6b37e0eae54960a40bcce75f8495f97a1ffb5ec0397a542f8b85469e336",
    "blockNumber": "0x28e",
    "contractAddress": "0xd48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b",
    "cumulativeQuotaUsed": "0x1711d",
    "errorMessage": null,
    "logs": [
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "quotaUsed": "0x1711d",
    "root": null,
    "transactionHash": "0x4c590891fdfd89d5e064c467ac74196ef4fcba5e80dc670da01800c652650913",
    "transactionIndex": "0x0"
  }
}
```

其中合约地址为： `0xd48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b`

### 查询 x 数值

```bash
$ cita-cli rpc call \
    --to 0xd48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b \
    --data 0x0c55699c \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
```

数值为 0

### 注册自动执行合约

调用 `register` 接口对测试合约进行注册。预期结果为 x 数值随块的增加而增加。

* 发送交易

```bash
$ cita-cli rpc senRawTransaction \
    --code 0x4420e486000000000000000000000000d48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --address 0xffffffffffffffffffffffffffffffffff020013
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 4,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0xd9bd3a0dbc406d8772dcb85785863897f4102faac1fdcdc4247a735ac3d1a4a5",
    "status": "OK"
  }
}
```

* 获取交易回执

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0xd9bd3a0dbc406d8772dcb85785863897f4102faac1fdcdc4247a735ac3d1a4a5 \
    --url http://127.0.0.1:1337
```

输出：

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x107f386d21b5af1c7f4cab4802f63ac7e5989b623161cae2ba533c4ed32061bd",
    "blockNumber": "0x3a7",
    "contractAddress": null,
    "cumulativeQuotaUsed": "0x6e4a",
    "errorMessage": null,
    "logs": [
      {
        "address": "0xffffffffffffffffffffffffffffffffff020013",
        "blockHash": "0x107f386d21b5af1c7f4cab4802f63ac7e5989b623161cae2ba533c4ed32061bd",
        "blockNumber": "0x3a7",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0x2d3734a8e47ac8316e500ac231c90a6e1848ca2285f40d07eaa52005e4b3a0e9",
          "0x000000000000000000000000d48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b"
        ],
        "transactionHash": "0xd9bd3a0dbc406d8772dcb85785863897f4102faac1fdcdc4247a735ac3d1a4a5",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      }
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000080000004000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000100000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000200000",
    "quotaUsed": "0x6e4a",
    "root": null,
    "transactionHash": "0xd9bd3a0dbc406d8772dcb85785863897f4102faac1fdcdc4247a735ac3d1a4a5",
    "transactionIndex": "0x0"
  }
}
```

这里从 `logs` 已经可以看出测试合约已经注册成功。


### 验证结果

查询 x 数值

```bash
$ cita-cli rpc call \
    --to 0xd48cc17fdfa7e0af76637c5a9e658bcc9e0e9b8b \
    --data 0x0c55699c \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000026"
}
```

数值变为 26

自动执行生效。
