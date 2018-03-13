# 用户可调用的权限管理的接口
*接口调用说明，详细见[docs](https://cryptape.github.io/cita/system_management/account/)*

涉及到的合约的函数签名如下(部分添加了初始化的合约地址):
*也可以`solc role_management.sol --hash`生成*

```
======= authorization.sol:Authorization =======
Function signatures: 
0c0a5c55: cancelAuth(address,address)
8ad2f289: checkPermission(address,address,bytes4)
b4026ed5: clearAuth(address)
372cd7ad: clearAuthOfPermission(address)
223964bc: queryAccounts(address)
d28d4e0c: queryAllAccounts()
945a2555: queryPermissions(address)
f10a7798: setAuth(address,address)

======= permission.sol:Permission =======
Function signatures: 
87f0bf31: addResources(address[],bytes4[])
43d726d6: close()
2f8cfe0e: deleteResources(address[],bytes4[])
19c38c66: inPermission(address,bytes4)
2c560ec0: queryInfo()
379725ee: queryName()
53f4a519: queryResource()
1ae97bd9: updateName(bytes32)

======= permission_creator.sol:PermissionCreator =======
Function signatures: 
ae8f1d29: createPermission(bytes32,address[],bytes4[])

======= permission_management.sol:PermissionManagement =======
Function signatures: 
f036ed56: addResources(address,address[],bytes4[])
3482e0c9: cancelAuthorization(address,address)
ba00ab60: cancelAuthorizations(address,address[])
a5925b5b: clearAuthorization(address)
98a05bb1: deletePermission(address)
6446ebd8: deleteResources(address,address[],bytes4[])
fc4a089c: newPermission(bytes32,address[],bytes4[])
0f5aa9f3: setAuthorization(address,address)
52c5b4cc: setAuthorizations(address,address[])
537bf9a3: updatePermissionName(address,bytes32)

======= role.sol:Role =======
Function signatures: 
0bc1734c: addPermissions(address[])
e66ec7cc: applyRolePermissionsOf(address)
40e20fb7: cancelRolePermissionsOf(address)
5c30b9df: clearRolePermissionsOf(address)
ae5942cd: deletePermissions(address[])
126004b8: deleteRole()
379725ee: queryName()
46f02832: queryPermissions()
71d6e229: queryRole()
1ae97bd9: updateName(bytes32)

======= role_creator.sol:RoleCreator =======
Function signatures: 
9630961d: createRole(bytes32,address[])

======= role_management.sol:RoleManagement =======
Function signatures: 
0773e6ba: addPermissions(address,address[])
a8319481: cancelRole(address,address)
c631e758: clearRole(address)
17b2e350: deletePermissions(address,address[])
54b025c5: deleteRole(address)
551ef860: newRole(bytes32,address[])
223964bc: queryAccounts(address)
ef8322fd: queryRoles(address)
a32710eb: setRole(address,address)
d9c090a0: updateRoleName(address,bytes32)
```

## permission_mangement

* Address: 0x00000000000000000000000000000000013241b2
* Interface: All

## role_management

* Address: 0xe3b5ddb80addb513b5c981e27bb030a86a8821ee
* Interface: All

## permission_creator

* Address: 0x00000000000000000000000000000000013241b3
* Interface: None

## permission

* Address: permision id即为合约的地址
* Interface: 查询类

```
19c38c66: inPermission(address,bytes4)
2c560ec0: queryInfo()
379725ee: queryName()
53f4a519: queryResource()
```

## authorization

* Address: 0x00000000000000000000000000000000013241b4
* Interface: 查询类

```
223964bc: queryAccounts(address)
d28d4e0c: queryAllAccounts()
945a2555: queryPermissions(address)
```

## role

* Address: role id即为合约的地址
* Interface: 查询类

```
379725ee: queryName()
46f02832: queryPermissions()
71d6e229: queryRole()
```

## role_creator

* Address: 0xe9e2593c7d1db5ee843c143e9cb52b8d996b2380
* Interface: None
