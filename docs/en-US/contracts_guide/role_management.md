# 角色管理合约接口

<h2 class="hover-list">Role Management</h2>

* [newRole](#newRole)
* [updateRoleName](#updateRoleName)
* [addPermissions](#addPermissions)
* [deletePermissions](#deletePermissions)
* [setRole](#setRole)
* [cancelRole](#cancelRole)
* [clearRole](#clearRole)
* [deleteRole](#deleteRole)

***

### newRole

新建角色。

* Parameters

    `bytes32` - The role name

    `address[]` - The permissions

* Returns

    `address` - The role address

* Example

```shell
$ scm RoleManagement newRole \
        --name 73747564656e7400000000000000000000000000000000000000000000000000 \
        --permissions '[ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee, 1acec7eaba22b46ba5d2a7c0bfc94a7741dfd32b]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

回执output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x156f87e4bf3563df3ea500ba3334bd873c1d3bbc749da5b899c14bd7ae00d3a6",
    "blockNumber": "0x21096",
    "contractAddress": null,
    "cumulativeQuotaUsed": "0x172c46",
    "errorMessage": null,
    "logs": [
      {
        "address": "0x558c280233cee856fb53931eb18747a40e688a43",
        "blockHash": "0x156f87e4bf3563df3ea500ba3334bd873c1d3bbc749da5b899c14bd7ae00d3a6",
        "blockNumber": "0x21096",
        "data": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee0000000000000000000000001acec7eaba22b46ba5d2a7c0bfc94a7741dfd32b",
        "logIndex": "0x0",
        "topics": [
          "0x5f961877a57fd34379ca2259585e1bf0392c0fa570593a4109a903898a993ec4",
          "0x73747564656e7400000000000000000000000000000000000000000000000000"
        ],
        "transactionHash": "0x61bc2da013ffea0e45f03d72103c1ec75dedeb74452b2fc465c478d58f43a420",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020008",
        "blockHash": "0x156f87e4bf3563df3ea500ba3334bd873c1d3bbc749da5b899c14bd7ae00d3a6",
        "blockNumber": "0x21096",
        "data": "0x",
        "logIndex": "0x1",
        "topics": [
          "0x8b0dfb31766ab53d2fb03166733d946b844f4b2da0ebce4b2b9323b8c5342e6c",
          "0x000000000000000000000000558c280233cee856fb53931eb18747a40e688a43",
          "0x73747564656e7400000000000000000000000000000000000000000000000000",
          "0x974d92309b0f7ddca104ffdda1ab73bb341d7aeded49a4a0f61f294e8b8bea6a"
        ],
        "transactionHash": "0x61bc2da013ffea0e45f03d72103c1ec75dedeb74452b2fc465c478d58f43a420",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      }
    ],
    "logsBloom": "0x00000000008000000000000000000000020000000000000000000000000000000000000000000008000000000000004000000000000000000004000000000000000000000000000000000000000000000000010000000000080000000000000000000000080000000000000000000000000000800000000008000000000000000000000800000000000000000000000000000001080000000000000000000102000080000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000002002010000000000000000000800000000000000000000000000000000000000000000000000000000",
    "quotaUsed": "0x172c46",
    "root": null,
    "transactionHash": "0x61bc2da013ffea0e45f03d72103c1ec75dedeb74452b2fc465c478d58f43a420",
    "transactionIndex": "0x0"
  }
}
```

从 log topic[1] 中找到新的角色合约地址：`0x000000000000000000000000558c280233cee856fb53931eb18747a40e688a43`

### updateRoleName

更新角色名称。

* Parameters

    `bytes32` - The role name

    `address` - The role address

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement updateRoleName \
        --name 0000000000000000000000000000000000000000000000000000000060fe47b1 \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### addPermissions

为角色添加权限。

* Parameters

    `address` - The role address

    `address[]` - The role permissions

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement addPermissions \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --permissions '[558c280233cee856fb53931eb18747a40e688a43]' \ 
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deletePermissions

删除权限。

* Parameters

    `address` - The role address

    `address[]` - The permissions

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement deletePermissions \ 
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --permissions '[558c280233cee856fb53931eb18747a40e688a43]' \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e66 \
```

### setRole

为某一个账户设置角色。

* Parameters

    `address` - The account address

    `address` - The role address

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement setRole \
        --account 0x101e99e1a654a99308175042aff4833a6528be74 \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### cancelRole

清除某个账户的指定权限

* Parameters

    `address` - The account address

    `address` - The role address

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement cancelRole \
        --account 0x101e99e1a654a99308175042aff4833a6528be74 \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e \
```

### clearRole

清除某个账户的所有权限。

* Parameters

    `address` - The account address

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement clearRole \
        --account 0x101e99e1a654a99308175042aff4833a6528be74 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### deleteRole

删除角色。

* Parameters

    `address` - The role address

* Returns

    `bool`

* Example

```shell
$ scm RoleManagement deleteRole \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```
