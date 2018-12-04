# 配额管理合约接口

<h2 class="hover-list">Quota Management</h2>

* [setBQL](#setBQL)
* [setDefaultAQL](#setDefaultAQL)
* [setAQL](#setAQL)
* [getAccounts](#getAccounts)
* [getQuotas](#getQuotas)
* [getBQL](#getBQL)
* [getDefaultAQL](#getDefaultAQL)
* [getAQL](#getAQL)
* [getAutoExecAQL](#getAQL)

### setBQL

设置区块配额上限。

* 参数

    `uint` - The value to be setted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm QuotaManager setBQL \
        --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setDefaultAQL

设置默认账号配额上限

* 参数

    空

* 返回值

    `uint` - The value

* 示例

```shell
$ scm QuotaManager setDefaultAQL \
    --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setAQL

设置指定账号配额上限。

* 参数

    `uint` - The value to be setted

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm QuotaManager setAQL \
    --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### getAccounts

查询所有指定账号。

* 参数

    空

* 返回值

    `address[]` - The accounts that have AQL

* 示例

```shell
$ scm QuotaManager getAccounts
```

### getQuotas

查询所有指定账号的配额上限。

* 参数

    空

* 返回值

    `uint[]` - The accounts' quotas

* 示例

```shell
$ scm QuotaManager getQuotas
```

### getBQL

查询默认块配额。

* 参数

    空

* 返回值

    `uint` - The value

* 示例

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

* 参数

    空

* 返回值

    `uint` - The value

* 示例

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

* 参数

    `address` - The account address

* 返回值

    `uint` - The account quota value

* 示例

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

### getAutoExecQL

查询自动执行配额限制。

* 参数

    None

* 返回值

    `uint` - The autoExec quota limit value
