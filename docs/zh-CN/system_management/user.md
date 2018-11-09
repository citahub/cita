# 用户管理

CITA 实现了基于组的用户管理，组之间为树形的关系，可对应企业的组织结构。

可使用权限管理系统对组进行授权，组内用户除了本身自己的权限之外还拥有所在组的权限。

对于组的管理，用户在拥有系统内置的权限的前提下，还对权限作用的范围做了约束：

* 一个组内的用户可作用于本组及本组所有子组

相对应的鉴权流程增加对组的权限的鉴定，过程如下：

* 对用户的权限进行鉴定
* 对用户所在组的权限进行鉴定

## 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式进行演示，操作类接口调用需要有相应的权限。

管理员新建用户组，输入命令：

```shell
$ scm GroupManagement newGroup \
      --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
      --name 7770660000000000000000000000000000000000000000000000000000000000 \
      --accounts "[e1c4021742730ded647590a1686d5c4bfcbae0b0,45a50f45cb81c8aedeab917ea0cd3c9178ebdcae]" \
      --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

默认 `origin` 是 `0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009`，我们要生成组名字的十六进制表示 `7770660000000000000000000000000000000000000000000000000000000000`，我们要添加到本用户组内的用户有两个，分别是 `e1c4021742730ded647590a1686d5c4bfcbae0b0`， `45a50f45cb81c8aedeab917ea0cd3c9178ebdcae`


回执输出:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
    "blockNumber": "0x6922",
    "contractAddress": null,
    "cumulativeGasUsed": "0x1b8fcf",
    "errorMessage": null,
    "gasUsed": "0x1b8fcf",
    "logs": [
      {
        "address": "0xce6cd8f8562e31d44b1101986204cec34b1df025",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae",
        "logIndex": "0x0",
        "topics": [
          "0x876145257ed9001029e48f639669c6a3d20c2256585b00a716e557653ccb4813",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009",
          "0x7770660000000000000000000000000000000000000000000000000000000000"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff02000b",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae",
        "logIndex": "0x1",
        "topics": [
          "0xe676706adf1adf2871518b989e3e4ae7c1cc5bf8bb6012ecc94652f84edf4adf",
          "0x000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009",
          "0x7770660000000000000000000000000000000000000000000000000000000000"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020009",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x",
        "logIndex": "0x2",
        "topics": [
          "0xa016866023d98d9af30c4dd99810d92915ae7897f25baa30c8c826bf077f486b",
          "0x000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x2"
      }
    ],
    "logsBloom": "0x00000002000000100000000000800000000000000000400004020000100000000000000000000002000000000080000000000000000000000000000000000002000000000000000000000000000080000004000000001000000000000004000000000000000000000010000000000000100000000020000000000000000000000000000000000000000000000000000000000000001000000000000040000000000000000000000000000000000000000000000000000000000000000000000400400000800004000000000000000000000000000000020010000000000000000000000000000000000000000000008000000000000001000000000000000000",
    "root": null,
    "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
    "transactionIndex": "0x0"
  }
}
```
到这里，我们已经成功新建了一个用户组。从 `log` 中可知，新用户组的地址是: `0xce6cd8f8562e31d44b1101986204cec34b1df025`。

让我们查询一下所有组信息，看看是否添加成功，命令输入：

```shell
$ scm GroupManagement queryGroups
```

回执输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000ffffffffffffffffffffffffffffffffff020009000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
}
```

可以看到 `0xce6cd8f8562e31d44b1101986204cec34b1df025` 已添加。

接着我们根据组地址，来查询组名字，输入命令：
```shell
$ scm Group queryName --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x7770660000000000000000000000000000000000000000000000000000000000"
}
```

可以看到，结果和我们新建组的输入信息一致，厉害了。看看组内都有那些用户吧，输入命令：

查询组用户
```shell
$ scm Group queryAccounts --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae"
}
```
我们在新建组时添加的两个用户已经添加进来了。

因为组之间是树型关系，所以我们也可以根据父组地址，查询子用户组的信息，命令如下：

查询子用户组的地址：
```shell
$ scm Group queryChild --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
}
```

查询子用户组个数:
```shell
$ scm Group queryChildLength --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

反过来，我们也可以根据子用户组地址，来向上查询父用户组的信息，命令如下：
```shell
$ scm Group queryParent --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```
回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009"
}
```
