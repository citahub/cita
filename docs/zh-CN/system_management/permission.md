# 权限管理

CITA实现了对账户的权限管理，并支持基于角色的权限管理。

CITA通过智能合约的方式来对权限进行管理。

## 账户概述

* 账户(account)： 链上唯一的标识，权限管理的主体对象。

    - 外部账户： 拥有公私钥对，可发送交易的用户。
    - 合约账户： 拥有相关的代码(code)及存储(storage)。

目前权限管理针对外部账户进行细粒度管理。CITA 默认集成了 superAdmin 账户，拥有权限管理涉及到的所有权限。在 CITA 启动前可以对 superAdmin 进行配置。
在权限系统开启时，由用户生成的外部账户，在 CITA 系统中没有任何权限，需要 superAdmin 对其进行授权。

权限管理默认未开启，配置相关信息查看[系统合约](./chain/config_tool)

## 权限管理概述

权限(permission)在此系统中的定义为多个资源(resource)的集合，其中资源(resource)为一个合约地址及一个函数签名。

### 系统默认权限类型

用户可自定义权限，其中系统内置了几种权限(禁止对其进行删除操作)，如下所示：

* `sendTx`:            表示发交易权限
* `createContract`:    表示创建合约权限
* `newPermission`:     表示创建一个新的权限权限
* `deletePermission`:  表示删除一个权限权限
* `updatePermission`:  表示更新一个权限权限
* `setAuth`:           表示对账号进行授权权限
* `cancelAuth`:        表示对帐号取消授权权限
* `newRole`:           表示创建一个新的角色权限
* `deleteRole`:        表示删除一个角色权限
* `updateRole`:        表示更新一个角色权限
* `setRole`:           表示对账号授予角色权限
* `cancelRole`:        表示对帐号取消授予角色权限
* `newGroup`:          表示创建一个新组权限
* `deleteGroup`:       表示删除一个组权限
* `updateGroup`:       表示更新一个组权限
* `newNode`:           表示增加普通节点权限
* `deleteNode`:        表示删除节点权限
* `updateNode`:        表示更新节点权限
* `accountQuota`:      表示账户配额设置权限
* `blockQuota`:        表示块配额设置权限
* `batchTx`:           表示批量交易权限
* `ermergencyBrake`:   表示紧急制动权限
* `quotaPrice`:        表示设置 quotaPrice 权限
* `version`:           表示设置版本权限

