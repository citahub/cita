# 权限管理合约接口

<h2 class="hover-list">Permission Management</h2>

* [newPermission](#newPermission)
* [deletePermission](#deletePermission)
* [updatePermissionName](#updatePermissionName)
* [addResources](#addResources)
* [deleteResources](#deleteResources)
* [setAuthorizations](#setAuthorizations)
* [setAuthorization](#setAuthorization)
* [cancelAuthorizations](#cancelAuthorizations)
* [cancelAuthorization](#cancelAuthorization)
* [clearAuthorization](#clearAuthorization)

***

### newPermission

创建新权限。

* 参数

    `bytes32` - The permission name

    `address[]`- The contracts of resource

    `bytes4[]` - The function signature of the resource

* 返回值

    `address` - New permission's address.

* 示例

```shell
$ scm PermissionManagement newPermission \
        --name 0000000000000000000000000000000000000000000000000000000060fe47b1 \
        --contracts '[5839153e0efe76efe0c974b728c4f49ca7ed75cc]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6\
```

获取回执：

```shell
$ rpc getTransactionReceipt --hash 0x2bf039eeeefbfb0724fcdebdcbc74de0f3b61e0212279981b548c9884f018b8f
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xd2e2d34783d9d30505ed23d90dca7c11ce42eda99306a153ad9e72095832ba26",
    "blockNumber": "0x583d",
    "contractAddress": null,
    "cumulativeGasUsed": "0x1b25fd",
    "errorMessage": null,
    "gasUsed": "0x1b25fd",
    "logs": [
      {
        "address": "0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee",
        "blockHash": "0xd2e2d34783d9d30505ed23d90dca7c11ce42eda99306a153ad9e72095832ba26",
        "blockNumber": "0x583d",
        "data": "0x0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000010000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc000000000000000000000000000000000000000000000000000000000000000160fe47b100000000000000000000000000000000000000000000000000000000",
        "logIndex": "0x0",
        "topics": [
          "0xb533e8b79dc7485ba7e4435e3395df911c1a3c767225941003d88a7812d216f7"
        ],
        "transactionHash": "0x2bf039eeeefbfb0724fcdebdcbc74de0f3b61e0212279981b548c9884f018b8f",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020005",
        "blockHash": "0xd2e2d34783d9d30505ed23d90dca7c11ce42eda99306a153ad9e72095832ba26",
        "blockNumber": "0x583d",
        "data": "0x0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000010000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc000000000000000000000000000000000000000000000000000000000000000160fe47b100000000000000000000000000000000000000000000000000000000",
        "logIndex": "0x1",
        "topics": [
          "0x792f7322d94960c6e90863b5aef39075ca54620cfa13a822081d733f79c48f91",
          "0x000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee",
          "0x0000000000000000000000000000000000000000000000000000000060fe47b1"
        ],
        "transactionHash": "0x2bf039eeeefbfb0724fcdebdcbc74de0f3b61e0212279981b548c9884f018b8f",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      }
    ],
    "logsBloom": "0x00000000000000020000000000000000000000000000008000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000080000000000000000040000000000000000040000000000000000001000000000000000000000000000000000000000000000000008000000800000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000200000000000000000000000000000000000000000000000000000000000000000000100000000000000000000100000000000000000000000000000800000002000000000000100000000100000",
    "root": null,
    "transactionHash": "0x2bf039eeeefbfb0724fcdebdcbc74de0f3b61e0212279981b548c9884f018b8f",
    "transactionIndex": "0x0"
  }
}
```

从 logs[0] 中获得新权限的地址为 `0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee`

### deletePermission

删除权限。

* 参数

    `address` - The permission address

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

### updatePermissionName

更新权限名称。

* 参数

    `address` - The permission address

    `bytes32`  - The permission name

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
 $ scm PermissionManagement updatePermissionName \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --name 0000000000000000000000000000000000000000000000000000000060fe47b2 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### addResources

添加资源。

* 参数

    `address` - The permission address

    `address[]` - The contracts of resource

    `bytes4[]` - The function signature of resource
* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement addResources \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --contracts '[1e041ec9a18590924d84a1f011eb0749c03fc41a]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### deleteResources

删除资源。

* 参数

    `address` - The permission address

    `address[]` - The contracts of resource

    `bytes4[]` - The function signature of resource

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement deleteResources \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --contracts '[1e041ec9a18590924d84a1f011eb0749c03fc41a]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### setAuthorizations

多次授权。

* 参数

    `address` - The account to be setted

    `address[]` - The permissions to be setted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement setAuthorizations \
    --permissions '[ffffffffffffffffffffffffffffffffff021000,ffffffffffffffffffffffffffffffffff021001]' \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### setAuthorization

授权。

* 参数

    `address` - The account to be setted

    `address` - The permission to be setted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement setAuthorization \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### cancelAuthorizations

取消多次授权。

* 参数

    `address` - The account address

    `address[]` - The permissions to be canceled

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement cancelAuthorizations \
    --permissions '[ffffffffffffffffffffffffffffffffff021000,ffffffffffffffffffffffffffffffffff021001]' \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### cancelAuthorization

取消授权

* 参数

    `address` - The account address

    `address` - The permission to be canceled

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm PermissionManagement cancelAuthorization \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### clearAuthorization

取消账户的所有授权。

* 参数

    `address` - The account's address

* 返回值

    `bool` - True, if successfully, otherwise false.
