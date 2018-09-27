# 系统配置

使用 CITA 搭建一条链，用户需要在区块链生成创世块时完成配置，创世块一旦生成，只有 chainName, operator, website 这三项可通过本合约接口进行修改。

## 初始化

sys_config.sol 合约通过写入创世块的方式来初始化，查看 [链的配置](../chain/config_tool.md)

## 接口

### 操作类接口

<table style = "text-align: center;">
  <tr>
    <th>名称</th>
    <th>需要权限</th>
    <th>传入参数</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      setOperator(operator)<br/>
      <strong>设置区块链运营方名称</strong>
      </td>
      <td>管理员</td>
    <td>
      运营方名称 (string)
    <td>无</td>
    <td>设置运营方名称</td>
  </tr>
  <tr>
    <td>
      setWebsite(website)<br/>
      <strong>设置运营方网站</strong>
    </td>
    <td>管理员</td>
    <td>
      网址 (string)
    <td>无</td>
    <td>设置运营方网站</td>
  </tr>
  <tr>
    <td>
      setChainName(chainName)<br/>
      <strong>设置区块链名称</strong>
    </td>
    <td>管理员</td>
    <td>
      链名称 (string)
    <td>无</td>
    <td>设置区块链名称</td>
  </tr>
</table>

#### 查询类接口

查询类接口不需要权限，默认值可参考 [链的配置](../chain/config_tool.md)

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