可以查看具体[权限的地址信息](https://github.com/cryptape/cita/blob/develop/cita-chain/types/src/reserved_addresses.rs)

## 权限管理操作实例

### 修改系统配置

通过以下命令生成配置文件(打开权限开关)：

```bash
$ ./env.sh ./scripts/create_cita_config.py create \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
	--contract_arguments SysConfig.checkCallPermission=true SysConfig.checkSendTxPermission=true SysConfig.checkCreateContractPermission=true
```

其中 `checkCallPermission`, `checkSendTxPermission`, `checkCreateContractPermission` 分别为合约调用、发送交易及创建合约的开关。

启动链接下来的步骤见[快速搭链](./chain/getting_started)部分。接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式（与交互式模式的命令是一致的）进行演示。

### 生成普通账户

```bash
$ cita-cli key create
```

输出：

```json
{
  "address": "0x37d1c7449bfe76fe9c445e626da06265e9377601",
  "private": "0x3ef2627393529fed043c7dbfd9358a4ae47a88a59949b07e7631722fd6959002",
  "public": "0x9dc6fc7856f5271e6e8c45e5c5fe22d2ff699ac3b24497599be77803d3c25fb4e2fe7da616c65a291910c947c89923009f354634421bddd0a25cd0a509bcf6a9"
}
```

### 部署合约

使用[测试合约](https://github.com/cryptape/cita/blob/develop/scripts/contracts/tests/contracts/SimpleStorage.sol)

#### 获得合约的相关信息

* 字节码

```bash
$ solc test_example.sol --bin
```

输出：

```
======= test_example.sol:SimpleStorage =======
Binary:
608060405234801561001057600080fd5b5060df8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c146078575b600080fd5b348015605957600080fd5b5060766004803603810190808035906020019092919050505060a0565b005b348015608357600080fd5b50608a60aa565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a723058205aed214856a5c433292a354261c9eb88eed1396c83dabbe105bde142e49838ac0029
```

* 函数签名

```bash
$ solc test_example.sol --hashes
```

输出：

```
======= test_example.sol:SimpleStorage =======
Function signatures:
6d4ce63c: get()
60fe47b1: set(uint256)
```

#### 部署合约

由于设置了权限的检查开关，所有用户默认是没有发交易及创建合约的权限的。首先需要通过 superAdmin 对其授 sendTx 发送交易及 createContract 创建合约权限。

* 授予发送交易和创建合约权限

发送交易权限地址为 `0xffffffffffffffffffffffffffffffffff021000`，创建合约权限地址为 `0xffffffffffffffffffffffffffffffffff021001`

由管理员进行操作。调用 `setAuthorizations`接口。

```bash
$ cita-cli scm PermissionManagement setAuthorizations \
    --permissions '[ffffffffffffffffffffffffffffffffff021000,ffffffffffffffffffffffffffffffffff021001]' \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x8addbd232737efb41e2aa45b481fe578b93f4bfb8dd6a971aad5e7593c3c47d2",
    "status": "OK"
  }
}
```

查看 receipt 信息：

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x8addbd232737efb41e2aa45b481fe578b93f4bfb8dd6a971aad5e7593c3c47d2 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x5956727ccb1de4876be148ef6fa54c764544cfd78cdc48b083603b40fbb71b8b",
    "blockNumber": "0x14",
    "contractAddress": null,
    "cumulativeGasUsed": "0x27a38",
    "errorMessage": null,
    "gasUsed": "0x27a38",
    "logs": [
      {
        "address": "0xffffffffffffffffffffffffffffffffff020006",
        "blockHash": "0x5956727ccb1de4876be148ef6fa54c764544cfd78cdc48b083603b40fbb71b8b",
        "blockNumber": "0x14",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0xef79a70821e438468db437d5f7401aecaf406a2cba3c7e7fd4339ef895dbb97e",
          "0x00000000000000000000000037d1c7449bfe76fe9c445e626da06265e9377601",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff021000"
        ],
        "transactionHash": "0x8addbd232737efb41e2aa45b481fe578b93f4bfb8dd6a971aad5e7593c3c47d2",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020006",
        "blockHash": "0x5956727ccb1de4876be148ef6fa54c764544cfd78cdc48b083603b40fbb71b8b",
        "blockNumber": "0x14",
        "data": "0x",
        "logIndex": "0x1",
        "topics": [
          "0xef79a70821e438468db437d5f7401aecaf406a2cba3c7e7fd4339ef895dbb97e",
          "0x00000000000000000000000037d1c7449bfe76fe9c445e626da06265e9377601",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff021001"
        ],
        "transactionHash": "0x8addbd232737efb41e2aa45b481fe578b93f4bfb8dd6a971aad5e7593c3c47d2",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      }
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000400001000000200000000000000000000020000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000001000000010000000000000100000000000000000000000000000000000000008800080000000000000000000000002000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0x8addbd232737efb41e2aa45b481fe578b93f4bfb8dd6a971aad5e7593c3c47d2",
    "transactionIndex": "0x0"
  }
}
```

授予权限成功。

* 部署合约

由测试用户进行操作

```bash
$ cita-cli rpc sendRawTransaction \
    --code 0x608060405234801561001057600080fd5b5060df8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c146078575b600080fd5b348015605957600080fd5b5060766004803603810190808035906020019092919050505060a0565b005b348015608357600080fd5b50608a60aa565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a723058205aed214856a5c433292a354261c9eb88eed1396c83dabbe105bde142e49838ac0029 \
    --private-key 0x3ef2627393529fed043c7dbfd9358a4ae47a88a59949b07e7631722fd6959002 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x8bca970a8836f291ca86d33beccb147c3d7b04b361589d41bd928db683d731aa",
    "status": "OK"
  }
}
```

获取 receipt 信息：

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x8bca970a8836f291ca86d33beccb147c3d7b04b361589d41bd928db683d731aa \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x8cf225903eb7c49b0494f991941dcb4d401b2c51c321defa931914fb8f0aa87b",
    "blockNumber": "0xf2",
    "contractAddress": "0x5839153e0efe76efe0c974b728c4f49ca7ed75cc",
    "cumulativeGasUsed": "0xaef9",
    "errorMessage": null,
    "gasUsed": "0xaef9",
    "logs": [
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0x8bca970a8836f291ca86d33beccb147c3d7b04b361589d41bd928db683d731aa",
    "transactionIndex": "0x0"
  }
}
```

