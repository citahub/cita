# admintool的功能和用法

## install requirements

```shell
pip install -r requirements.txt
```

## 主要功能

可通过运行如下命令查看：

```shell
cd target/install
./bin/admintool.sh --help
```

结果如下：

```shell
usage: ./admintool.sh -a admin_id -l ip_list -n consensus_name -m crypto_method -d block_duration -t
option:
-a admin_id    admin identifier
    default value is 'admin'

-l ip_list     list all the node's IP and port
    default value is '127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003'

-n consensus_name  name of consensus algorithm
    default value is 'cita-bft', other is 'raft' and 'poa'

-m crypto_method    name of crypto algorithm
    default value is 'SECP'

-d block_duration    block generating duration(millisecond)
    default value is '3000'

-t            consensus test flag, only valid for cita-bft

-h enable jsonrpc http
   default enable 'true'

-w enable jsonrpc websocket
   default enable 'false'

-P define jsonrpc HTTP port or websocket port
   default port is '1337' or '4337'

-k start with kafka

-Q singel node id
```

当前默认初始配置为四个节点，如果需要在admintool.sh脚本里 **初始配置N个节点** ，可通过如下命令，比如配置五个节点：

```shell
./bin/admintool.sh -l "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003,127.0.0.1:4004"
```

## setup

```shell
./bin/admintool.sh
```

运行之后会生成`node*`以及backup备份文件夹．`node*` 里面包含节点文件以及相关的配置文件，具体如下：

- 生成私钥和地址，私钥存放在`node*/privkey`，其中nodeID为节点号；而所有节点地址都存放在`backup/authorities`；
- 生成网络配置文件，存放在`node*/network.toml`，文件内容主要为总节点数、本地节点端口以及其它节点的ip和端口号；
- 生成genesis块文件，存放在`node*/genesis.json`， 其中timestamp为时间戳，秒为单位；prevhash指前一个块哈希，这里是默认值；而alloc指部署到创世块的合约内容；
- 生成节点配置文件，存放在`node*/consensus.json`，主要包含共识算法的相关参数；
- 生成jsonrpc配置文件，存放在`node*/jsonrpc.toml`，主要包含jsonrpc模块的相关参数。
  backup文件下存放是用于增加单节点的备份信息，里面有authorities，genesis.json两个文件，其作用见下文［单独增加节点］

## 系统合约

系统合约是从genesis块开始就部署到链上的用来实现特定功能的合约，它的合约地址写在genesis块里，是固定的地址。CITA里主要的系统合约有节点管理合约、配额管理合约和权限管理合约等。

### 初始化系统合约说明

