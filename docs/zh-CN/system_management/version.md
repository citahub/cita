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