得到合约地址为 `0x5839153e0efe76efe0c974b728c4f49ca7ed75cc`

如果用户想要调用测试合约的接口，需要根据接口生成一个新的权限，然后由 admin 把权限赋予用户。

### 生成新的权限

由管理员进行操作

```bash
$ cita-cli scm PermissionManagement newPermission \
    --name 0000000000000000000000000000000000000000000000000000000060fe47b1 \
    --contracts '[5839153e0efe76efe0c974b728c4f49ca7ed75cc]' \
    --function-hashes '[60fe47b1]' \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x239fb9c6121d6c512e4b8e3422da378e6d329c4d5073fd7b83ee67d28cc89565",
    "status": "OK"
  }
}
```

获取 receipt 信息：

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0x239fb9c6121d6c512e4b8e3422da378e6d329c4d5073fd7b83ee67d28cc89565 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xea33ec74f7ce12fae0a6c5df4137d3234e120a8c6cfd2c557052184409729c98",
    "blockNumber": "0x282",
    "contractAddress": null,
    "cumulativeGasUsed": "0x2290fe",
    "errorMessage": null,
    "gasUsed": "0x2290fe",
    "logs": [
      {
        "address": "0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee",
        "blockHash": "0xea33ec74f7ce12fae0a6c5df4137d3234e120a8c6cfd2c557052184409729c98",
        "blockNumber": "0x282",
        "data": "0x0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000010000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc000000000000000000000000000000000000000000000000000000000000000160fe47b100000000000000000000000000000000000000000000000000000000",
        "logIndex": "0x0",
        "topics": [
          "0xb533e8b79dc7485ba7e4435e3395df911c1a3c767225941003d88a7812d216f7"
        ],
        "transactionHash": "0x239fb9c6121d6c512e4b8e3422da378e6d329c4d5073fd7b83ee67d28cc89565",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020005",
        "blockHash": "0xea33ec74f7ce12fae0a6c5df4137d3234e120a8c6cfd2c557052184409729c98",
        "blockNumber": "0x282",
        "data": "0x0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000010000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc000000000000000000000000000000000000000000000000000000000000000160fe47b100000000000000000000000000000000000000000000000000000000",
        "logIndex": "0x1",
        "topics": [
          "0x792f7322d94960c6e90863b5aef39075ca54620cfa13a822081d733f79c48f91",
          "0x000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee",
          "0x0000000000000000000000000000000000000000000000000000000060fe47b1"
        ],
        "transactionHash": "0x239fb9c6121d6c512e4b8e3422da378e6d329c4d5073fd7b83ee67d28cc89565",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      }
    ],
    "logsBloom": "0x00000000000000020000000000000000000000000000008000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000080000000000000000040000000000000000040000000000000000001000000000000000000000000000000000000000000000000008000000800000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000200000000000000000000000000000000000000000000000000000000000000000000100000000000000000000100000000000000000000000000000800000002000000000000100000000100000",
    "root": null,
    "transactionHash": "0x239fb9c6121d6c512e4b8e3422da378e6d329c4d5073fd7b83ee67d28cc89565",
    "transactionIndex": "0x0"
  }
}
```

从 logs[0] 中获得新权限的地址为 `0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee`

### 使用新权限

* 把新权限赋予测试用户

由管理员进行操作。

```bash
$ cita-cli scm PermissionManagement setAuthorization \
    --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0xc088f083c8ac7d89bd7056d09629f59c1f67cd6c97120807cee782c8200402e1",
    "status": "OK"
  }
}
```

获取 receipt 信息：

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0xc088f083c8ac7d89bd7056d09629f59c1f67cd6c97120807cee782c8200402e1 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xbaf441b79671f0333adf1511361d258dcc606b3ecc6c42ac04d7e31794e1726b",
    "blockNumber": "0x554",
    "contractAddress": null,
    "cumulativeGasUsed": "0x13587",
    "errorMessage": null,
    "gasUsed": "0x13587",
    "logs": [
      {
        "address": "0xffffffffffffffffffffffffffffffffff020006",
        "blockHash": "0xbaf441b79671f0333adf1511361d258dcc606b3ecc6c42ac04d7e31794e1726b",
        "blockNumber": "0x554",
        "data": "0x",
        "logIndex": "0x0",
        "topics": [
          "0xef79a70821e438468db437d5f7401aecaf406a2cba3c7e7fd4339ef895dbb97e",
          "0x00000000000000000000000037d1c7449bfe76fe9c445e626da06265e9377601",
          "0x000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee"
        ],
        "transactionHash": "0xc088f083c8ac7d89bd7056d09629f59c1f67cd6c97120807cee782c8200402e1",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      }
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400001000004200000000000000000000020000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000900080000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0xc088f083c8ac7d89bd7056d09629f59c1f67cd6c97120807cee782c8200402e1",
    "transactionIndex": "0x0"
  }
}
```

