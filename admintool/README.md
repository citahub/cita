# admintool的功能和用法

## install requirements

```
$ pip install -r requirements.txt
```

## 主要功能
可通过运行如下命令查看：
```
./admintool.sh --help
```

结果如下：
```
usage: ./admintool.sh -a admin_id -l ip_list -n consensus_name -m crypto_method -d block_duration -t -b block_tx_limit -f tx_filter_size
option:
-a admin_id    admin identifier
    default value is 'admin'

-l ip_list     list all the node's IP and port
    default value is '127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003'

-n consensus_name  name of consensus algorithm
    default value is 'tendermint', other is 'raft' and 'poa'

-m crypto_method    name of crypto algorithm
    default value is 'SECP'

-d block_duration    block generating duration(millisecond)
    default value is '3000'

-t            consensus test flag, only valid for tendermint

-b block_tx_limit    the limit of tx count in one block
    default value is '300'

-f tx_filter_size    the range of hisory tx to check duplication
    default value is '100000'

-c tx_pool_size    flow control for tx pool
    default value is '0'

-h enable jsonrpc http
   default enable 'true'

-w enable jsonrpc websocket
   default enable 'false'

-P define jsonrpc HTTP port or websocket port
   default port is '1337' or '4337'
```

当前默认初始配置为四个节点，如果需要在admintool.sh脚本里**初始配置N个节点**，可通过如下命令，比如配置五个节点：
```
./admintool.sh -l "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003,127.0.0.1:4004"
```

## setup

```
$ ./admintool.sh
```

  运行之后会生成`release`文件夹，里面包含节点文件以及相关的配置文件，具体如下：
- 生成私钥和地址，私钥存放在`admintool/release/nodeID/privkey`，其中nodeID为节点号；而所有节点地址都存放在`admintool/release/authorities`；
- 生成网络配置文件，存放在`admintool/release/nodeID/network.toml`，文件内容主要为总节点数、本地节点端口以及其它节点的ip和端口号；
- 生成genesis块文件，存放`在admintool/release/nodeID/genesis.json`， 其中timestamp为时间戳，秒为单位；prevhash指前一个块哈希，这里是默认值；而alloc指部署到创世块的合约内容；
- 生成节点配置文件，存放在`admintool/release/nodeID/consensus.json`，主要包含共识算法的相关参数；
- 生成jsonrpc配置文件，存放在`admintool/release/nodeID/jsonrpc.json`，主要包含jsonrpc模块的相关参数。

## 系统合约

系统合约是从genesis块开始就部署到链上的用来实现特定功能的合约，它的合约地址写在genesis块里，是固定的地址。CITA里主要的系统合约有节点管理合约、配额管理合约和权限管理合约等。

### 初始化系统合约说明

用户可选择自定义初始化系统合约数据及使用系统默认数据，其中release文件下的`init_data.json`为初始化系统合约数据文件。

#### 用户自定义初始化系统合约数据

用户可在本目录下创建`init_data.json`文件来自定义系统合约的初始化数据。格式参考`init_data_example.json`文件，如下:

```json
{
    "0x00000000000000000000000000000000013241a2": [],
    "0x00000000000000000000000000000000013241a3": "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888",
    "0x00000000000000000000000000000000013241a4": [["0x1a702a25c6bca72b67987968f0bfb3a3213c5688"], ["0x0dbd369a741319fa5107733e2c9db9929093e3c7"]]
}
```

其中:

* `0x00000000000000000000000000000000013241a2`: 代表共识节点管理系统合约地址，其节点地址列表由系统生成，忽略此选项。
* `0x00000000000000000000000000000000013241a3`: 代表配额管理系统合约地址，用户可修改`0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888`值为自己生成的地址，其为配额管理的管理员地址，
                                                可由此地址进行配额的管理。 ***须保存好对应的私钥***
* `0x00000000000000000000000000000000013241a4`: 代表权限管理系统合约地址，第一个数组为拥有发送交易权限的地址列表，第二个数组为拥有创建合约权限的地址列表。
                                                用户可分别填入多个地址。 ***须保存好对应的私钥***

#### 使用默认的初始化数据

用户可使用系统默认的初始化数据，即`init_data_example.json`文件，地址及其对应的私钥如下表所示:

|                          privkey                                 |                   address                  |
|:----------------------------------------------------------------:|:------------------------------------------:|
| 61b760173f6d6b87726a28b93d7fcb4b4f842224921de8fa8e49b983a3388c03 | 0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888 |
| 866c936ff332228948bdefc15b1877c88e0effce703ee6de898cffcafe9bbe25 | 0x1a702a25c6bca72b67987968f0bfb3a3213c5688 |
| 352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214 | 0x0dbd369a741319fa5107733e2c9db9929093e3c7 |


### 节点管理系统合约

