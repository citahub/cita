# 数据订正

## 数据订正概述

数据订正 Amend 是超级管理员 Super Admin 通过发送特定的交易到链上，来干预或者修正链的运行

* Amend 交易的发送者必须是系统的超级管理员，否则会提示没有权限执行
* Amend 交易的目的地址是特殊地址：0xffffffffffffffffffffffffffffffffff010002

## 数据订正的类型

Amend 交易现在有四种不同的数据订正的类型 ABI,Code,Balance,Key-Value, 组装不同类型的 Amend 交易的差别，主要体现在构造 Transaction 时的 Value 和 Data 值，下面介绍的数据订正类型：

### ABI
ABI 类型用来修改账户的二进制接口信息：
交易中 Value 的值为1，Data 字段前20个字节为账户地址，后面的字节为 ABI 的数据

### Code
Code 类型用来修改合约账户内的代码：
交易中 Value 的值为2，Data 字段前20个字节为合约账户地址，后面的字节为二进制的 Code

### Key->Value
Key->Value 类型用来修改某账户使用的底层KV数据库的key-value信息：
交易中 Value 的值为3，Data 字段前20个字节为账户地址，后面的字节是一系列的 Key-Value 对，
前32字节为 Key，后32字节为 Value 交替存放。

**注意：此操作针对了解区块链存储的专业人士使用，慎用！**

### Balance

Balance 类型用来修改账户内部的资金 Balance 的数值：
交易中 Value 的值为5，Data字段前20个字节为账户地址，后32个字节组成U256的数值，作为将要设置的Balance的数值