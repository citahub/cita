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

* Parameters

    `address` - The group address

* Returns

    `bytes32` - The name of group

    `address[]` - The accounts of group

* Example

```shell
$ scm Group queryInfo --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryName

查询组名字。

* Parameters

    `address` - The group address

* Returns

    `bytes32` - The name of group

* Example

```shell
$ scm Group queryName --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryAccounts

查询组内所有用户。

* Parameters

    `address` - The group address

* Returns

    `address[]` - All accounts address

* Example

```shell
$ scm Group queryAccounts --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

### queryChild

查询子组。

* Parameters

    `address` - The group address

* Returns

    `address` - The children of group

* Example

```shell
$ scm Group queryChild --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryChildLength

查询子组个数。

* Parameters

    `address` - The group address

* Returns

    `uint` - The number of the children group

* Example

```shell
$ scm Group queryChildLength --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

### queryParent

查询父组。

* Parameters

    `address` - The group address

* Returns

    `address` - The parent of the group

* Example

```shell
$ scm Group queryParent --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```
