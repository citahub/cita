# 账号管理

CITA统一对账号进行基于角色的权限管理。系统内置了两种角色:

* 管理员角色拥有全部权限。包括管理节点的权限、发送交易的权限、创建合约的权限以及所有普通角色的权限;
* 普通角色拥有读取的权限以及创建角色的权限。包括验证节点是否是共识节点、获取共识节点列表、判断是否拥有角色、获取角色列表、判断是否拥有权限、获取权限列表等。

其中管理员角色和管理员绑定，即管理员账号有且只有管理员角色。管理员的添加只能由管理员操作，并且无法进行删除操作。角色可由用户创建，可相互授予及收回，而角色对应的权限只可由管理员进行设置。

CITA通过智能合约的方式来对账号进行管理。其中账号管理合约用来管理角色，权限管理合约用来管理角色的权限。

### 账号管理合约接口

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
      createRole(name, permissions) <br/>
      <strong>创建角色</strong>
    </td>
    <td>用户角色</td>
    <td>
      name bytes32: 角色名称
      <br/>
      permissions uint8[]: 权限列表
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后创建一个新的角色，并拥有创建者指定权限，默认创建角色继承创建者角色权限，需调用权限管理合约接口</td>
  </tr>
  <tr>
    <td>
      addAdmin(address) <br/>
      <strong>添加管理员</strong>
    </td>
    <td>管理员角色</td>
    <td>address: 为账号地址</td>
    <td>操作是否成功 (bool)</td>
    <td>成功后授予账号管理员角色，即添加了新的管理员</td>
  </tr>
  <tr>
    <td>
      ownRole(address, roleName) <br/>
      <strong>判断账号是否拥有角色</strong>
    </td>
    <td>用户角色 (只读)</td>
    <td>
      address: 账号地址
      <br/>
      roleName bytes32: 角色名称
    </td>
    <td>操作是否成功 (bool)</td>
    <td>判断账号是否拥有指定的角色</td>
  </tr>
  <tr>
    <td>
      listRole(address) <br/>
      <strong>查询账号拥有的角色</strong>
    </td>
    <td>用户角色</td>
    <td>address: 为账号地址</td>
    <td>账号拥有的角色列表 (bytes32[MAX_ROLE]，其中 MAX_ROLE 为角色拥有的最大角色数)</td>
    <td>读取账号所拥有的权限</td>
  </tr>
  <tr>
    <td>
      grandRole(address, roleName) <br/>
      <strong>授予指定账号角色</strong>
    </td>
    <td>用户角色</td>
    <td>
      address: 账号地址
      <br/>
      roleName bytes32: 角色名称
    </td>
    <td>操作是否成功 (bool)</td>
    <td>对给定账号授予已存在的角色，成功后该账号拥有此角色</td>
  </tr>
  <tr>
    <td>
      revokeRole(address, roleName) <br/>
      <strong>收回指定账号角色</strong>
    </td>
    <td>角色创建者或管理员角色</td>
    <td>
      address: 账号地址
      <br/>
      roleName bytes32: 角色名称
    </td>
    <td>操作是否成功 (bool)</td>
    <td>撤销账号拥有的角色，成功后该账号失去此角色</td>
  </tr>
</table>

### 权限管理合约接口

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
      setRolePermission(roleName, permissions)<br/>
      <strong>设置角色权限</strong>
    </td>
    <td>管理员角色</td>
    <td>
      roleName bytes32: 角色名称
      <br/>
      permissions uint8[]: 权限列表
    </td>
    <td>操作是否成功 (bool)</td>
    <td>只能由管理员授予角色权限，成功后角色拥有指定权限</td>
  </tr>
  <tr>
    <td>
      ownPermission(roleName, permission) <br/>
      <strong>判断角色是否拥有权限</strong>
    </td>
    <td>用户角色 (只读)</td>
    <td>
      roleName bytes32: 角色名称
      <br/>
      permission uint8: 权限名称
    </td>
    <td>角色是否拥有该权限 (bool)</td>
    <td>判断角色是否拥有指定的权限</td>
  </tr>
  <tr>
    <td>
      listPermission(roleName) <br/>
      <strong>查询角色拥有的权限</strong>
    </td>
    <td>用户角色</td>
    <td>bytes32: 为角色名称</td>
    <td>为角色拥有的权限列表 (bytes32[MAX_PERMISSION]，其中MAX_PERMISSION为角色拥有的最大权限数)</td>
    <td>读取角色所拥有的权限</td>
  </tr>
</table>

## 配额管理

通过配额管理合约实现对区块(中的视图）以及用户配额消耗上限的管理:

* 设置区块配额上限即为每个区块设置统一的配额上限;
* 设置账号配额上限包括:

    - 默认的账号配额上限，全局设置，即若账号未指定配额上限，默认为此值;
    - 设置指定账号配额上限，可针对不同用户灵活分配对应的配额上限。

### 配额管理合约接口

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
    <td>管理员角色</td>
    <td>quotaLimit uint: 配额值</td>
    <td>操作是否成功 (bool)</td>
    <td>设置每个块的配额上限</td>
  </tr>
  <tr>
    <td>
      setDefaultAQL(quotaLimit)<br/>
      <strong>设置默认账号配额上限</strong>
    </td>
    <td>管理员角色</td>
    <td>quotaLimit uint: 配额值</td>
    <td>操作是否成功 (bool)</td>
    <td>设置默认的账号配额上限</td>
  </tr>
  <tr>
    <td>
      setAQL(address, quotaLimit) <br/>
      <strong>设置指定账号配额上限</strong>
    </td>
    <td>管理员角色</td>
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
    <td>普通角色</td>
    <td>空</td>
    <td>查询到的配额上限 (uint)</td>
    <td>查询设置的区块配额上限</td>
  </tr>
  <tr>
    <td>
      getDefaultAQL() <br/>
      <strong>查询默认账号配额上限</strong>
    </td>
    <td>普通角色</td>
    <td>空</td>
    <td>查询到的配额上限 (unit)</td>
    <td>查询设置的默认账号配额上限</td>
  </tr>
  <tr>
    <td>
      getAQL <br/>
      <strong>查询指定账号配额上限</strong>
    </td>
    <td>普通角色</td>
    <td>address: 为指定的账号地址</td>
    <td>查询到的配额上限 (uint)</td>
    <td>查询设置的指定账号配额上限</td>
  </tr>
</table>
