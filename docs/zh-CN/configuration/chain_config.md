# 链级配置

当拿到发布件解压后，或从源码编译后，不要着急动节点，在这之前，很重要的一步就是我们需要对链进行初始化配置。
这些配置信息将被写入链的创世块，创世块一旦生成，SysConfig 中只有 `chainName`，`operator`，`website` 这三项可以在链运行之后再进行修改，其他项均不可再修改, 因此请大家慎重设定各配置项。
在 CITA 里面，我们提供了工具 config tool 来帮助你在起链前对链进行初始化配置, 提供了命令行工具 CITA-CLI 来帮助你在起链后修改个别配置。

本文档将为你详细介绍链的各个可配置项，包括链自身的一些属性、系统合约、RPC接口、节点间网络连接等；
然后通过具体的操作示例，演示如何起链前对链进行初始化配置；
并带你详细了解初始化配置后文件的目录结构；
最后，将通过具体示例，演示起链后如何修改个别配置。
相信阅读完此文档后，你将可以自己定制一条满足你需求的链。

## 可配置项

执行以下命令查看各个配置项：

```shell
$ ./env.sh ./scripts/create_cita_config.py create --help
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
                                    [--enable_tls]
```

我们一一解释：

> **Notice**
> 起链时，强制要求的配置项为 `super_admin` 和 `nodes`，系统不提供默认配置。

### `--authorities` 将共识节点的地址写到链上
* 安全起见，我们建议的流程是：先由每个共识节点单独生成各自的私钥和地址，私钥请务必由自己妥善保管；地址交由负责起链的管理员，通过该命令写到链上。起链后，生成的 `test-chain/*/privkey` 文件为空，由各节点独自将自己的私钥填写进来。
* 没有传递参数的话，默认会自动生成对应节点数量的私钥/地址对：地址写到链上；私钥存放在各个节点的 `test-chain/*/privkey` 文件里。

### `--chain_name` 指定链的名
* 执行该命令后会生成以链的名字为名称的文件夹，该文件夹里面再按节点序号创建 0，1，2 等节点文件夹，分别存放每个节点的配置文件。
* 如果没有传递 `chain_name` 参数，则默认链的名字为 `test-chain`。

### `--nodes` 指定节点的 ip 地址和端口
每个节点都需要提供 ip 和 port，ip 与 port 用冒号隔开，节点间用逗号隔开。这个参数有多少个网络地址，最终将会生成对应数量的节点(上限256)，并且相对应的节点序号就按照参数中网络地址的顺序，从 0 开始，往后递增。

### `--super_admin` 指定超级管理员地址
该账户拥有最高权限，用来管理整条链的运行状态。用户**必须**自己设置超级管理员。

### `--contract_arguments` 指定链自身的一些系统配置和系统合约的配置
`test-chain/template` 目录下的 `init_data.yml` 中记录了一种示例配置，我们可以看到各配置项如下：

