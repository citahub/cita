# 组管理合约接口

<h2 class="hover-list">Group Management</h2>

* [newGroup](#newGroup)
* [deleteGroup](#deleteGroup)
* [updateGroupName](#updateGroupName)
* [addAccounts](#addAccounts)
* [deleteAccounts](#deleteAccounts)
* [checkScope](#checkScope)
* [queryGroups](#queryGroups)

***

### newGroup

创建一个用户组。

* 参数

    `address` - The sender's origin group

    `bytes32` -  The name of group

    `address[]` - The accounts of group

* 返回值

    `address` - The group address

* 示例

```shell
$ scm GroupManagement newGroup \
      --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
      --name 7770660000000000000000000000000000000000000000000000000000000000 \
      --accounts "[e1c4021742730ded647590a1686d5c4bfcbae0b0,45a50f45cb81c8aedeab917ea0cd3c9178ebdcae]" \
      --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

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
从 log 中可知，新用户组的地址是: 0xce6cd8f8562e31d44b1101986204cec34b1df025

### deleteGroup

删除用户组。

* 参数

    `address` - The sender's orgin group

    `address` -  The target group to be deleted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm GroupManagement deleteGroup \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
         --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### updateGroupName

更新用户组名称。

* 参数

    `address` - The sender's orgin group

    `address` -  The target group to be deleted

    `bytes32` - The new name to be updated

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm GroupManagement updateGroupName \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \ --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
        --name 8880660000000000000000000000000000000000000000000000000000000000 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### addAccounts

添加用户。

* 参数

    `address` - The sender's orgin group

    `address` -  The target group to be deleted

    `address[]` - The accounts to be added

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例e

```shell
 $ scm GroupManagement addAccounts \
         --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \ --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
         --accounts '[887d3378018c45ec72bed1947d34ac59a4402ddb,f7636f910e2fff0014d693498fe43d2e539b8742]' \
          --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deleteAccounts

删除用户。

* 参数

    `address` - The sender's orgin group

    `address` -  The target group to be deleted

    `address[]` - The accounts to be added

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm GroupManagement deleteAccounts \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
        --accounts '[887d3378018c45ec72bed1947d34ac59a4402ddb,f7636f910e2fff0014d693498fe43d2e539b8742]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### checkScope

* 参数

    `address` - The sender's orgin group

    `address` -  The target group to be deleted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm GroupManagement checkScope \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
```

### queryGroups

查询所有组。

* 参数

    空

* 返回值

    `address[]` - All groups address

* 示例

```shell
$ scm GroupManagement queryGroups
```
