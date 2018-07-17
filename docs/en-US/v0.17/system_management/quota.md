# 配额管理

## 配额管理概述

通过配额管理合约实现对区块(中的视图）以及用户配额消耗上限的管理:

* 设置区块配额上限即为每个区块设置统一的配额上限;
* 设置账号配额上限包括:

    - 默认的账号配额上限，全局设置，即若账号未指定配额上限，默认为此值;
    - 设置指定账号配额上限，可针对不同用户灵活分配对应的配额上限。

## 配额管理合约接口

说明:

* BQL: BlockQuotaLimit 缩写
* AQL: AccountQuotaLimit 缩写

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
