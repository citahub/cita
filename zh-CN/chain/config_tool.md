# 链的配置

按照快速搭链的教程编译成功后，我们可以在启动节点前，对链进行配置。本文档主要介绍如何配置链自身的一些属性、RPC接口、节点间网络连接，并通过具体的操作示例，教你如何搭建你自己的链。

## 配置项

按照快速搭链的教程编译成功后，`test-chain/template` 目录下的 `init_data.yml` 中记录了链的配置项内容如下， 下面我们一一解释：

```
-Contracts:
 -SysConfig:
   delayBlockNumber: 1
   checkPermission: false
   checkSendTxPermission: false
   checkCreateContractPermission: false
   checkQuota: false
   checkFeeBackPlatform:false
   chainOwner: '0x0000000000000000000000000000000000000000'
   chainName: test-chain
   chainId: 1
   operator: test-operator
   website: https://www.example.com (https://www.example.com/)
   blockInterval: 3000
   economicalModel: 0
   name: Nervos AppChain Test Token
   symbol: NATT
   avatar: https://avatars1.githubusercontent.com/u/35361817
 -QuotaManager:
   admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
 -NodeManager:
   nodes:
   '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
   stakes:
   0
 -ChainManager:
   parentChainId: 0
   parentChainAuthorities: []
 -Authorization:
   superAdmin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
 -Group:
   parent: '0x0000000000000000000000000000000000000000'
   name: rootGroup
   accounts:
   '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
 -Admin:
   admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
 -VersionManager:
   version: 1
```

* `SysConfig` : 初始化一些系统信息
    - `delayBlockNumber` : 表示系统合约在几个块之后生效，默认为 1 个块。当前此功能已废弃。
    - `checkPermission` : 合约调用权限检查开关
    - `checkSendTxPermission` : 发送交易权限检查开关
    - `checkCreateContractPermission` : 创建合约权限检查开关
    - `checkQuota` : 配额检查开关
    - `checkFeeBackPlatform` : 出块激励选择开关，默认为 false，表示返回给共识节点，为 true 时返回给运营方地址( chainOwner )
    - `chainOwner` : 运营方地址，结合 checkFeeBackPlatform 一块使用
    - `chainName` : 链的名字
    - `chainId` : 链 Id
    - `operator` : 运营方名称
    - `website` : 运营方网站
    - `blockInterval` ： 出块间隔，默认 3 秒
    - `economicalModel`： 经济模型(稍后会做详细介绍)
    - `name` : Token 名称
    - `symbol` : Token 符号
    - `avatar` : Token 图标链接
* `QuotaManager` : 初始化管理员地址
    - `admin` : 默认管理员地址
* `NodeManager` : 初始化共识节点
    - `nodes` : 共识节点地址
    - `stakes` : 共识节点对应的出块权重
* `ChainManager` : 初始化链的一些信息，用于跨链。
    - `parentChainId` : 父链 ID
    - `parentChainAuthorities` : 父链的共识节点列表
* `Authorization` : 初始化管理员地址
    - `superAdmin` : 管理员地址
* `Group` : 初始化用户组
    - `parent` : 父组的地址
    - `name` : 组的名称
    - `accounts` : 组内用户列表
* `Admin` : 管理员
    - `admin` : 管理员地址
* `VersionManager` : 协议版本号
    - `version` : 协议版本号

创世块一旦生成，只有 `chainName`，`operator`，`website` 这三项可以在链运行之后再进行修改，其他项均不可再修改。

## 经济模型选择

CITA 中存在两种经济模型，Quota(默认) 和 Charge。

`economicalModel = 0` 表示 Quota 模型, 交易不需要手续费，quota 免费设置， 无余额概念，交易不扣除余额。

`economicalModel = 1` 表示 Charge 模型， 交易需要手续费，针对交易的每一步执行进行单步扣费模式，扣除余额。

## 配置工具

在 `docker` 环境下，我们使用 `./script/create_cita_config.py` 来构建一条链， 有两种模式：

* create: 创建全新的一条链
* append: 为已运行链新增一个节点

可通过运行命令 `./env.sh scripts/create_cita_config.py -h` 了解更多。

### Create 配置

```shell
$ ./env.sh ./script/create_cita_config.py create --help

usage: create_cita_config.py create [-h]
                                    [--authorities AUTHORITY[,AUTHORITY[,AUTHORITY[,AUTHORITY[, ...]]]]]
                                    [--chain_name CHAIN_NAME]
                                    [--nodes IP:PORT[,IP:PORT[,IP:PORT[,IP:PORT[, ...]]]]]
                                    [--super_admin SUPER_ADMIN]
                                    [--contract_arguments Contract.Argument=Value [Contract.Argument=Value ...]]
                                    [--timestamp TIMESTAMP]
                                    [--resource_dir RESOURCE_DIR]
                                    [--grpc_port GRPC_PORT]
                                    [--jsonrpc_port JSONRPC_PORT]
                                    [--ws_port WS_PORT]
```

