# Installation
目前使用的python版本为python2.7
```
$ pip install -r requirements.txt
```

# 配置N个节点
在admintool.sh脚本里初始配置N个节点ip和端口号，可修改IP_LIST参数，如配置六个节点：
```
IP_LIST="127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003,127.0.0.1:4004,127.0.0.2:4005"
```

# 运行admintool.sh脚本
  主要功能如下：
- 生成私钥和地址，私钥存放在`admintool/release/nodeID/privkey`，其中nodeID为节点号；而所有节点地址都存放在`admintool/release/authorities`；
- 生成网络配置文件，存放在`admintool/release/nodeID/network.toml`，文件内容主要为总节点数、本地节点端口以及其它节点的ip和端口号；
- 生成genesis块文件，存放`在admintool/release/nodeID/genesis.json`， 其中timestamp为时间戳，秒为单位；prevhash指前一个块哈希，这里是默认值；而alloc指部署到创世块的合约内容；
- 生成节点配置文件，存放在`admintool/release/nodeID/consensus.json`，主要包含共识算法的相关参数；
- 生成jsonrpc配置文件，存放在`admintool/release/nodeID/jsonrpc.json`，主要包含jsonrpc模块的相关参数。

# 节点的动态管理
- **节点的动态管理**是通过调用部署到链上的节点管理合约来实现的，合约存放在`cita/contracts/node_manager.sol`，其中合约地址部署在创世块中，函数签名可通过`solc node_manager.sol --hashes`编译得到，node_manager.sol合约详情如下所示：
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
- **节点状态**，目前节点有三种状态：Close，Ready，Start，初始默认为Close；
- **增加共识节点**，首先申请者调用合约上的newNode(address)方法，此时节点状态变更为Ready；然后需要审批者（共识节点）通过调用approveNode(address)来同意节点状态为Ready的节点加入共识，此时节点状态变更为Start；
- **删除共识节点**，通过调用合约上的deleteNode(address)方法，节点状态变更为Close。
