```
-Contracts:
 -SysConfig:
   delayBlockNumber: 1
   checkCallPermission: false
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
   avatar: https://cdn.cryptape.com/icon_appchain.png
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
    - `checkCallPermission` : 合约调用权限检查开关
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
    - `economicalModel`： 经济模型。CITA 中存在两种经济模型，Quota(默认) 和 Charge。`economicalModel = 0` 表示 Quota 模型，交易只需不超过限额即可，限额由管理员设置，详细机制可查看[配额管理](./system_management/quota)；`economicalModel = 1` 表示 Charge 模型， 交易需要手续费，针对交易的每一步执行进行单步扣费模式，扣除余额，详细机制可查看[配额价格管理](./system_management/price)。
    - `name` : Token 名称
    - `symbol` : Token 符号
    - `avatar` : Token 图标链接
  * `QuotaManager` : 初始化配额管理合约的管理员地址
    - `admin` : 默认管理员地址
  * `NodeManager` : 初始化共识节点管理合约
    - `nodes` : 共识节点地址
    - `stakes` : 共识节点对应的出块权重
  * `ChainManager` : 初始化链的一些信息，用于跨链。
    - `parentChainId` : 父链 ID
    - `parentChainAuthorities` : 父链的共识节点列表
  * `Authorization` : 初始化权限管理合约
    - `superAdmin` : 管理员地址
  * `Group` : 初始化用户组管理合约
    - `parent` : 父组的地址
    - `name` : 组的名称
    - `accounts` : 组内用户列表
  * `Admin` : 管理员
    - `admin` : 管理员地址
  * `VersionManager` : 协议版本管理合约
    - `version` : 协议版本号

### `--time_stamp` 指定起链的时间戳
* 具体数值是指自 1970-1-1 以来的毫秒数，默认是取当前的时间，如果时间取在未来，则链起来之后不会出块。
* 这个值在生成的genesis.son文件中可以查看到。

### `--resource_dir` 指定资源目录
* 除了创世块中的数组，链有时候还需要额外自带一些数据（比如说零知识证明），但是因为数据比较大，无法放入创世块，因此在这里可以通过传递参数指定一个单独的资源目录。
* 指定该参数后，生成的配置会多一个 resource 目录，用户指定目录下的文件讲会被拷贝进来，然后，配置工具会计算该目录下所有文件的 hash 值，作为 genesis.json 中 prevhash 字段中的值。prevhash 默认全部是 0，通过传入此参数，prevhash 的值将发生改变。

### `--grpc_port`、`jsonrpc_port`、`ws_port` 指定起始端口号
* grpc，jsonrpc，ws_port 等参数指定的端口号是一个起始端口号。节点实际使用的端口号，按照节点排列顺序顺延，即 port+n（ n 为节点序号）。比如总共 4 个节点，传递 grpc_port 参数为 7000 ，则 test-chain/0 的 grpc 端口号为 7000，test-chain/1 的 grpc 端口号为 7001，以此类推。
* grpc_port 存在 `test-chain/*/executor.toml` 中，jsonrpc port 和 ws port 都存在 `test-chain/*/jsonrpc.toml` 中 。
* CITA有一些保留端口，设置节点网络端口，或者自定义端口的时候要避免产生端口冲突。保留端口有：
  * 默认的 `grpc` 端口：5000 到 5000 + N（N 为节点总数,以下相同）
  * 默认的 `jsonrpc` 端口：1337 到 1337 + N
  * 默认的 `websocket` 端口：4337 到 4337+N
  * 默认的 `rabbitmq` 端口：4369(epmd)/25672(Erlang distribution)/5671，5672(AMQP)/15672(management plugin)

###  `--enable_tls` 是否开通节点间通讯加密
* 指定节点间数据是否使用 TLS (Transport Layer Security) 加密传输，不加此选项默认为不加密传输。
* 创建链时加上此选项，会在 `test-chain/*/network.toml` 配置文件中增加 `enable = true` 和每个 peer 中 `common_name = ${chain_name}.cita` 的配置项。

## 初始化配置操作示例

以下是最基础起链命令，该命令生成一条包含四个节点的新链，端口默认 4000 , 4001 , 4002 , 4003， 默认超级管理员，经济模型为 `Quota`, 所有权限控制关闭。

```shell
$ ./env.sh ./scripts/create_cita_config.py create --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
```

接下来演示来生成一条高级配置的链, 命令如下：

```shell
$ ./env.sh ./scripts/create_cita_config.py create --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" --contract_arguments SysConfig.checkSendTxPermission=true SysConfig.checkCallPermission=true SysConfig.economicalModel=1 SysConfig.checkFeeBackPlatform=true SysConfig.chainOwner=0x9a6bd7272edb238f13002911d8c93dd6bb646d15
```

上述命令，生成一条包含四个节点，端口默认 4000 , 4001 , 4002 , 4003， 超级管理员地址 `0xab159a4817542585c93f01cfce9cfe6cd4cbd26a`， 运营方地址
`0x9a6bd7272edb238f13002911d8c93dd6bb646d15`， 经济模型 `Charge`， 出块激励返回运营方，权限全开的链。

## 初始化配置后生成的目录结构

采用 `create` 默认创建 4 个共识节点的目录结构如下:

```bash
$ ls test-chain/
0 1 2 3 template
$ ls 0
address     consensus.toml  forever.toml       logs
auth.toml   data            genesis.json       network.toml
chain.toml  executor.toml   jsonrpc.toml       privkey
```

相对应给出的参数，生成 4 个节点，`test-chain/*` 里面包含节点的配置文件，具体如下：

* `privkey` : 存放私钥
* `address` : 存放地址
* `*.toml` :  各个微服务配置文件，详细说明见微服务说明
* `genesis.json` ： 生成 genesis 块文件， 其中 timestamp 为时间戳，秒为单位；prevhash 指前一个块哈希，这里是默认值；而 alloc 指部署到创世块的合约内容；
* `test-chain/template` 目录下是模板文件，包括这个链的共识节点地址 `test-chain/template/authorities.list`，系统参数 `test-chain/template/init_data.yml`, 节点端口地址 `test-chain/template/nodes.list` 等信息
* `logs` : 记录链运行的日志信息
* `data` : 数据存储

## 修改个别配置操作示例

起链后，也就是说创世块一旦生成，除 chainName, operator, website 三项可以在运行时更改，其他配置均无法修改。接下来我们用 [cita-cli](https://github.com/cryptape/cita-cli) 来进行演示，以管理员修改 `chainName` 作为示例：

确保你的链正常运行，进入 cita-cli 交互式模式，输入命令：

```shell
$ scm SysConfig setChainName --chain-name "AAA" --admin-private \ 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

查询交易回执无误后，我们成功的把链名称从默认的 `test-chain` 更改为 `AAA`。

我们可以通过 `getMeta` 查询更改后的结果，示例如下：

```shell
$ rpc getMetaData
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockInterval": 3000,
    "chainId": 1,
    "chainName": "AAA",
    "economicalModel": 1,
    "genesisTimestamp": 1538101178583,
    "operator": "test-operator",
    "tokenAvatar": "https://cdn.cryptape.com/icon_appchain.png",
    "tokenName": "Nervos AppChain Test Token",
    "tokenSymbol": "NATT",
    "validators": [
      "0x185e7072f53574666cf8ed8ec080e09b7e39c98f"
    ],
    "version": 1,
    "website": "https://www.example.com"
  }
}

```
`chainName` 已更新。
