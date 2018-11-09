# 配额管理

## 简述

cita 中的配额数量存在两个限制：
* `BQL(BlockQuotaLimit)` : 表示块配额的最大值， 默认 1073741824
* `AQL(AccountQuotaLimit)` : 表示账户配额的最大值， 默认 268435456

我们可以通过配额管理合约实现对区块以及账户配额消耗上限的管理:

* 设置区块配额上限(BQL)
* 设置账号配额上限(AQL):

    - 默认的账号配额上限
    - 设置指定账号配额上限

## 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示。

### 块配额操作

确保你的链正常运行，查询默认块配额，进入 cita-cli 交互式模式，输入命令：
```shell
$ scm QuotaManager getBQL
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000040000000"
}
```

管理员修改块配额， 输入命令：

```shell
scm QuotaManager setBQL --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

查询修改后的块配额：
```shell
$ scm QuotaManager getBQL
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000020000000"
  "
}
```
默认块配额已更新。

### 账户配额操作

确保你的链正常运行，查询默认账户配额，进入 cita-cli 交互式模式，输入命令：
```shell
$ scm QuotaManager getDefaultAQL
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000010000000"
}
```

管理员修改账户配额， 输入命令：

```shell
$ scm QuotaManager setDefaultAQL --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

查询修改后的账户配额：
```shell
$ scm QuotaManager getDefaultAQL
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000020000000"
}
```
默认账户配额已更新。
