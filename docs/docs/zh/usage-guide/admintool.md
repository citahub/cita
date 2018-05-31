# admintool的功能和用法

## 主要功能

create_cita_config.py 分为两种模式：

- create: 创建全新的一条链
- append: 在已有的链配置上新增一个节点，新增节点命令请查看节点管理章节

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

参数的简单解释：

- `chain_name` : cita 支持侧链后，通过 chain_name 生成不同的链配置，默认为 node

- `super_admin` : 指定生成链的超级管理员地址

- `contract_arguments` : 设定系统合约的默认值，这个参数具体的信息请详细查看系统合约文档

	例如：

	- 链的 chain_id 值设为1，默认为随机数 `--contract_arguments "SysConfig.chain_id=1"`
	- 链的经济模型设为 quota，默认为 quota(0), 其他选项有 charge(1) `—contract_arguments "SysConfig.economical_model=0"`



## setup

默认 4 个节点的生成方式如下:

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
$ ls node/
  0  1  2  3  template
```

相对应给出的参数，生成 4 个节点，`node/*` 里面包含节点的配置文件，具体如下：

- 生成私钥和地址，私钥存放在`node/*/privkey`，其中 nodeID 为节点号；而所有节点地址都存放在`backup/authorities`；
- 生成网络配置文件，存放在`node/*/network.toml`，文件内容主要为总节点数、本地节点端口以及其它节点的ip和端口号；
- 生成 genesis 块文件，存放在`node/*/genesis.json`， 其中 timestamp 为时间戳，秒为单位；prevhash 指前一个块哈希，这里是默认值；而 alloc 指部署到创世块的合约内容；
- 生成节点配置文件，存放在`node/*/consensus.json`，主要包含共识算法的相关参数；
- 生成 jsonrpc 配置文件，存放在`node/*/jsonrpc.toml`，主要包含 jsonrpc 模块的相关参数。
- `node/template`目录下是模板文件，包括这个链的共识节点地址 `node/template/authorities.list`，系统合约生成参数 `node/template/init_data.yml`, 节点端口地址`node/template/nodes.list` 等信息

## 系统合约

系统合约是从 genesis 块开始就部署到链上的用来实现特定功能的合约，它的合约地址写在genesis 块里，是固定的地址。CITA 里主要的系统合约有节点管理合约、配额管理合约、权限管理合约及用户管理合约等。

### 初始化系统合约说明

用户可选择自定义初始化系统合约数据及使用系统默认数据，其中 release 文件下的`init_data.json`为初始化系统合约数据文件。

### 用户自定义初始化系统合约数据

用户可在本目录下创建`init_data.json`文件来自定义系统合约的初始化数据。格式参考`init_data_example.json`文件，如下:

```json
{
    "0x00000000000000000000000000000000013241a2": [
        [],
        [
            "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"
        ]
    ],
    "0x00000000000000000000000000000000013241a3": "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888",
    "0x0000000000000000000000000000000031415926": [
        1, // 系统合约生效需要的块数
        false, // 权限检查的开关
        false, // 配额检查开关
        "test-chain", // chain name
        0, // chain id
        "test-operator", // chain operator
        "https://www.example.com", // website
        3000, // block interval
        0 // 交易收费模式，0代表配额模式，1代表使用Gas计费
    ],
    "0x00000000000000000000000000000000013241b4": "0x9dcd6b234e2772c5451fd4ccf7582f4283140697",
    "0x00000000000000000000000000000000013241b5": {},
    "0x00000000000000000000000000000000013241b6": [
        "0x0000000000000000000000000000000000000000",
        "rootGroup",
        [
            "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523",
        ]
    ]
}
```

其中:

- `0x00000000000000000000000000000000013241a2`: 代表共识节点管理系统合约地址，二维数组内`[]`内为节点地址列表，由系统生成，忽略此选项，用户可修改`["0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"]`值为自己生成的地址，其为管理员地址，可由此地址进行共识节点的增删。 ***须保存好对应的私钥***
- `0x00000000000000000000000000000000013241a3`: 代表配额管理系统合约地址，用户可修改`0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888`值为自己生成的地址，其为配额管理的管理员地址，可由此地址进行配额的管理。 ***须保存好对应的私钥***
- `0x0000000000000000000000000000000031415926`: 代表配置合约SysConfig。
- `0x00000000000000000000000000000000013241b4`: 代表 CITA 权限管理合约地址，用户可修改`0x9dcd6b234e2772c5451fd4ccf7582f4283140697`值为自己生成的地址，其为超级管理员地址，此地址拥有权限管理本身的所有权限。 ***须保存好对应的私钥***
- `0x00000000000000000000000000000000013241b5`: 代表权限合约地址，其中的多个地址分别代表系统内置的权限类型。
- `0x00000000000000000000000000000000013241b6`: 代表用户管理合约地址，三个参数分别代表 rootGroup 的父 Group、rootGroup 的名称以及组内初始的用户地址。用户可填入多个组内用户地址。 ***须保存好对应的私钥***

### 使用默认的初始化数据

用户可使用系统默认的初始化数据，即`init_data_example.json`文件，地址及其对应的私钥如下表所示:

|                          privkey                                 |                   address                  |
|:----------------------------------------------------------------:|:------------------------------------------:|
| 5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 | 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523 |

### 用户自定义检查配置文件

用户可在本目录下创建`chain.toml`文件来自定义发送交易时是否检查账户的permission等，默认是需要检查的。格式参考`chain_config_example.toml`文件，如下:

```shell
prooftype = 2
```

也可在本目录下创建`executor.toml`文件来自定义 journaldb 的类型等。格式参考`executor_config_example.toml`文件，如下:

```shell
prooftype = 2
journaldb_type = "archive"
```

其中:

- `prooftype`: 表示当前使用的共识算法，0 表示采用的 Poa 算法、1 表示采用的 Raft 算法、2 表示采用的 Tendermint 算法，默认采用 Tendermint 算法。
- `journaldb_type`: 表示当前使用的 JournalDB 算法，有 "archive" "light" "fast" "basic" 等4种类型，默认是 archive。

### 节点管理系统合约

节点管理合约存放在`scripts/contracts/system/node_manager.sol`。

在`scripts/contracts/docs`目录提供了`NodeManager-hashes.json`可供查看，并提供了针对用户和开发者的文档，分别为`NodeManager-userdoc.json`及`NodeManager-devdoc.json`

共识节点管理的相关描述及方法介绍见[node_manager](https://cryptape.github.io/cita/zh/system_management/node/index.html#_6)

### 配额管理系统合约

配额管理合约存放在`scripts/contracts/system/quota_manager.sol`。

在`scripts/contracts/docs`目录下提供了`QuotaManager-hashes.json`可供查看，并提供了针对用户和开发者的文档，分别为`QuotaManager-userdoc.json`及`QuotaManager-devdoc.json`

配额管理的相关描述及方法介绍见[quota_manager](https://cryptape.github.io/cita/zh/system_management/quota/index.html)

### 权限管理系统合约

权限管理合约存放在`scripts/contracts/permission_management`。

在`scripts/contracts/docs`目录下提供了`PermissionManagement-hashes.json`可供查看，并提供了针对用户和开发者的文档，分别为`PermissionManagement-userdoc.json`及`PermissionManagement-devdoc.json`

详细的接口说明见[permission_management](https://cryptape.github.io/cita/zh/system_management/permission/index.html#_3)

## 单独增加节点

相关描述及操作见[ordinary_node_management](https://cryptape.github.io/cita/zh/system_management/node/index.html#_2)
