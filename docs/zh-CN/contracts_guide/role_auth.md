# 角色授权合约接口

<h2 class="hover-list">Role Authorization</h2>

* [queryRoles](#queryRoles)
* [queryAccounts](#queryAccounts)

***

### queryRoles

查询某一账户的所有角色。

* 参数

    `address` - 待查询的账户地址

* 返回值

    `address[]` - 拥有的角色列表

* 示例

```shell
$ scm RoleManagement queryRoles --account 0x101e99e1a654a99308175042aff4833a6528be74
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000558c280233cee856fb53931eb18747a40e688a430000000000000000000000001be912bdfe6ae5d28f7e9d2f1a5329788e5a4fe6"
}
```

### queryAccounts

查询某一角色下的所有账户。

* 参数

    `address` - 角色地址

* 返回值

    `address[]` - 拥有此角色的所有账户

* 示例

```shell
$ scm RoleManagement queryAccounts --address 0x558c280233cee856fb53931eb18747a40e688a43
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001000000000000000000000000101e99e1a654a99308175042aff4833a6528be74"
}
```