节点管理合约存放在`cita/contracts/node_manager.sol`，函数签名可通过`solc node_manager.sol --hashes`编译得到，也可以在[remix](https://remix.ethereum.org)上查看.
node_manager.sol合约详情如下所示：
```
contract address: 0x00000000000000000000000000000000013241a2
Function signatures:
    dd4c97a0: approveNode(address)
    2d4ede93: deleteNode(address)
    30ccebb5: getStatus(address)
    609df32f: listNode()
    ddad2ffe: newNode(address)
    645b8b1b: status(address)
```
节点管理合约的目的是为了能够动态增删节点，即在已经运行的一些节点中增加或删除节点，这可以通过调用合约中的方法实现。
合约中节点有三种状态：Close，Ready，Start，初始默认为Close，可以通过调用合约里的函数来改变节点状态。
比如，增加节点，申请者首先调用newNode()方法，审批者(共识节点)调用approveNode来同意该节点成为共识节点。下面介绍下合约中主要的几种方法：

- `newNode(address)`，该方法功能是申请成为共识节点，其中参数address表示申请节点的地址，申请者通过cita_sendTransaction调用合约上的该方法，此时节点状态变更为Ready；

- `approveNode(address)`，该方法功能是同意其成为共识节点，其中参数address表示状态为Ready的节点地址，审批者(共识节点)通过cita_sendTransaction
调用该方法来同意节点状态为Ready的节点加入共识，此时节点状态变更为Start；

- `deleteNode(address)`， 该方法功能是删除共识节点，其中参数address表示状态Start的节点地址，状态为Start的节点可通过cita_sendTransaction调用合约上的该方法来退出共识，
此时节点状态变更为Close。


### 配额管理系统合约

配额管理合约存放在`cita/contracts/quota.sol`，合约详情如下所示：

```
contract address: 0x00000000000000000000000000000000013241a3
Function signatures:
70480275: addAdmin(address)
54f6127f: getData(bytes32)
6cf72948: getSpecialUsers()
7a490f7e: getUsersQuota()
24d7806c: isAdmin(address)
dfa87425: setAccountGasLimit(address,uint256)
a69257f3: setBlockGasLimit(uint256)
eb93eddf: setGlobal(bytes32,bytes32)
c9bcec77: setGlobalAccountGasLimit(uint256)
748ba8dd: setIsGlobal(bytes32,bool)
50f2ee97: setSpecial(address,bytes32,bytes32)
```

配额管理合约为每个block和account设置gasLimit，其中block中的gasLimit有效地控制该区块中的交易数量，account中的gasLimit有效地控制该用户在当前区块中发送的交易数量，
显然，account中的gasLimit是小于等于block中的gasLimit。用户分为specialUser和globalUser，其中specialUser的gasLimit默认值等于block的gasLimit，即只要不达到区块的gasLimit，
可以任意发送交易。而globalUser的gasLimit默认值要远小于block的gasLimit。另外，还有一类管理员账户，可设置block和account的gasLimit。
合约的主要方法如下：

- `addAdmin(address)`，该方法为添加管理账户，只有发送交易的用户为管理员才能成功添加其他用户为管理员。

- `setAccountGasLimit(address,uint256)`，该方法为设置其他用户的gasLimit，只有身份为管理员的地址才可以成功调用。

- `setBlockGasLimit(uint256)`，该方法为设置区块的gasLimit，只有身份为管理员的地址才可以成功调用。

- `getData(bytes32)`，该方法为查询用户或区块的gasLimit，所有地址都可以通过cita_sendTransaction成功调用此方法。

- `getSpecialUsers()`，该方法为查询所有specical用户，所有地址都可以通过cita_sendTransaction成功调用此方法。

- `getUsersQuota()`，该方法为查询special用户对应的gasLimit，即配额，所有地址都可以通过cita_sendTransaction成功调用此方法。


### 权限管理系统合约

权限管理合约存放在`cita/contracts/permission_manager.sol`，该合约将权限管理引进系统，有效控制用户交易的权限，合约详情如下所示：
```
contract address: 0x00000000000000000000000000000000013241a4
Function signatures:
    301da870: grantPermission(address,uint8)
    54ad6352: queryPermission(address)
    6f4eaf7a: queryUsersOfPermission(uint8)
    dd8a8a05: revokePermission(address,uint8)
```

目前该合约的权限管理功能比较简单，权限共有两种：发送交易和创建合约（Send, Create）。每个地址默认没有这两种权限，可通过有权限的地址授权获得其中一种或两种权限。
比如，授予Create权限，已经拥有Create权限的地址可调用合约中的grantPermission()方法，来给其他地址授予Create权限。
合约中的方法介绍如下：

- `grantPermission(address,uint8)`，该方法是授予某种权限，其中参数中的address表示授予权限的地址，uint8表示权限名称，拥有该权限的地址可
通过cita_sendTransaction调用该方法来授予其他地址该权限。

- `revokePermission(address,uint8)`，该方法是取消某种权限，其中参数address表示取消权限的地址，uint8表示权限名称，拥有该权限的地址可
通过cita_sendTransaction调用该方法来取消其他地址该权限。

- `queryPermission(address)`，该方法是查询指定地址的权限，可通过cita_sendTransaction调用该方法来查询。

- `queryUsersOfPermission(uint8)`， 该方法是查询拥有指定权限的所有用户，可通过cita_sendTransaction调用该方法来查询。
