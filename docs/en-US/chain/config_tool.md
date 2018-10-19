# Blockchain configuration

After compiling successfully following getting started, we can configure the chain before starting the node. This document mainly explains how to configure the attributes of the chain itself, including RPC interface, network connection between nodes and so on. And let you know how to build your own chain through specific operation examples.

## Configuration Item

After compiling successfully, the chain configuration items are recorded in `init_data.yml` in the `test-chain/template` directory. Below, we will explain each items:

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

* `SysConfig` : initialize some system information
  - `delayBlockNumber` : indicates that the system contract takes effect after several blocks. The default is 1 block. Deprecation now.
  - `checkPermission` : contract call permission switch
  - `checkSendTxPermission` : send transaction permission switch
  - `checkCreateContractPermission` : contract create permission switch
  - `checkQuota` : quota switch
  - `checkFeeBackPlatform` : feeback switch, the default is `false`, which means return to the address of consensus node, while `true` means return to a certain address set by chain owner
  - `chainOwner` : a certain address set by chain owner, used when `checkFeeBackPlatform` is `true`
  - `chainName` : the name of the chain
  - `chainId` : chain Id
  - `operator` : operator name
  - `website` : operator website
  - `blockInterval` : block interval, default is 3 seconds
  - `economicalModel`: economic model (more details on this below)
  - `name` : token name
  - `symbol` : token symbol
  - `avatar` : link of token icon 
* `QuotaManager` : initialize the administrator address
  - `admin` : default administrator address
* `NodeManager` : initialize consensus node
  - `nodes` : consensus node address
  - `stakes` : the weight of consensus nodes to produce the block 
* `ChainManager` : initialize some information for cross-chain
  - `parentChainId` : parent chain ID
  - `parentChainAuthorities` : list of consensus nodes for the parent chain
* `Authorization` : initialize the administrator address
  - `superAdmin` : administrator address
* `Group` : initialize user group
  - `parent` : the address of the parent group
  - `name` : the name of the group
  - `accounts` : list of users in the group
* `Admin` : administrator
  - `admin` : administrator address
* `VersionManager` : protocol version number
  - `version` : protocol version number

Once the Genesis block is generated, only this three items `chainName`, `operator`, and `website` can be modified after the chain is run. No other items can be modified.

## Economic Model

There are two economic models designed in CITA, Quota (default) and Charge.

`economicalModel = 0` means the Quota model, which means the transaction does not require a fee because the quota is free to set. There is no balance concept, and the transaction does not deduct the balance.

`economicalModel = 1` means the Charge model, which means the transaction requires a fee. The single-step deduction mode is executed for each step of the transaction, and the balance is deducted.

## Configuration Tool

In the `docker` environment, we use `./script/create_cita_config.py` to configure a chain. There are two modes for this tool:

* create: configure the new chain before starting it
* append: add a node to the chain that has been run

You can learn more by running the command `./env.sh scripts/create_cita_config.py -h`.

### `Create` Model

```shell
$ ./env.sh ./script/create_cita_config.py create --help

Usage: create_cita_config.py create [-h]
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

Explanation of necessary parameters:

* `chain_name`: specify the name of the chain. After CITA supports sidechain, generate different chain configuration through chain_name. The default is test-chain.
* `nodes`: specify the IP address and port of the node
* `super_admin` : specify super administrator address
* `contract_arguments` : set the default value of the system contract. For details of this parameter, please check the system contract document.

Notice:

1. The configuration tool will create a folder with the name `chain_name`. The default is `test-chain`. In this folder, you can see folders named with 0, 1, 2 and 3 in which storing the configuration files of each node.
2. Multiple nodes are running on the same server for testing purposes.
    The port number specified by the parameters gRPC, JSON-RPC, WebSocket, etc. is a starting port number.
    The port number actually used by the node is deferred according to the order of nodes, that is, port+n (n is the node number).
    For example, a total of 4 nodes, passing the grpc_port parameter to 7000. The gRPC port number of test-chain/0 is 7000, the gRPC port number of test-chain/1 is 7001, and same in after.
3. CITA has some reserved ports, so it is necessary to avoid port conflicts when setting node network ports, or custom ports. Reserved ports are:
    * Default `grpc` port: 5000 to 5000 + N (N is the total number of nodes, the same below)
    * Default `jsonrpc` port: 1337 to 1337 + N
    * Default `websocket` port: 4337 to 4337+N
    * Default `rabbitmq` port: 4369(epmd)/25672(Erlang distribution)/5671,5672(AMQP)/15672(management plugin)

### Operation example

The following is the most basic command to start a chain, which generates a new chain with four nodes. The default port is 4000, 4001, 4002, 4003. The super administrator uses the default setting. The economic model is `Quota`, and all permission controls are closed.

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
```

The next step is to generate a chain with advanced configurations by the following commands:

```shell
$ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" --contract_arguments SysConfig.checkSendTxPermission=true SysConfig.checkPermission=true SysConfig.economicalModel=1 SysConfig.checkFeeBackPlatform=true SysConfig.chainOwner=0x9a6bd7272edb238f13002911d8c93dd6bb646d15 SysConfig.super_admin=0xab159a4817542585c93f01cfce9cfe6cd4cbd26a
```

The above command generates a chain with four nodes, port defaults to 4000, 4001, 4002, 4003, super administrator address `0xab159a4817542585c93f01cfce9cfe6cd4cbd26a`, operator address
`0x9a6bd7272edb238f13002911d8c93dd6bb646d15`, economic model `Charge`, the transaction fee returning to the operator, and with all the permission.

### Configuring Super Administrator Account Address

```shell
$ ./env.sh ./scripts/create_cita_config.py create --super_admin=0xab159a4817542585c93f01cfce9cfe6cd4cbd26a ...
```

The `--super_admin` parameter in the above command line is used to set the super administrator account address. This account has the highest authority and is used to manage the running status of the entire chain. For security, we highly recommend the users ** should / must ** set the super administrator address by themselves.

In test scenarios, CITA configures a default administrator account address (and its corresponding private key, only for the secp256k1_sha3 version):

```json
{
  Address: 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523
  Private-key: 5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
}
```

### `Append` model

To demonstrate how to add a ordinary node, we need to specify the `chain_name` that has been created. The command is as follows:

```shell
$ ./env.sh ./scripts/create_cita_config.py append --chain_name test-chain --node "127.0.0.1:4004"
$ ls test-chain/
  0 1 2 3 4 template
```

Note: Only one node can be added at a time, and the added one is a normal node. How to upgrade a common contact to a consensus node, please refer to `Node Management`.

## Directory structure

The directory structure for creating 4 consensus nodes by `create` is as follows:

```bash
$ ls test—chain/
  0 1 2 3 template
$ ls 0
  Auth.toml executor.toml jsonrpc.toml chain.toml forever.toml logs
  Consensus.toml network.toml genesis.json privkey data
```

According to the given parameters, 4 nodes are generated. `test-chain/*` contains the configuration file of the nodes, as follows:

* `privkey` : stores private key
* `*.toml` : microservice configuration file, please refer to the microservice description for details.
* `genesis.json` : genesis block file, in which, `timestamp` is in seconds; `prevhash` refers to the previous block hash, here is the default value; and `alloc` refers to the contract content deployed to the Genesis block;
* The `test-chain/template` ：template files, including the consensus node address in `test-chain/template/authorities.list`, and the system contract generation parameter in `test-chain/template/init_data.yml`, node Port address in `test-chain/template/nodes.list` and other information
* `logs` : log information 
* `data` : data storage