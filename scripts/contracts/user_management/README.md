# 用户管理合约的接口

涉及到的合约的函数签名如下(部分添加了初始化的合约地址):
*可用`solc group_management.sol --hash`生成*

```
======= group.sol:Group =======
Function signatures: 
ac71abde: addAccounts(address[])
1eee993a: addChild(address)
43d726d6: close()
0846ca3c: deleteAccounts(address[])
8ea7296b: deleteChild(address)
cc798890: queryAccounts()
e4be5159: queryChild()
c065ecc2: queryChildLength()
2c560ec0: queryInfo()
379725ee: queryName()
5ce5ba9b: queryParent()
1ae97bd9: updateName(bytes32)

======= group_creator.sol:GroupCreator =======
Function signatures: 
3c673470: createGroup(address,bytes32,address[])

======= group_management.sol:GroupManagement =======
Function signatures: 
2c84e31f: addAccounts(address,address,address[])
eadf4672: checkScope(address,address)
d86df333: deleteAccounts(address,address,address[])
baeb8cad: deleteGroup(address,address)
d7cd7209: newGroup(address,bytes32,address[])
7eafcdb1: updateGroupName(address,address,bytes32)
```

## group_management

* Address: 0x00000000000000000000000000000000013241C2
* Interface: All

## group_creator

* Address: 0x00000000000000000000000000000000013241c3
* Interface: None

## group

* Address: group id即为合约的地址
* Interface: 查询类

```
Function signatures:
cc798890: queryAccounts()
e4be5159: queryChild()
c065ecc2: queryChildLength()
2c560ec0: queryInfo()
379725ee: queryName()
5ce5ba9b: queryParent()
```
