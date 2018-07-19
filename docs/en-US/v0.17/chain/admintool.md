# config_tool的功能和用法

## 主要功能

create_cita_config.py 分为两种模式：

- create: 创建全新的一条链
- append: 在已有的链配置上新增一个节点

可通过运行如下命令查看：

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

optional arguments:
  -h, --help            show this help message and exit
  --authorities AUTHORITY[,AUTHORITY[,AUTHORITY[,AUTHORITY[, ...]]]]
                        Authorities (addresses) list.
  --chain_name CHAIN_NAME
                        Name of the new chain.
  --nodes IP:PORT[,IP:PORT[,IP:PORT[,IP:PORT[, ...]]]]
                        Node network addresses for new nodes.
  --super_admin SUPER_ADMIN
                        Address of super admin.
  --contract_arguments Contract.Argument=Value [Contract.Argument=Value ...]
                        Update constructor arguments for system contract. Can
                        be specify more than once.
  --timestamp TIMESTAMP
                        Specify a timestamp to use.
  --resource_dir RESOURCE_DIR
                        Chain resource directory.
  --grpc_port GRPC_PORT
                        grpc port for this chain
  --jsonrpc_port JSONRPC_PORT
                        jsonrpc port for this chain
  --ws_port WS_PORT     websocket port for this chain
```

注意：
1. 配置工具会创建以chain_name为名称的文件夹，如果没有传递该参数则默认为`test-chain`。该文件夹里面再按节点序号创建0，1，2等节点文件夹，分别存放每个节点的配置文件。
2. 为了方便测试时多个节点在同一台服务器上运行。
    grpc，jsonrpc，ws_port等参数指定的端口号是一个起始端口号。
    节点实际使用的端口号，按照节点排列顺序顺延，即port+n（n为节点序号）。
    比如总共4个节点，传递grpc_port参数为7000。则test-chain/0的grpc端口号为7000，test-chain/1的grpc端口号为7001，等等。
3. CITA有一些保留端口，设置节点网络端口，或者自定义端口的时候要避免产生端口冲突。保留端口有：
    - 默认的grpc端口：5000到5000+N（N为节点总数,以下相同）
    - 默认的jsonrpc端口：1337到1337+N
    - 默认的websocket端口：4337到4337+N
    - 默认的rabbitmq端口：4369(epmd)/25672(Erlang distribution)/5671，5672(AMQP)/15672(management plugin)

参数的简单解释：

- `chain_name` : cita 支持侧链后，通过 chain_name 生成不同的链配置，默认为 test-chain

- `super_admin` : 指定生成链的超级管理员地址

- `contract_arguments` : 设定系统合约的默认值，这个参数具体的信息请详细查看系统合约文档

	例如：

	- 链的 chain_id 值设为1，默认为随机数 `--contract_arguments "SysConfig.chainId=1"`
	- 链的经济模型设为 quota，默认为 quota(0), 其他选项有 charge(1) `--contract_arguments "SysConfig.economicalModel=0"`
      (***此模型下只有`super_admin`账户及节点地址有余额***)

## setup

默认创建 4 个共识节点的生成方式如下:

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
$ ls test-chain/
  0  1  2  3  template
```

相对应给出的参数，生成 4 个节点，`test-chain/*` 里面包含节点的配置文件，具体如下：

- 生成私钥和地址，私钥存放在`test-chain/*/privkey`，其中 `*` 为节点号；而所有节点地址都存放在`test-chain/template/authorities`；
- 生成网络配置文件，存放在`test-chain/*/network.toml`，文件内容主要为总节点数、本地节点端口以及其它节点的ip和端口号；
- 生成 genesis 块文件，存放在`test-chain/*/genesis.json`， 其中 timestamp 为时间戳，秒为单位；prevhash 指前一个块哈希，这里是默认值；而 alloc 指部署到创世块的合约内容；
- 生成节点配置文件，存放在`test-chain/*/consensus.toml`，主要包含共识算法的相关参数；
- 生成 jsonrpc 配置文件，存放在`test-chain/*/jsonrpc.toml`，主要包含 jsonrpc 模块的相关参数。
- `test-chain/template`目录下是模板文件，包括这个链的共识节点地址 `test-chain/template/authorities.list`，系统合约生成参数 `test-chain/template/init_data.yml`, 节点端口地址`test-chain/template/nodes.list` 等信息

增加 1 个节点，需要指定已经创建过的`chain_name`。举例如下：
```shell
$ ./env.sh ./scripts/create_cita_config.py append --chain_name test-chain --node "127.0.0.1:4004"
$ ls test-chain/
  0  1  2  3  4  template
```

注意：每次只能增加一个节点，且增加的为普通节点。如何把普通接点升级为共识节点，请参考`节点管理`。

### 配置文件

`config_tool/config_example` 目录下为各个微服务默认使用的配置文件。用户可在 `config_tool` 下创建格式类似的文件自定义配置信息。

#### cita-chain

`config_tool` 下创建`chain.toml`文件来自定义 `cita-chain` 配置。格式参考 `config_example/chain.toml` 文件，如下:

