# 用户管理

CITA实现了基于组的用户管理，组之间为树形的关系，可对应企业的组织结构。

可使用权限管理系统对组进行授权，组内用户除了本身自己的权限之外还拥有所在组的权限。

对于组的管理，用户在拥有系统内置的权限的前提下，还对权限作用的范围做了约束：

* 一个组内的用户可作用于本组及本组所有子组

相对应的鉴权流程增加对组的权限的鉴定，过程如下：

* 对用户的权限进行鉴定
* 对用户所在组的权限进行鉴定

## 接口说明

### 操作类接口

<table>
  <tr>
    <th>name</th>
    <th>permissions</th>
    <th>parameters</th>
    <th>return</th>
    <th>describe</th>
  </tr>
  <tr>
    <td>
      newGroup(origin, name, accounts) <br/>
      <strong>Create a new group</strong>
    </td>
    <td>newGroup</td>
    <td>
      origin: The user's origin group
      <br/>
      name: The name of the new group
      <br/>
      accounts: The accounts of the new group
    </td>
    <td>The address of the new group</td>
    <td>A group is a smart contract</td>
  </tr>
  <tr>
    <td>
      deleteGroup(origin, target) <br/>
      <strong>Delete a group</strong>
    </td>
    <td>deleteGroup</td>
    <td>
      origin: The user's origin group
      <br/>
      target: The target group that will be deleted
    </td>
    <td>true/false</td>
    <td>Close the smart contract</td>
  </tr>
  <tr>
    <td>
      updateGroupName(origin, target, name) <br/>
      <strong>Update the group name</strong>
    </td>
    <td>updateGroup</td>
    <td>
      origin: The user's origin group
      <br/>
      target: The target group that will be updated
      <br/>
      name: The new name of the group
    </td>
    <td>true/false</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      addAccounts(origin, target, accounts) <br/>
      <strong>Add the group's accounts</strong>
    </td>
    <td>updateGroup</td>
    <td>
      origin: The user's origin group
      <br/>
      target: The target group that will be updated
      <br/>
      accounts: The accounts will be added
    </td>
    <td>true/false</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      deleteAccounts(origin, target, accounts) <br/>
      <strong>Delete the group's accounts</strong>
    </td>
    <td>updateGroup</td>
    <td>
      origin: The user's origin group
      <br/>
      target: The target group that will be updated
      <br/>
      accounts: The accounts will be deleted
    </td>
    <td>true/false</td>
    <td>None</td>
  </tr>
</table>

### query

#### group_management

<table>
  <tr>
    <th>name</th>
    <th>parameters</th>
    <th>return</th>
    <th>describe</th>
  </tr>
  <tr>
    <td>
      checkScope(origin, name) <br/>
      <strong>Check the target group in the scope of the origin group</strong>
    </td>
    <td>
      origin: The user's origin group
      <br/>
      target: The target group
    </td>
    <td>true/false</td>
    <td>The origin group is ancestor of the target group</td>
  </tr>
  <tr>
    <td>
      queryGroups() <br/>
      <strong>Query all groups</strong>
    </td>
    <td>None</td>
    <td>All groups</td>
    <td>None</td>
  </tr>
</table>

#### 查询类接口

查询类接口不需要权限。

<table>
  <tr>
    <th>name</th>
    <th>parameters</th>
    <th>return</th>
    <th>describe</th>
  </tr>
  <tr>
    <td>
      queryInfo() <br/>
      <strong>Query the information of group</strong>
    </td>
    <td>None</td>
    <td>Include the name and accounts</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryName() <br/>
      <strong>Query the name of group</strong>
    </td>
    <td>None</td>
    <td>The name of group</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryAccounts() <br/>
      <strong>Query the accounts of group</strong>
    </td>
    <td>None</td>
    <td>The accounts of group</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryChild() <br/>
      <strong>Query the children of group</strong>
    </td>
    <td>None</td>
    <td>The children of group</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryChildLength() <br/>
      <strong>Query the number of children</strong>
    </td>
    <td>None</td>
    <td>The number of children</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryParent() <br/>
      <strong>Query the parent of group</strong>
    </td>
    <td>None</td>
    <td>The parent of group</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      inGroup() <br/>
      <strong>Check the user in the group</strong>
    </td>
    <td>None</td>
    <td>true/false</td>
    <td>None</td>
  </tr>
</table>
