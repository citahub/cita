# 用户可调用的权限管理的接口
*简单文档，后续补充再整合入docs*

涉及到的合约的函数签名如下(部分添加了初始化的合约地址):
*也可以是用`solc role_management.sol --hash`生成*

```
======= authorization.sol:Authorization =======
Address: 0x00000000000000000000000000000000013241b4
Function signatures: 
0c0a5c55: cancelAuth(address,address)
8ad2f289: checkPermission(address,address,bytes4)
b4026ed5: clearAuth(address)
223964bc: queryAccounts(address)
945a2555: queryPermissions(address)
f10a7798: setAuth(address,address)

======= permission.sol:Permission =======
Function signatures: 
87f0bf31: addResources(address[],bytes4[])
43d726d6: close()
2f8cfe0e: deleteResources(address[],bytes4[])
19c38c66: inPermission(address,bytes4)
2c560ec0: queryInfo()
1ae97bd9: updateName(bytes32)

======= permission_creator.sol:PermissionCreator =======
Address: 0x00000000000000000000000000000000013241b3
Function signatures: 
ae8f1d29: createPermission(bytes32,address[],bytes4[])

======= permission_management.sol:PermissionManagement =======
Address: 0x00000000000000000000000000000000013241b2
Function signatures: 
f036ed56: addResources(address,address[],bytes4[])
3482e0c9: cancelAuthorization(address,address)
a5925b5b: clearAuthorization(address)
98a05bb1: deletePermission(address)
6446ebd8: deleteResources(address,address[],bytes4[])
fc4a089c: newPermission(bytes32,address[],bytes4[])
0f5aa9f3: setAuthorization(address,address)
537bf9a3: updatePermissionName(address,bytes32)

======= role.sol:Role =======
Function signatures: 
0bc1734c: addPermissions(address[])
e66ec7cc: applyRolePermissionsOf(address)
40e20fb7: cancelRolePermissionsOf(address)
5c30b9df: clearRolePermissionsOf(address)
ae5942cd: deletePermissions(address[])
126004b8: deleteRole()
71d6e229: queryRole()
1ae97bd9: updateName(bytes32)

======= role_creator.sol:RoleCreator =======
Address: 0xe9e2593c7d1db5ee843c143e9cb52b8d996b2380
Function signatures: 
9630961d: createRole(bytes32,address[])

======= role_management.sol:RoleManagement =======
Address: 0xe3b5ddb80addb513b5c981e27bb030a86a8821ee
Function signatures: 
0773e6ba: addPermissions(address,address[])
a8319481: cancelRole(address,address)
c631e758: clearRole(address)
17b2e350: deletePermissions(address,address[])
54b025c5: deleteRole(address)
551ef860: newRole(bytes32,address[])
a32710eb: setRole(address,address)
d9c090a0: updateRoleName(address,bytes32)
```

## permission_mangement

All

## role_management

All

## permission_creator

None

## permission

permision id即为合约的地址

查询类：
```
19c38c66: inPermission(address,bytes4)
2c560ec0: queryInfo()
```
## role

role id即为合约的地址

查询类：
```
71d6e229: queryRole()
```

## role_creator

Node
