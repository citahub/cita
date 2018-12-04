# 角色授权合约接口

<h2 class="hover-list">Role Authorization</h2>

* [queryRoles](#queryRoles)
* [queryAccounts](#queryAccounts)

***

### queryRoles

查询某一账户的所有角色。

* Parameters

    `address` - The account address

* Returns

    `address[]` - The roles of the account

* Example

```shell
$ scm RoleManagement queryRoles --account 0x101e99e1a654a99308175042aff4833a6528be74
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000558c280233cee856fb53931eb18747a40e688a430000000000000000000000001be912bdfe6ae5d28f7e9d2f1a5329788e5a4fe6"
}
```

### queryAccounts

查询某一角色下的所有账户。

* Parameters

    `address` - The role address

* Returns

    `address[]` - The accounts address

* Example

```shell
$ scm RoleManagement queryAccounts --address 0x558c280233cee856fb53931eb18747a40e688a43
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001000000000000000000000000101e99e1a654a99308175042aff4833a6528be74"
}
```