```shell
prooftype = 2
```
其中:

- `prooftype`: 表示当前使用的共识算法，0 表示采用的 Poa 算法、1 表示采用的 Raft 算法、2 表示采用的 Tendermint 算法，默认采用 Tendermint 算法。

#### cita-executor

`config_tool` 下创建`executor.toml`文件来自定义 `cita-executor` journaldb 的类型等。格式参考`config_example/executor.toml`文件，如下:

```shell
prooftype = 2
journaldb_type = "archive"
grpc_port = 5000
```

其中:

- `prooftype`: 表示当前使用的共识算法，0 表示采用的 Poa 算法、1 表示采用的 Raft 算法、2 表示采用的 Tendermint 算法，默认采用 Tendermint 算法。
- `journaldb_type`: 表示当前使用的 JournalDB 算法，有 "archive" "light" "fast" "basic" 等4种类型，默认是 archive。
- `grpc_port`: grpc端口

#### cita-auth

`config_tool` 下创建`auth.toml`文件来自定义 `cita-auth` 配置。格式参考`config_example/auth.toml`文件，如下:

```
count_per_batch = 30
buffer_duration = 30
tx_verify_thread_num = 4
tx_verify_cache_size = 100000
tx_pool_limit = 0
prof_start = 0
prof_duration = 0
```

其中：

- `count_per_batch`: 表示批量的数量

#### cita-bft

`config_tool` 下创建`consensus.toml`文件来自定义 `cita-bft` 配置。格式参考`config_example/consensus.toml`文件，如下:

```
[ntp_config]
enabled = true
threshold = 1000
address = "0.pool.ntp.org:123"
```

其中：

- `enabled`: 表示开启ntp
- `threshold`: 表示时间偏移的阀值
- `address`: 表示ntp服务器地址

## 系统合约

系统合约是从 genesis 块开始就部署到链上的用来实现特定功能的合约，它的合约地址写在genesis 块里，是固定的地址。

CITA 里主要的系统合约有配置合约、共识节点管理合约、配额管理合约、权限管理合约及用户管理合约等。

### 初始化系统合约说明

其中:

- `0xffffffffffffffffffffffffffffffffff020000`: 代表配置合约SysConfig。
- `0xffffffffffffffffffffffffffffffffff020001`: 代表共识节点管理系统合约地址。
- `0xffffffffffffffffffffffffffffffffff020003`: 代表配额管理系统合约地址。
- `0xffffffffffffffffffffffffffffffffff020002`: 代表链信息管理系统合约地址。
- `0xffffffffffffffffffffffffffffffffff020004`: 代表 CITA 权限管理合约地址。
- `0xffffffffffffffffffffffffffffffffff02000a`: 代表用户管理合约地址。

用户可使用系统默认数据，也可通过参数 `contract_arguments` 自定义配置。默认配置如下：

```
Contracts:
- SysConfig:
  - delayBlockNumber: 1
  - checkPermission: false
  - checkQuota: false
  - chainName: test-chain
  - chainId: 1
  - operator: test-operator
  - website: https://www.example.com
  - blockInterval: 3000
  - economicalModel: 0
- QuotaManager:
  - admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- NodeManager:
  - nodes:
    - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
  - admins:
    - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
  - stakes:
    - 0
- ChainManager:
  - parentChainId: 0
  - parentChainAuthorities: []
- Authorization:
  - superAdmin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- Group:
  - parent: '0x0000000000000000000000000000000000000000'
  - name: rootGroup
  - accounts:
    - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
```

默认使用的账户如下：

|                          privkey                                 |                   address                  |
|:----------------------------------------------------------------:|:------------------------------------------:|
| 5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 | 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523 |


### 节点管理系统合约

节点管理合约为`scripts/contracts/system/node_manager.sol`。

函数签名在`scripts/contracts/docs`目录提供了`NodeManager-hashes.json`可供查看，
并提供了针对用户和开发者的文档，分别为`NodeManager-userdoc.json`及`NodeManager-devdoc.json`

共识节点管理的相关描述及方法介绍见[node_manager](./system_management/node)

### 配额管理系统合约

配额管理合约存放在`scripts/contracts/system/quota_manager.sol`。

函数签名在`scripts/contracts/docs`目录下提供了`QuotaManager-hashes.json`可供查看，
并提供了针对用户和开发者的文档，分别为`QuotaManager-userdoc.json`及`QuotaManager-devdoc.json`

配额管理的相关描述及方法介绍见[quota_manager](./system_management/quota)

### 权限管理系统合约

函数签名权限管理合约存放在`scripts/contracts/permission_management`。

在`scripts/contracts/docs`目录下提供了`PermissionManagement-hashes.json`可供查看，
并提供了针对用户和开发者的文档，分别为`PermissionManagement-userdoc.json`及`PermissionManagement-devdoc.json`

详细的接口说明见[permission_management](./system_management/permission)

## 单独增加节点

相关描述及操作见[ordinary_node_management](./system_management/node)