* 查询账户权限

### 调用测试合约

查询测试账号的权限：

```bash
$ cita-cli scm Authorization queryPermissions \
    --account 0x37d1c7449bfe76fe9c445e626da06265e9377601 \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003000000000000000000000000ffffffffffffffffffffffffffffffffff021000000000000000000000000000ffffffffffffffffffffffffffffffffff021001000000000000000000000000ca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee"
}
```

已经可以看到新添加的权限了。

### 调用测试合约

调用测试合约 set 方法，传入参数为 1 ：

```bash
$ cita-cli rpc sendRawTransaction \
    --code 0x60fe47b10000000000000000000000000000000000000000000000000000000000000001 \
    --private-key 0x3ef2627393529fed043c7dbfd9358a4ae47a88a59949b07e7631722fd6959002 \
    --address 0x5839153e0efe76efe0c974b728c4f49ca7ed75cc \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0xa9179ed4226e16c332fc0b70a136f4b7dec59b8dd964c22381e24a35e22d0d2b",
    "status": "OK"
  }
}
```

查看 receipt 信息：

```bash
$ cita-cli rpc getTransactionReceipt \
    --hash 0xa9179ed4226e16c332fc0b70a136f4b7dec59b8dd964c22381e24a35e22d0d2b \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x2984cd1ad2beaf267d3bff78e8dcb64bbf75bcc9721007d0f2a7c4a01ac68a1b",
    "blockNumber": "0x152e",
    "contractAddress": null,
    "cumulativeGasUsed": "0x4f51",
    "errorMessage": null,
    "gasUsed": "0x4f51",
    "logs": [
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "root": null,
    "transactionHash": "0xa9179ed4226e16c332fc0b70a136f4b7dec59b8dd964c22381e24a35e22d0d2b",
    "transactionIndex": "0x0"
  }
}
```

从 errMessage 中已经可以看出交易成功了。

通过调用 get 方法查询结果：

```bash
$ cita-cli rpc call \
    --to 0x5839153e0efe76efe0c974b728c4f49ca7ed75cc \
    --data 0x6d4ce63c \
    --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

可以看出结果已经是 1 了。

## 角色管理概述

在权限之上封装了一层更贴近于现实生活中的角色类型，角色包含多种权限。可对用户赋予角色，则用户拥有角色内的所有权限。

* 角色的增删改等相关操作独立于权限管理。操作需要权限管理赋予相应权限，不会造成权限管理的变动。
* 关于角色的授权操作： 授予角色时会调用权限管理的授权接口，所以会造成权限管理的变动。 ***建议角色的授权与权限的授权二者选其一，应该尽量避免同时使用***
* 关于角色的鉴权： 鉴权是在底层操作，底层没有角色的概念，鉴权与权限管理统一。

用户可自定义角色。