用户可选择自定义初始化系统合约数据及使用系统默认数据，其中release文件下的`init_data.json`为初始化系统合约数据文件。

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
        1,
        false,
        false
    ],
    "0x00000000000000000000000000000000013241b4": "0x9dcd6b234e2772c5451fd4ccf7582f4283140697",
    "0x00000000000000000000000000000000013241b5": {
        "0x00000000000000000000000000000000013241b5": [
            "newPermission",
            [
                "0x00000000000000000000000000000000013241b2"
            ],
            [
                "fc4a089c"
            ]
        ],
        "0x00000000000000000000000000000000023241b5": [
            "deletePermission",
            [
                "0x00000000000000000000000000000000013241b2"
            ],
            [
                "98a05bb1"
            ]
        ],
        "0x00000000000000000000000000000000033241b5": [
            "updatePermission",
            [
                "0x00000000000000000000000000000000013241b2",
                "0x00000000000000000000000000000000013241b2",
                "0x00000000000000000000000000000000013241b2"
            ],
            [
                "f036ed56",
                "6446ebd8",
                "537bf9a3"
            ]
        ],
        "0x00000000000000000000000000000000043241b5": [
            "setAuth",
            [
                "0x00000000000000000000000000000000013241b2",
                "0x00000000000000000000000000000000013241b2"
            ],
            [
                "0f5aa9f3",
                "52c5b4cc"
            ]
        ],
        "0x00000000000000000000000000000000053241b5": [
            "cancelAuth",
            [
                "0x00000000000000000000000000000000013241b2",
                "0x00000000000000000000000000000000013241b2",
                "0x00000000000000000000000000000000013241b2"
            ],
            [
                "3482e0c9",
                "a5925b5b",
                "ba00ab60"
            ]
        ],
        "0x00000000000000000000000000000000063241b5": [
            "newRole",
            [
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee"
            ],
            [
                "551ef860"
            ]
        ],
        "0x00000000000000000000000000000000073241b5": [
            "deleteRole",
            [
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee"
            ],
            [
                "54b025c5"
            ]
        ],
        "0x00000000000000000000000000000000083241b5": [
            "updateRole",
            [
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee",
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee",
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee"
            ],
            [
                "0773e6ba",
                "17b2e350",
                "d9c090a0"
            ]
        ],
        "0x00000000000000000000000000000000093241b5": [
            "setRole",
            [
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee"
            ],
            [
                "a32710eb"
            ]
        ],
        "0x000000000000000000000000000000000a3241b5": [
            "cancelRole",
            [
                "0xe3b5ddb80addb513b5c981e27bb030a86a8821ee"
            ],
            [
                "a8319481"
            ]
        ],
        "0x0000000000000000000000000000000000000001": [
            "sendTx",
            [
                "0x0000000000000000000000000000000000000001"
            ],
            [
                "00000000"
            ]
        ],
        "0x0000000000000000000000000000000000000002": [
            "createContract",
            [
                "0x0000000000000000000000000000000000000002"
            ],
            [
                "00000000"
            ]
        ]
    },
    "0x00000000000000000000000000000000013241b6": [
        "0x0000000000000000000000000000000000000000",
        "rootGroup",
        [
            "0x1a702a25c6bca72b67987968f0bfb3a3213c5688",
            "0x0dbd369a741319fa5107733e2c9db9929093e3c7",
            "0x9dcd6b234e2772c5451fd4ccf7582f4283140697"
        ]
    ]
}
```

其中:

- `0x00000000000000000000000000000000013241a2`: 代表共识节点管理系统合约地址，二维数组内`[]`内为节点地址列表，由系统生成，忽略此选项，用户可修改`["0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"]`值为自己生成的地址，
                                                其为管理员地址，
                                                可由此地址进行共识节点的增删。 ***须保存好对应的私钥***
- `0x00000000000000000000000000000000013241a3`: 代表配额管理系统合约地址，用户可修改`0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888`值为自己生成的地址，其为配额管理的管理员地址，
                                                可由此地址进行配额的管理。 ***须保存好对应的私钥***
- `0x0000000000000000000000000000000031415926`: 代表只读配置合约，三个参数分别代表系统合约生效需要的块数，默认为1代表下一个块生效；权限检查的开关，默认关闭；配额检查开关，默认为关闭。
- `0x00000000000000000000000000000000013241b4`: 代表新CITA权限管理合约地址，用户可修改`0x9dcd6b234e2772c5451fd4ccf7582f4283140697`值为自己生成的地址，其为超级管理员地址，
                                                此地址拥有权限管理本身的所有权限。 ***须保存好对应的私钥***
- `0x00000000000000000000000000000000013241b5`: 代表权限合约地址，其中的多个地址分别代表系统内置的权限类型。
- `0x00000000000000000000000000000000013241b6`: 代表用户管理合约地址，三个参数分别代表rootGroup的父Group、rootGroup的名称以及组内初始的用户地址。
                                                用户可填入多个组内用户地址。 ***须保存好对应的私钥***

### 使用默认的初始化数据

用户可使用系统默认的初始化数据，即`init_data_example.json`文件，地址及其对应的私钥如下表所示:

|                          privkey                                 |                   address                  |
|:----------------------------------------------------------------:|:------------------------------------------:|
| 5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 | 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523 |
| 61b760173f6d6b87726a28b93d7fcb4b4f842224921de8fa8e49b983a3388c03 | 0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888 |
| 993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9 | 0x9dcd6b234e2772c5451fd4ccf7582f4283140697 |

### 用户自定义检查配置文件

用户可在本目录下创建`chain.toml`文件来自定义发送交易时是否检查账户的permission等，默认是需要检查的。格式参考`chain_config_example.toml`文件，如下:

```shell
prooftype = 2
```

也可在本目录下创建`executor.toml`文件来自定义journaldb的类型等。格式参考`executor_config_example.toml`文件，如下:

```shell
prooftype = 2
journaldb_type = "archive"
```

其中:

- `prooftype`: 表示当前使用的共识算法，0表示采用的Poa算法、1表示采用的Raft算法、2表示采用的Tendermint算法，默认采用Tendermint算法。
- `journaldb_type`: 表示当前使用的JournalDB算法，有"archive" "light" "fast" "basic"等4种类型，默认是archive。

### 节点管理系统合约

节点管理合约存放在`install/scripts/contracts/node_manager.sol`，函数签名可通过`solc node_manager.sol --hashes`编译得到，也可以在[remix](https://remix.ethereum.org)上查看。

node_manager.sol合约详情如下所示：

```shell
contract address: 0x00000000000000000000000000000000013241a2
======= node_manager.sol:NodeManager =======
Function signatures: 
70480275: addAdmin(address)
dd4c97a0: approveNode(address)
2d4ede93: deleteNode(address)
30ccebb5: getStatus(address)
24d7806c: isAdmin(address)
609df32f: listNode()
ddad2ffe: newNode(address)
645b8b1b: status(address)
```

共识节点管理的相关描述及方法介绍见[node_manager](https://cryptape.github.io/cita/system_management/node/index.html#_6)

### 配额管理系统合约

配额管理合约存放在`install/scripts/contracts/system/quota_manager.sol`，合约函数hash详情如下所示：

```shell
contract address: 0x00000000000000000000000000000000013241a3
======= quota_manager.sol:QuotaManager =======
Function signatures: 
70480275: addAdmin(address)
942a8ad3: getAQL(address)
8a48ac03: getAccounts()
0bc8982f: getBQL()
bd9fbe7b: getDefaultAQL()
cdbcff6d: getQuotas()
24d7806c: isAdmin(address)
499a1bcd: setAQL(address,uint256)
931cd0cc: setBQL(uint256)
b107ea12: setDefaultAQL(uint256)
```

配额管理的相关描述及方法介绍见[quota_manager](https://cryptape.github.io/cita/system_management/quota/index.html)

### 单独增加节点

相关描述及操作见[普通节点管理](https://cryptape.github.io/cita/system_management/node/index.html#_2)

#### 权限管理系统合约

新权限系统合约存放在`install/scripts/contracts/permission_management`文件夹下，相关接口说明在`install/scripts/contracts/permission_management/README.md`，详细的接口说明见[permission_management](https://cryptape.github.io/cita/system_management/permission/index.html)