必要参数解释：

* `chain_name` : 指定链的名字，cita 支持侧链后，通过 chain_name 生成不同的链配置，默认为 test-chain
* `nodes` : 指定节点的 ip 地址和端口
* `super_admin` : 指定超级管理员地址
* `contract_arguments` : 设定系统合约的默认值，这个参数具体的信息请详细查看系统合约文档

注意事项：

1. 配置工具会创建以 `chain_name` 为名称的文件夹，如果没有传递该参数则默认为 `test-chain` 。该文件夹里面再按节点序号创建 0，1，2 等节点文件夹，分别存放每个节点的配置文件。
2. 为了方便测试时多个节点在同一台服务器上运行。
    grpc，jsonrpc，ws_port 等参数指定的端口号是一个起始端口号。
    节点实际使用的端口号，按照节点排列顺序顺延，即 port+n（ n 为节点序号）。
    比如总共 4 个节点，传递 grpc_port 参数为 7000 。则 test-chain/0 的 grpc 端口号为 7000，test-chain/1 的 grpc 端口号为 7001 等等。
3. CITA有一些保留端口，设置节点网络端口，或者自定义端口的时候要避免产生端口冲突。保留端口有：
    * 默认的 `grpc` 端口：5000 到 5000 + N（N 为节点总数,以下相同）
    * 默认的 `jsonrpc` 端口：1337 到 1337 + N
    * 默认的 `websocket` 端口：4337 到 4337+N
    * 默认的 `rabbitmq` 端口：4369(epmd)/25672(Erlang distribution)/5671，5672(AMQP)/15672(management plugin)

### 操作示例

以下是最基础起链命令，该命令生成一条包含四个节点的新链，端口默认 4000 , 4001 , 4002 , 4003， 默认超级管理员，经济模型为 `Quota`, 所有权限控制关闭。

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
```

接下来演示来生成一条高级配置的链, 命令如下：

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" --contract_arguments SysConfig.checkSendTxPermission=true SysConfig.checkPermission=true SysConfig.economicalModel=1 SysConfig.checkFeeBackPlatform=true  SysConfig.chainOwner=0x9a6bd7272edb238f13002911d8c93dd6bb646d15 SysConfig.super_admin=0xab159a4817542585c93f01cfce9cfe6cd4cbd26a
```

上述命令，生成一条包含四个节点，端口默认 4000 , 4001 , 4002 , 4003， 超级管理员地址 `0xab159a4817542585c93f01cfce9cfe6cd4cbd26a`， 运营方地址
`0x9a6bd7272edb238f13002911d8c93dd6bb646d15`， 经济模型 `Charge`， 出块激励返回运营方，权限全开的链。

### 配置超级管理员帐户地址

```shell
$ ./env.sh ./scripts/create_cita_config.py create --super_admin=0xab159a4817542585c93f01cfce9cfe6cd4cbd26a ...
```

上述命令行参数中的 `--super_admin` 参数，用于设置超级管理员账户地址，该账户拥有最高权限，用来管理整条链的运行状态。在使用的时候，为安全起见，用户**应该/必须**自行设置超级管理员地址。

对于测试场合，CITA 配置了一个默认管理员账户地址（及其对应的私钥，只针对 secp256k1_sha3 版本）:

```json
{
  address: 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523
  private-key: 5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
}
```

### Append 配置

演示如何增加 1 个节点，我们需要指定已经创建过的 `chain_name`， 命令如下：

```shell
$ ./env.sh ./scripts/create_cita_config.py append --chain_name test-chain --node "127.0.0.1:4004"
$ ls test-chain/
  0  1  2  3  4  template
```

注意：每次只能增加一个节点，且增加的为普通节点。如何把普通接点升级为共识节点，请参考 `节点管理`。

## 链目录结构

采用 `create` 默认创建 4 个共识节点的目录结构如下:

```bash
$ ls test—chain/
  0  1  2  3  template
$ ls 0
  auth.toml executor.toml jsonrpc.toml chain.toml forever.toml logs
  consensus.toml network.toml genesis.json privkey data
```

相对应给出的参数，生成 4 个节点，`test-chain/*` 里面包含节点的配置文件，具体如下：

* `privkey` : 存放私钥
* `*.toml` :  各个微服务配置文件，详细说明见微服务说明
* `genesis.json` ： 生成 genesis 块文件， 其中 timestamp 为时间戳，秒为单位；prevhash 指前一个块哈希，这里是默认值；而 alloc 指部署到创世块的合约内容；
* `test-chain/template` 目录下是模板文件，包括这个链的共识节点地址 `test-chain/template/authorities.list`，系统合约生成参数 `test-chain/template/init_data.yml`, 节点端口地址 `test-chain/template/nodes.list` 等信息
* `logs` : 记录链运行的日志信息
* `data` : 数据存储
