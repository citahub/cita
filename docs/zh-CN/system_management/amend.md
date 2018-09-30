# 数据订正

## 简述

数据订正 Amend 是超级管理员 Super Admin 通过发送特定的交易到链上，来干预或者修正链的运行

* Amend 交易的发送者必须是系统的超级管理员，否则会提示没有权限执行
* Amend 交易的目的地址是特殊地址：0xffffffffffffffffffffffffffffffffff010002

## 数据订正的类型

Amend 交易现在有四种不同的数据订正的类型 ABI,Code,Balance,Key-Value, 发送不同类型的 Amend 交易的差别，主要体现在 cita-cli 的 Amend 子命令中，一般情况下 

* address 参数为账户地址

* admin-private-key 参数为超级管理员的私钥

* url 参数后面是 JSON-RPC 的地址

下面介绍具体的数据订正操作：

### ABI
ABI 类型用来修改账户的二进制接口信息，参数如下：
amend abi 代表修改 ABI信息，命令中的 content参数 的值为 ABI 的数据信息

- 示例：

修改 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 的 ABI 信息
```bash
./cita-cli amend abi --content "[{"constant":false,"inputs":[],"name":"setGoStraight","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"getChoice","outputs":[{"name":"","type":"uint8"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"getDefaultChoice","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]" --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --admin-private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 --url http://127.0.0.1:1337
```

查询新的 ABI 信息
```bash
./cita-cli rpc getAbi --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --url http://127.0.0.1:1337
```

### Code
Code 类型用来修改合约账户内的代码，参数如下：
amend code 代表修改 Code 信息，命令中的 content 参数 的值为 Code 的二进制数据信息 

- 示例：

修改 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 的 Code 信息
```bash
./cita-cli amend code --content 0x608060405234801561001057600080fd5b50610152806100206000396000f300608060405260043610610057576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806346aadaa51461005c57806367cb61b614610073578063843f7258146100ac575b600080fd5b34801561006857600080fd5b506100716100d7565b005b34801561007f57600080fd5b506100886100fc565b6040518082600381111561009857fe5b60ff16815260200191505060405180910390f35b3480156100b857600080fd5b506100c1610112565b6040518082815260200191505060405180910390f35b60026000806101000a81548160ff021916908360038111156100f557fe5b0217905550565b60008060009054906101000a900460ff16905090565b60006002600381111561012157fe5b9050905600a165627a7a723058207a9f4ef112e089314a40d0efbfe8e88e3c04add43fcee5cbae6cd9f55d9d0ef30029 --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --admin-private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 --url http://127.0.0.1:1337
```

查询新的code信息

```bash
./cita-cli rpc getCode --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --url http://127.0.0.1:1337
```

### Balance

Balance 类型用来修改账户内部的资金 Balance 的数值，参数如下：
amend balance 代表修改 Balance 信息，命令中的 balance 参数数值为修改后的数值

- 示例：

修改 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 的 Balance 数值
```bash
./cita-cli amend balance --balance 0x88888 --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --admin-private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 --url http://127.0.0.1:1337
```

查看新的 Balance 数值
```bash
./cita-cli rpc getBalance --address 0xa4691ea78dbc3c1fa5fcc78c67ffbffe8c6bdeb7 --url http://127.0.0.1:1337
```

### Key->Value
Key->Value 类型用来修改某账户使用的底层KV数据库的 Key-Value信息,参数如下：
amend kv-h256 代表修改数据库的 KV 信息，命令中的 kv 参数值为一系列的 Key-Value 对，
前面的 H256 信息 Key，后面的 H256 信息为 Value,交替存放。

- 示例：

修改账户 0xffffffffffffffffffffffffffffffffff020000 的 key 0x000000000000000000000000000000000000000000000000000000000000002b 的值为 0x0000000000000000000000000000000000000000000000010000000000000bb8
```bash
./cita-cli amend kv-h256 --address 0xffffffffffffffffffffffffffffffffff020000 --admin-private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 --kv 0x000000000000000000000000000000000000000000000000000000000000002b 0x0000000000000000000000000000000000000000000000010000000000000bb8 --url http://127.0.0.1:1337
```

**注意：此操作针对了解区块链存储的专业人士使用，慎用！**
