# 系统配置

使用 CITA 搭建一条链， 会把链的一些基本配置(例如链名称， 链的运营方机构，权限配置等等)写入创世块中，创世块一旦生成，除 chainName, operator, website 三项可以在运行时通过系统合约更改，其他信息无法修改。

### 操作示例：

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示。

仅以管理员设置 `chainName` 作为示例：

确保你的链正常运行，进入 cita-cli 交互式模式，输入命令：
```shell
$ scm SysConfig setChainName --chain-name "AAA" --admin-private \ 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

查询交易回执无误后，我们成功的把链名称从默认的 `test-chain` 更改为 `AAA`。

我们可以通过 `getMeta` 查询更改后的结果，示例如下：

```shell
$ rpc getMetaData
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockInterval": 3000,
    "chainId": 1,
    "chainName": "AAA",
    "economicalModel": 1,
    "genesisTimestamp": 1538101178583,
    "operator": "test-operator",
    "tokenAvatar": "https://avatars1.githubusercontent.com/u/35361817",
    "tokenName": "Nervos AppChain Test Token",
    "tokenSymbol": "NATT",
    "validators": [
      "0x185e7072f53574666cf8ed8ec080e09b7e39c98f"
    ],
    "version": 1,
    "website": "https://www.example.com"
  }
}

```
`chainName` 已更新。

#### 查询类接口

查询类接口不需要权限，默认值可参考 [链的配置](./chain/config_tool.md)

<table style = "text-align: center;">
  <tr>
    <th>名称</th>
    <th>传入参数</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      getDelayBlockNumber()<br/>
      <strong>获取系统合约设置更新后生效需等待的出块数</strong>
    </td>
    <td>
        无
    </td>
    <td>
      所需块数 (uint)
    </td>
    <td>
      获取更新权限、配额等系统合约设置后生效需等待的出块数
    </td>
  </tr>
  <tr>
    <td>
      getPermissionCheck()<br/>
      <strong>查询是否开启权限检查</strong>
    </td>
    <td>
      无
    </td>
    <td>
      状态 (bool)
    </td>
    <td>
      查询是否开启权限检查，true 开启
    </td>
  </tr>
  <tr>
    <td>
      getSendTxPermissionCheck()<br/>
      <strong>查询发送交易是否需要权限</strong>
    </td>
    <td>
      无
    </td>
    <td>
      状态 (bool)
    </td>
    <td>
      查询发送交易是否需要权限，true 需要
    </td>
  </tr>
  <tr>
    <td>
      getCreateContractPermissionCheck()<br/>
      <strong>查询合约创建是否需要权限</strong>
    </td>
    <td>
      无
    </td>
    <td>
      状态 (bool)
    </td>
    <td>
      查询创建合约是否检查权限，true 需要
    </td>
  </tr>
  <tr>
    <td>
      getQuotaCheck()<br/>
      <strong>查询是否开启配额检查</strong>
    </td>
    <td>
      无
    </td>
    <td>
      状态 (bool)
    </td>
    <td>
      查询是否开启配额检查，true 仅当经济模型设置为 quota 模式时表示开始
    </td>
  </tr>
  <tr>
    <td>
      getFeeBackPlatformCheck()<br/>
      <strong>查询出块激励选择</strong>
    </td>
    <td>
      无
    </td>
    <td>
      状态 (bool)
    </td>
    <td>
      查询出块激励选择，true 返还给运营方地址（chainOwner），false 返还给共识节点。
    </td>
  </tr>
  <tr>
    <td>
      getChainOwner()<br/>
      <strong>获取链运营方地址</strong>
    </td>
    <td>
      无
    </td>
    <td>
      链运营方地址 (address)
    </td>
    <td>
      获取链运营方地址
    </td>
  </tr>
  <tr>
    <td>
      getChainName()<br/>
      <strong>获取链名称</strong>
    </td>
    <td>
      无
    </td>
    <td>
      名称 (string)
    </td>
    <td>
      获取链名称
    </td>
  </tr>
  <tr>
    <td>
      getChainId()<br/>
      <strong>获取链 ID</strong>
    </td>
    <td>
      无
    </td>
    <td>
      ID (uint32)
    </td>
    <td>
      获取链 ID
    </td>
  </tr>
  <tr>
    <td>
      getOperator()<br/>
      <strong>获取链运营方名称</strong>
    </td>
    <td>
      无
    </td>
    <td>
      运营方名称 (string)
    </td>
    <td>
      获取链运营方名称
    </td>
  </tr>
  <tr>
    <td>
      getWebsite()<br/>
      <strong>获取链运营方网站</strong>
    </td>
    <td>
      无
    </td>
    <td>
      网站 (string)
    </td>
    <td>
      获取链运营方网站
    </td>
  </tr>
  <tr>
    <td>
      getBlockInterval()<br/>
      <strong>获取出块时间</strong>
    </td>
    <td>
      无
    </td>
    <td>
      时间 (uint64)
    </td>
    <td>
      获取链出块时间（毫秒）
    </td>
  </tr>
  <tr>
    <td>
      getTokenInfo()<br/>
      <strong>获取代币信息</strong>
    </td>
    <td>
      无
    </td>
    <td>
      名称 (string)<br/>
      符号 (string)<br/>
      符号连接 (string)<br/>
    </td>
    <td>
      获取代币相关信息
    </td>
  </tr>
</table>

## 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示， 部分 cita-cli 查询不到的接口，请通过 getMeta 查询。

仅以查询链经济模型作为示例：

确保你的链正常运行，进入 cita-cli 交互式模式，输入命令：
```shell
$ scm SysConfig getEconomicalModel
```

输出:
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}


```

`economicalModel = 1`, 表示链的经济模型为 `charge`。
