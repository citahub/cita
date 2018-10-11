# CITA Contracts

<h2 class="hover-list">System Contracts</h2>

* [setChainName](#setChainName)
* [setOperator](#setOperator)
* [setWebsite](#setWebsite)
* [getPermissionCheck](#getPermissionCheck)
* [getQuotaCheck](#getQuotaCheck)
* [getFeeBackPlatformCheck](#getFeeBackPlatformCheck)
* [getChainOwner](#getChainOwner)

***

<h2 class="hover-list">Node Management</h2>

* [approveNode](#approveNode)
* [deleteNode](#deleteNode)
* [listNode](#listNode)
* [setStake](#setStake)
* [getStatus](#getStatus)
* [listStake](#listStake)
* [stakePermillage](#stakePermillage)

***

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
* [inPermission](#inPermission)
* [queryInfo](#queryInfo)
* [queryName](#queryName)
* [queryResource](#queryResource)

***

<h2 class="hover-list">Role Management</h2>

* [newRole](#newRole)
* [deleteRole](#deleteRole)
* [updateRoleName](#updateRoleName)
* [addPermissions](#addPermissions)
* [deletePermissions](#deletePermissions)
* [setRole](#setRole)
* [cancelRole](#cancelRole)
* [clearRole](#clearRole)
* [queryRoles](#queryRoles)
* [queryAccounts](#queryAccounts)
* [queryName](#queryName)
* [queryPermissions](#queryPermissions)
* [lengthOfPermissions](#lengthOfPermissions)
* [inPermissions](#inPermissions)

***

<h2 class="hover-list">Authorization Management</h2>

* [queryPermissions](#queryPermissions)
* [queryAccounts](#queryAccounts)
* [queryAllAccounts](#queryAllAccounts)
* [addPermissions](#addPermissions)
* [checkResource](#checkResource)
* [checkPermission](#checkPermission)

***


<h2 class="hover-list">Group Management</h2>

* [newGroup](#newGroup)
* [deleteGroup](#deleteGroup)
* [updateGroupName](#updateGroupName)
* [addAccounts](#addAccounts)
* [deleteAccounts](#deleteAccounts)
* [checkScope](#checkScope)
* [queryGroups](#queryGroups)

***

<h2 class="hover-list">Users Management</h2>

* [queryInfo](#queryInfo)
* [queryName](#queryName)
* [queryAccounts](#queryAccounts)
* [queryChild](#queryChild)
* [queryChildLength](#queryChildLength)
* [queryParent](#queryParent)

***

<h2 class="hover-list">Quota Management</h2>

* [setBQL](#setBQL)
* [setDefaultAQL](#setDefaultAQL)
* [setAQL](#setAQL)
* [getAccounts](#getAccounts)
* [getQuotas](#getQuotas)
* [getBQL](#getBQL)
* [getDefaultAQL](#getDefaultAQL)
* [getAQL](#getAQL)

***

<h2 class="hover-list">Batch Tx</h2>

* [multiTxs](#multiTxs)

***

<h2 class="hover-list">Admin Management Interfaces</h2>

* [isAdmin](#isAdmin)
* [update](#update)

***

<h2 class="hover-list">Version Management Interfaces</h2>

* [setVersion](#setVersion)
* [getVersion](#getVersion)

***

<h2 class="hover-list">Price Management Interfaces</h2>

* [setQuotaPrice](#setQuotaPrice)
* [getQuotaPrice](#getQuotaPrice)

***

<h2 class="hover-list">Emergency brake  Interfaces</h2>

* [setState](#setState)

***

### setChainName

设置链名称。

* Parameters

    `String chainName` - The Chain name

* Returns

    `None`

* Example

```shell
$ scm SysConfig setChainName \
        --chain-name "AAA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setOperator

设置运营方。

* Parameters

    `String operator` - The Chain operator

* Returns

    `None`

* Example

```shell
 $ scm SysConfig setOperator \
        --operator "CITA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setWebsite

设置运营方地址。

* Parameters

    `String website` - The Operator website

* Returns

    `None`

* Example

```shell
$ scm SysConfig setWebsite \
        --website "https://github.com/cryptape" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### getPermissionCheck

查询权限是否开启。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getPermissionCheck
```

### getQuotaCheck

查询 quota 检查是否开启。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getQuotaCheck
```

### getFeeBackPlatformCheck

查询出块激励返回开关是否开启。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getFeeBackPlatformCheck
```

### getChainOwner

查询链的持有者地址。

* Parameters

    `height(Optional)`

* Returns

    `address` - The chain owner's address

* Example

```shell
$ scm SysConfig getChainOwner
```

### approveNode

确认共识节点。

* Parameters

    `address node` - The new node address

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm NodeManager approveNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deleteNode

删除共识节点。

* Parameters

    `address node` - The node address

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm NodeManager deleteNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### listNode

共识节点列表。

* Parameters

    `None`

* Returns

    `address[]` - The consensus nodes

* Example

```shell
$ scm NodeManager listNode
```

### setStake

设置共识节点 stake 。

* Parameters

    `address node , uint64 stake` - The node address and stake to be setted.

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm NodeManager setStake \
        --address 0xae0f69a2d95146d104365e0502a0d521717ced7f \
        --stake 0000000000000000000000000000000000000000000000000000000000000002 \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### getStatus

获取共识节点状态。

* Parameters

    `address` - The node address

* Returns

    `uint8` - 0: closed, 1: started

* Example

```shell
$ scm NodeManager getStatus --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```

### listStake

共识节点 stake 列表。

* Parameters

    `height(Optional)`

* Returns

    `uint64[] stakes` - The node stakes list

* Example

```shell
$ scm NodeManager listStake
```

### stakePermillage

共识节点出块权重千分比。

* Parameters

    `address` - The node address

* Returns

    `uint64` - The node stake permillage.

* Example

```shell
$ scm NodeManager stakePermillage --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```

### newPermission

创建新权限。

* Parameters

    `bytes32 name` - The permission name

    `address[] conts`- The contracts of resource

    `bytes4[] funcs` - The function signature of the resource

* Returns

    `address` - New permission's address.

* Example

```shell
$ scm PermissionManagement newPermission \
        --name 0000000000000000000000000000000000000000000000000000000060fe47b1 \
        --contracts '[5839153e0efe76efe0c974b728c4f49ca7ed75cc]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6\
```

```json
$ rpc getTransactionReceipt --hash 0x2bf039eeeefbfb0724fcdebdcbc74de0f3b61e0212279981b548c9884f018b8f
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

* Parameters

    `address permission` - The permission address

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

### updatePermissionName

更新权限名称。

* Parameters

    `address permission` - The permission address

    `bytes32 name`  - The permission name

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
 $ scm PermissionManagement updatePermissionName \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --name 0000000000000000000000000000000000000000000000000000000060fe47b2 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### addResources

添加资源。

* Parameters

    `address permission` - The permission address

    `address[] contracts` - The contracts of resource

    `bytes4[] function-hashes` - The function signature of resource
* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement addResources \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --contracts '[1e041ec9a18590924d84a1f011eb0749c03fc41a]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deleteResources

删除资源。

* Parameters

    `address permission` - The permission address

    `address[] contracts` - The contracts of resource

    `bytes4[] function-hashes` - The function signature of resource

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement deleteResources \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --contracts '[1e041ec9a18590924d84a1f011eb0749c03fc41a]' \
        --function-hashes '[60fe47b1]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setAuthorizations

多次授权。

* Parameters

    `address account` - The account to be setted

    `address[] permissions` - The permissions to be setted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement setAuthorizations \
    --permissions '[ffffffffffffffffffffffffffffffffff021000,ffffffffffffffffffffffffffffffffff021001]' \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setAuthorization

授权。

* Parameters

    `address account` - The account to be setted

    `address permission` - The permission to be setted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement setAuthorization \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### cancelAuthorizations

取消多次授权。

* Parameters

    `address account` - The account address

    `address[] permissions` - The permissions to be canceled

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement cancelAuthorizations \
    --permissions '[ffffffffffffffffffffffffffffffffff021000,ffffffffffffffffffffffffffffffffff021001]' \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### cancelAuthorization

取消授权

* Parameters

    `address account` - The account address

    `address permission` - The permission to be canceled

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm PermissionManagement cancelAuthorization \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
        --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### clearAuthorization

取消账户的所有授权。

* Parameters

    `address account` - The account's address

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

### inPermission

检查资源是否在 permission 中。

* Parameters

    `address contract` - The contract address of the resource

    `bytes4 function-hash` -  The function signature of the resource

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm Permission inPermission \
        --contract 0x1e041ec9a18590924d84a1f011eb0749c03fc41a \
        --function-hash 0x60fe47b1 \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

### queryInfo

* Parameters

    `address permission` - The permission address

* Returns

    `bytes32 permission` - The permission name

    `address[] cont` - The contract address of the resource

    `bytes4[] func` - The function signature of the resource

* Example

```shell
$ scm Permission queryInfo --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000060fe47b2000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc0000000000000000000000001e041ec9a18590924d84a1f011eb0749c03fc41a000000000000000000000000000000000000000000000000000000000000000260fe47b10000000000000000000000000000000000000000000000000000000060fe47b100000000000000000000000000000000000000000000000000000000"
}
```

### queryName

* Parameters

    `address permission` - The permission address

* Returns

    `bytes32 name` - The permission name

* Example

```shell
$ scm Permission queryName --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000060fe47b2"
}
```

### queryResource

* Parameters

    `address permission` - The permission address

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm Permission queryResource --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000020000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc0000000000000000000000001e041ec9a18590924d84a1f011eb0749c03fc41a000000000000000000000000000000000000000000000000000000000000000260fe47b10000000000000000000000000000000000000000000000000000000060fe47b100000000000000000000000000000000000000000000000000000000"
}

```

### newRole
### deleteRole
### updateRoleName
### addPermissions
### deletePermissions
### setRole
### cancelRole
### clearRole
### queryRoles
### queryAccounts
### queryName
### queryPermissions
### lengthOfPermissions
### inPermissions
### queryPermissions
### queryAccounts
### queryAllAccounts
### addPermissions
### checkResource
### checkPermission

### newGroup

创建一个用户组。

* Parameters

    `address origin` - The sender's origin group

    `bytes32 name` -  The name of group

    `address[] accounts` - The accounts of group

* Returns

    `address` - The group address

* Example

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

* Parameters

    `address origin` - The sender's orgin group

    `address target` -  The target group to be deleted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm GroupManagement deleteGroup \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
         --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### updateGroupName

更新用户组名称。

* Parameters

    `address origin` - The sender's orgin group

    `address target` -  The target group to be deleted

    `bytes32 name` - The new name to be updated

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm GroupManagement updateGroupName \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \ --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
        --name 8880660000000000000000000000000000000000000000000000000000000000 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### addAccounts

添加用户。

* Parameters

    `address origin` - The sender's orgin group

    `address target` -  The target group to be deleted

    `address[] accounts` - The accounts to be added

* Returns

    `bool` - True, if successfully, otherwise false.

* Examplee

```shell
 $ scm GroupManagement addAccounts \
         --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \ --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
         --accounts '[887d3378018c45ec72bed1947d34ac59a4402ddb,f7636f910e2fff0014d693498fe43d2e539b8742]' \
          --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deleteAccounts

删除用户。

* Parameters

    `address origin` - The sender's orgin group

    `address target` -  The target group to be deleted

    `address[] accounts` - The accounts to be added

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm GroupManagement deleteAccounts \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
        --accounts '[887d3378018c45ec72bed1947d34ac59a4402ddb,f7636f910e2fff0014d693498fe43d2e539b8742]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### checkScope

* Parameters

    `address origin` - The sender's orgin group

    `address target` -  The target group to be deleted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm GroupManagement checkScope \
        --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
        --target 0xce6cd8f8562e31d44b1101986204cec34b1df025 \
```

### queryGroups

查询所有组。

* Parameters

    `None`

* Returns

    `address[]` - All groups address

* Example

```shell
$ scm GroupManagement queryGroups
```

### queryInfo

查询组信息。

* Parameters

    `address group` - The group address

* Returns

    `bytes32 name` - The name of group

    `address[] accounts` - The accounts of group

* Example

```shell
$ scm Group queryInfo --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryName

查询组名字。

* Parameters

    `address group` - The group address

* Returns

    `bytes32 name` - The name of group

* Example

```shell
$ scm Group queryName --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryAccounts

查询组内所有用户。

* Parameters

    `address group` - The group address

* Returns

    `address[]` - All accounts address

* Example

```shell
$ scm Group queryAccounts --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryChild

查询子组。

* Parameters

    `address group` - The group address

* Returns

    `address` - The children of group

* Example

```shell
$ scm Group queryChild --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryChildLength

查询子组个数。

* Parameters

    `address group` - The group address

* Returns

    `uint` - The number of the children group

* Example

```shell
$ scm Group queryChildLength --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryParent

查询父组。

* Parameters

    `address group` - The group address

* Returns

    `address` - The parent of the group

* Example

```shell
$ scm Group queryParent --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### setBQL

设置区块配额上限。

* Parameters

    `uint value` - The value to be setted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm QuotaManager setBQL \
        --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setDefaultAQL

设置默认账号配额上限

* Parameters

    `None`

* Returns

    `uint value` - The value

* Example

```shell
$ scm QuotaManager setDefaultAQL \
    --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setAQL

设置指定账号配额上限。

* Parameters

    `uint value` - The value to be setted

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm QuotaManager getAQL --address 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523
```

### getAccounts

查询所有指定账号。

* Parameters

    `None`

* Returns

    `address[] accounts` - The accounts that have AQL

* Example

```shell
$ scm QuotaManager getAccounts
```

### getQuotas

查询所有指定账号的配额上限。

* Parameters

    `None`

* Returns

    `uint[] value` - The accounts' quotas

* Example

```shell
$ scm QuotaManager getQuotas
```

### getBQL

查询默认块配额。

* Parameters

    `None`

* Returns

    `uint value` - The value

* Example

```shell
$ scm QuotaManager getBQL
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000040000000"
}
```


### getDefaultAQL

查询默认账户配额。

* Parameters

    `None`

* Returns

    `uint value` - The value

* Example

```shell
$ scm QuotaManager getDefaultAQL
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000010000000"
}
```

### getAQL

查询某一账户配额。

* Parameters

    `address account` - The account address

* Returns

    `uint value` - The account quota value

* Example

```shell
$ scm QuotaManager getAQL --address 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000040000000"
}
```

### multiTxs
### isAdmin
### update
### setVersion

设置协议号版本。

* Parameters

    `uint version` - The version

* Returns

    `None`

* Example

```shell
$ cita-cli scm VersionManager setVersion \
            --version 1 \
            --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### getVersion

* Parameters

    `None`

* Returns

    `uint` - The version

* Example

```shell
$ cita-cli scm VersionManager getVersion
```

```json
// Result
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

### setQuotaPrice

设置 `quota price`，默认为 1。

* Parameters

    `uint price` - The setting quota price

* Returns

    `Boolean` - True if success,other false.

* Example

```shell
$ cita-cli scm PriceManager setQuotaPrice \
              --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
              --price 0x0000000000000000000000000000000000000000000000000000000000000002
```

### getQuotaPrice

查询当前链 quota price。

* Parameters

    `None`

* Returns

    `uint` - The quota price

* Example

```shell
$ cita-cli scm PriceManager getQuotaPrice
```

```json
// Result
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000002"
}

```

### setState

开启紧急制动模式。

* Parameters

    `bool state` - state

* Returns

    `None`

* Example

```shell
$ cita-cli scm EmergencyBrake setState \
    --state true \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```