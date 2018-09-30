# 批量交易

`CITA` 支持批量交易，目前只能进行批量合约的调用。

## 简述

批量交易是由系统合约实现的，通过组装数据调用其接口来完成。

### 合约信息

合约地址为: `0xffffffffffffffffffffffffffffffffff02000e`

接口签名如下:

```
======= batch_tx.sol:BatchTx =======
Function signatures:
82cc3327: multiTxs(bytes)
```

### 数据组装规则

参数类型为 `bytes`，encode规则和ABI一致。拼装规则如下:

* 二十字节的目标调用合约的地址
* 四字节的目标合约的调用数据的长度
    - 四字节的函数签名
    - ABI格式编码的函数参数
* 目标合约的调用数据(第一条交易信息结束)
* ...(第n条交易信息)

拼装之后按照 bytes 的 ABI 编码即可。

以下是两个交易信息的示例:

```
897c71052abad4ca9a5059f070d5a3a119d1e1ec
00000004
2d910f2c
897c71052abad4ca9a5059f070d5a3a119d1e1ec
00000004
2d910f2c
```

## 操作示例

*首先需要启动一条链，具体方法见快速入门部分*

其中[测试合约](https://github.com/cryptape/cita/blob/develop/scripts/contracts/tests/contracts/test_batch_tx.sol)函数签名如下:

```
======= contracts/test_batch_tx.sol:SelfAdd =======
Function signatures:
2d910f2c: AddOne()
0c55699c: x()
```

其中：

* `AddOne()`表示对x加一
* `x()`表示获取x数值

接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式（与交互式模式的命令是一致的）进行演示。

### 生成随机私钥

```bash
$ cita-cli key create
```

输出：

```json
{
  "address": "0x1828c1cd16bd1f3b50f21137ddeaec5c099c4bbf",
  "private": "0x989ac80f54d2fe79e1cd1b5a425df11177ed40ef8b3e8cfab8e7f65742e61cb9",
  "public": "0xf088eb444f9262cf3c9662802104bfceaf62e7c00e7161f661c640e05422f01397b3c7b104ae7262b02a4a91823bcd62002328ec795b5396c4bd72bf8fa78320"
}
```

### 部署测试合约

* 发送交易

```bash
$ cita-cli rpc sendRawTransaction \
    --code 0x608060405234801561001057600080fd5b5060fd8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680630c55699c14604e5780632d910f2c146076575b600080fd5b348015605957600080fd5b506060608a565b6040518082815260200191505060405180910390f35b348015608157600080fd5b5060886090565b005b60005481565b600160008082825401925050819055506000547f11c1a8e7158fead62641b1e07f61c32daccb5a0432cabfe33a43e8de610042f160405160405180910390a25600a165627a7a7230582021264d3aa498b31d10a5a7086d3e3ba4fb8c23f5a30b64ef8426b19ae2de29870029 \
    --private-key 0x989ac80f54d2fe79e1cd1b5a425df11177ed40ef8b3e8cfab8e7f65742e61cb9 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x6054cd8ba0754eb352ddd283193d3233be559296a7c15cfd50797216cc9b331f",
    "status": "OK"
  }
}
```

* 获取 receipt

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x6054cd8ba0754eb352ddd283193d3233be559296a7c15cfd50797216cc9b331f \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xc4359ce5b1bf054d84f970e4250c79cb3c82cc04695c717370850b088fd059b6",
    "blockNumber": "0xf97",
    "contractAddress": "0x626a7a06fe11041e71efc24b32e304bba7f6038a",
    "cumulativeGasUsed": "0xc66f",
    "errorMessage": null,
    "gasUsed": "0xc66f",
    "logs": [
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0x6054cd8ba0754eb352ddd283193d3233be559296a7c15cfd50797216cc9b331f",
    "transactionIndex": "0x0"
  }
}
```

其中合约地址为： `0x626a7a06fe11041e71efc24b32e304bba7f6038a`

### 查询 x 数值

```bash
$ cita-cli rpc call \
    --to 0x626a7a06fe11041e71efc24b32e304bba7f6038a \
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

### 批量交易

测试批量调用测试合约的 `AddOne()` 函数，预期结构为 x 数值变为 2 。

* 发送交易

```bash
$ cita-cli scm BatchTx multiTxs \
    --tx-code 0x626a7a06fe11041e71efc24b32e304bba7f6038a2d910f2c \
    --tx-code 0x626a7a06fe11041e71efc24b32e304bba7f6038a2d910f2c \
    --private-key 0x989ac80f54d2fe79e1cd1b5a425df11177ed40ef8b3e8cfab8e7f65742e61cb9 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x945bdff33136e7e0392fef1c59ffd66aed45cf87db9b5684d31759aa32abb4d8",
    "status": "OK"
  }
}
```

* 获取receipt

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x945bdff33136e7e0392fef1c59ffd66aed45cf87db9b5684d31759aa32abb4d8 \
    --url http://127.0.0.1:1337
```

输出：

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x6e786244053a3fa51d93d1c31a4b504673b011ce7c3b1ae3447bdee11e51a386",
    "blockNumber": "0x10d1",
    "contractAddress": null,
    "cumulativeGasUsed": "0xd224",
    "errorMessage": null,
    "gasUsed": "0xd224",
    "logs": [
      {
        "address": "0x626a7a06fe11041e71efc24b32e304bba7f6038a",
        "blockHash": "0x6e786244053a3fa51d93d1c31a4b504673b011ce7c3b1ae3447bdee11e51a386",
        "blockNumber": "0x10d1",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0x11c1a8e7158fead62641b1e07f61c32daccb5a0432cabfe33a43e8de610042f1",
          "0x0000000000000000000000000000000000000000000000000000000000000001"
        ],
        "transactionHash": "0x945bdff33136e7e0392fef1c59ffd66aed45cf87db9b5684d31759aa32abb4d8",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0x626a7a06fe11041e71efc24b32e304bba7f6038a",
        "blockHash": "0x6e786244053a3fa51d93d1c31a4b504673b011ce7c3b1ae3447bdee11e51a386",
        "blockNumber": "0x10d1",
        "data": "0x",
        "logIndex": "0x1",
        "topics": [
          "0x11c1a8e7158fead62641b1e07f61c32daccb5a0432cabfe33a43e8de610042f1",
          "0x0000000000000000000000000000000000000000000000000000000000000002"
        ],
        "transactionHash": "0x945bdff33136e7e0392fef1c59ffd66aed45cf87db9b5684d31759aa32abb4d8",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      }
    ],
    "logsBloom": "0x04000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000020000400000040000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000800000000000000000000000000000000000000000000400000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000040000000000000000000000000000000000000000000008000000000000000000000",
    "root": null,
    "transactionHash": "0x945bdff33136e7e0392fef1c59ffd66aed45cf87db9b5684d31759aa32abb4d8",
    "transactionIndex": "0x0"
  }
}
```

这里从 `logs` 已经可以看出两条交易都已经执行成功


### 验证结果

查询 x 数值

```bash
$ cita-cli rpc call \
    --to 0x626a7a06fe11041e71efc24b32e304bba7f6038a \
    --data 0x0c55699c \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000002"
}
```

数值变为 2

批量交易成功执行。
