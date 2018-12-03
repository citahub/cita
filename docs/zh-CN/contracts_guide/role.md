# 角色合约接口

<h2 class="hover-list">Role</h2>

* [queryName](#queryName)
* [queryPermissions](#queryPermissions)
* [lengthOfPermissions](#lengthOfPermissions)
* [inPermissions](#inPermissions)

***

### queryName

查询角色名称。

* 参数

    空

* 返回值

    `bytes32` - 角色名称

* 示例

```shell
$ scm Role queryName --address 0x558c280233cee856fb53931eb18747a40e688a43
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000060fe47b1"
}
```

### queryPermissions

查询角色所有权限。

* 参数

    空

* 返回值

    `address[]` - 角色拥有的权限列表

* 示例

```shell
$ scm Role queryPermissions --address 0x558c280233cee856fb53931eb18747a40e688a43
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee0000000000000000000000001acec7eaba22b46ba5d2a7c0bfc94a7741dfd32b000000000000000000000000558c280233cee856fb53931eb18747a40e688a43"
}
```

### lengthOfPermissions

查询角色拥有权限数。

* 参数

    空

* 返回值

    `uint` - 角色拥有的权限个数

* 示例

```shell
$ scm Role lengthOfPermissions --address 0x558c280233cee856fb53931eb18747a40e688a43
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000002"
}

```

### inPermissions

判断权限是否存在角色中

* 参数

    `address` - 权限地址

* 返回值

    `bool` - 在则返回为真，反之则反

* 示例

```shell
$  scm Role inPermissions \
        --address 0x558c280233cee856fb53931eb18747a40e688a43 \
        --permission 0x1acec7eaba22b46ba5d2a7c0bfc94a7741dfd32b \
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```
