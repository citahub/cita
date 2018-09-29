# 协议版本管理

CITA 通过设置协议版本号的方式，激活硬分叉，升级系统。该合约实现了协议版本号的设置与查询。

## 接口说明

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
      setVersion(version) <br/>
      <strong>设置版本号</strong>
    </td>
    <td>管理员</td>
    <td>
      版本号 (uint32)
    <td>无</td>
    <td>设置新版本号</td>
  </tr>
</table>

### 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示。

目前的 `version协议版本号` 默认为0， 接下来演示管理员如何修改协议版本号。

确保你的链正常运行，并且拥有相应的权限，进入 cita-cli 交互式模式，输入命令：
```shell
scm VersionManager setVersion --version 1 --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

查询交易回执无误后，我们成功的把协议版本号从默认的 `0` 更改为 `1`。

#### 查询类接口

查询类接口不需要权限。

<table style = "text-align: center;">
  <tr>
    <th>名称</th>
    <th>传入参数</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      getVersion() <br/>
      <strong>获取当前版本号</strong>
    </td>
    <td>
        无
    </td>
    <td>当前版本号 (uint32)</td>
    <td>获取当前版本号</td>
  </tr>
</table>

### 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示。

确保你的链正常运行，进入 cita-cli 交互式模式，输入命令：
```shell
scm VersionManager getVersion
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```
可以看到，协议版本号成功修改为 `1`。
