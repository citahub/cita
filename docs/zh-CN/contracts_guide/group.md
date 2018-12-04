# 组合约接口

<h2 class="hover-list">Users Management</h2>

* [queryInfo](#queryInfo)
* [queryName](#queryName)
* [queryAccounts](#queryAccounts)
* [queryChild](#queryChild)
* [queryChildLength](#queryChildLength)
* [queryParent](#queryParent)

***

### queryInfo

查询组信息。

* 参数

    `address` - The group address

* 返回值

    `bytes32` - The name of group

    `address[]` - The accounts of group

* 示例

```shell
$ scm Group queryInfo --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryName

查询组名字。

* 参数

    `address` - The group address

* 返回值

    `bytes32` - The name of group

* 示例

```shell
$ scm Group queryName --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryAccounts

查询组内所有用户。

* 参数

    `address` - The group address

* 返回值

    `address[]` - All accounts address

* 示例

```shell
$ scm Group queryAccounts --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryChild

查询子组。

* 参数

    `address` - The group address

* 返回值

    `address` - The children of group

* 示例

```shell
$ scm Group queryChild --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryChildLength

查询子组个数。

* 参数

    `address` - The group address

* 返回值

    `uint` - The number of the children group

* 示例

```shell
$ scm Group queryChildLength --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryParent

查询父组。

* 参数

    `address` - The group address

* 返回值

    `address` - The parent of the group

* 示例

```shell
$ scm Group queryParent --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```
