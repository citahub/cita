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

## 配额管理合约接口

<table>
  <tr>
    <th>名称</th>
    <th>需要权限</th>
    <th>传入参数</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      setBQL(quotaLimit)<br/>
      <strong>设置区块配额上限</strong>
    </td>
    <td>管理员</td>
    <td>quotaLimit uint: 配额值</td>
    <td>操作是否成功 (bool)</td>
    <td>设置每个块的配额上限</td>
  </tr>
  <tr>
    <td>
      setDefaultAQL(quotaLimit)<br/>
      <strong>设置默认账号配额上限</strong>
    </td>
    <td>管理员</td>
    <td>quotaLimit uint: 配额值</td>
    <td>操作是否成功 (bool)</td>
    <td>设置默认的账号配额上限</td>
  </tr>
  <tr>
    <td>
      setAQL(address, quotaLimit) <br/>
      <strong>设置指定账号配额上限</strong>
    </td>
    <td>管理员</td>
    <td>
      address: 指定的账号的地址
      <br/>
      quotaLimit uint: 设置的配额值
    </td>
    <td>操作是否成功 (bool)</td>
    <td>设置指定账号的配额上限</td>
  </tr>
  <tr>
    <td>
      getBQL() <br/>
      <strong>查询区块配额上限</strong>
    </td>
    <td>None</td>
    <td>None</td>
    <td>查询到的配额上限 (uint)</td>
    <td>查询设置的区块配额上限</td>
  </tr>
  <tr>
    <td>
      getDefaultAQL() <br/>
      <strong>查询默认账号配额上限</strong>
    </td>
    <td>None</td>
    <td>None</td>
    <td>查询到的配额上限 (unit)</td>
    <td>查询设置的默认账号配额上限</td>
  </tr>
  <tr>
    <td>
      getAQL <br/>
      <strong>查询指定账号配额上限</strong>
    </td>
    <td>None</td>
    <td>address: 为指定的账号地址</td>
    <td>查询到的配额上限 (uint)</td>
    <td>查询设置的指定账号配额上限</td>
  </tr>
  <tr>
    <td>
      getAccounts <br/>
      <strong>查询所有指定账号</strong>
    </td>
    <td>None</td>
    <td>None</td>
    <td>查询到的指定账户的列表</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      getQuotas <br/>
      <strong>查询所有指定账号的配额上限</strong>
    </td>
    <td>None</td>
    <td>None</td>
    <td>查询到的配额上限列表</td>
    <td>None</td>
  </tr>
</table>

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