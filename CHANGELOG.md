# Changelog

All notable changes to this project will be documented in this file. And this project adheres to [Semantic Versioning].

## [Unreleased]

## [v0.24.1] - 2019-06-14

Fix the issue about memory leak in cita-executor.
Check the details at [#588]

## [v0.24.0] - 2019-05-18

### Framework

- [Optimization] Support new protocol version 2. [@ouwenkg]
- [Optimization] Bump cita-sdk-js to `0.23.1` to fix the problem of version 2. [@kaikai1024]
- [Optimization] Bump Rust toolchain to `1.34.1`. [@kaikai1024]
- [Optimization] Use fixed Solidity version for system contracts. [@kaikai1024]
- [Optimization] Upgrade the node packages to avoid the security alert. [@kaikai1024]
- [Optimization] Use new log pattern. [@kaikai1024]
- [Optimization] Update genesis submodule: use tag. [@kaikai1024]

### Executor

- [Fix] Fix coinbase in auto exec. [@ouwenkg]
- [Fix] Fix potential panic issue in evm trace. [@ouwenkg]
- [Fix] Refund token when suicide. [@ouwenkg]
- [Optimization] Update zktx to use stable verison of Rust. [@kaikai1024]

### Network

- [Feature] Use new P2P library. [@jerry-yu]
- [Feature] Let network be configuration. [@jerry-yu]
- [Fix] Fix bug for saving wrong session id. [@jerry-yu]
- [Fix] Fix for operating repeated peer key. [@jerry-yu]

### Chain

- [Fix] Fix the bug that getLogs would break down the chain when `toBlock` is very large. [@leeyr338]
- [Optimization] Update log in `forward` of `cita-chain`. [@ouwenkg]
- [Fix] Move `chain_version` into `BlockSysConfig`. [@ouwenkg]

### Consensus

- [Fix] Fix Bft panic when new ordinary node syncs the blocks. [@leeyr338]
- [Fix] Fix the bug that chain can't product block after restart docker. [@leeyr338]
- [Fix] Fix the bug for jumping round when delayed. [@jerry-yu]

### Forever

- [Fix] Fix service start error when `.*.pid` files exist. [@leeyr338]

### Tools

- [Fix] Fix panic in snapshot path. [@ouwenkg]

### Test

- [Optimization] Set more time for checking P2P network. [@leeyr338]
- [Optimization] Use testdata submodule to test rpc interface. [@kaikai1024]
- [Fix] Fix bug of `blockNumber.sh` script. [@rev-chaos]

### Scripts

- [Optimization] Modify script for new network config. [@leeyr338]
- [Fix] Set logrotate output log as a relative path. [@leeyr338]
- [Fix] Generate privkey file when use authorities option. [@leeyr338]
- [Feature] Let native token be configurable. [@leeyr338]
- [Fix] Fix docker multi-ports expose error. [@leeyr338]
- [Optimization] Remove the useless scripts. [@kaikai1024]
- [Optimization] Remove warnings of yaml. [@ouwenkg]

### Doc

- [Doc] Refactor `getting-started` doc. [@zhouyun-zoe]
- [Doc] Add `economics model` docs.[@zhouyun-zoe]
- [Doc] Add `zktx` docs. [@kaikai1024]
- [Doc] Add `depository` sample. [@leeyr338]
- [Doc] Add `operation guide` section. [@zhouyun-zoe]
- [Doc] Add English version of release guide. [@YUJIAYIYIYI]
- [Doc] Add logging rule doc. [@leeyr338]

## [v0.23.1] - 2019-05-05

Fix the issue about showing the wrong version.
Check the details at [#538]

## [v0.23.0] - 2019-04-26

### Upgrade Note

In `v0.23.0`, CITA upgraded the P2P discovery protocol, which leads to Incompatibility with `v0.22.0`. So the nodes with CITA `v0.23.0` and the nodes with CITA `v0.22.0` cannot discover each other in the network. Therefore, when upgrading, all nodes in the network need to be upgraded to `v0.23.0` at the same time.

Following [Upgrade Instructions](https://docs.citahub.com/en-US/cita/protocol-upgrade/overview) to upgrade the nodes.

### New Feature Description

The `v0.23.0` version added two RPC interfaces:

* [Get software version]:

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"getVersion","params":[],"id":83}'
```

* [Get peer information]:

```shell
curl -X POST --data '{"jsonrpc":"2.0","method":"peersInfo","params":[],"id":83}'
```

### Framework

- [Optimization] Update default rust toolchain to `v1.34.0`. [@yangby-cryptape] [@kaikai1024]
- [Optimization] Update cita-sdk-js version. [@kaikai1024]
- [Fix] Reorganize toml path. [@ouwenkg]
- [Feature] Log output mode can be configured as stdout or file. [@Kayryu]

### Executor

- [Optimization] Integrate vm-test. [@ouwenkg]
- [Optimization] Add unit test of calling contract. [@kaikai1024]

### Auth

- [Fix] Auth crashes when it is not ready. [@leeyr338]

### Network

- [Fix] High CPU usage. [@leeyr338]
- [Fix] Refuse connect when reach max connections. [@leeyr338]
- [Optimization] Add discovery test of network. [@leeyr338]
- [Optimization] Use new version `p2p` to fix network run crash. [@jerry-yu]

### Consensus

- [Optimization] Set the default NTP service to false. [@kaikai1024]
- [Fix] Not generate block. [@jerry-yu]

### RPC

- [Feature] Add `getVersion` interface. [@luqz]
- [Feature] Add `peersInfo` interface. [@leeyr338]
- [Fix] Get logs break down the chain when `toBlock` very large. [@leeyr338]

### Scripts

- [Optimization] Installation && Exectution Optimization: new usage of cita script. [@clearloop]
- [Fix] Redirect the stdout and stderr for daemon processes in docker. [@yangby-cryptape]
- [Fix] Eliminate warnings when create nodes in docker. [@ouwenkg]
- [Fix] Generate privkey file when use authorities option. [@leeyr338]
- [Optimization] Log rotate output log as a relative path. [@leeyr338]
- [Optimization] Patch to absolute paths' in starting scripts. [@clearloop]
- [Optimization] Format the `env.sh` using `ShellCheck`. [@clearloop]

### Doc

- [Doc] Add style guide of codes. [@kaikai1024]
- [Doc] Add all contributors. [@kaikai1024]
- [Doc] Add more template types of issue and pull request. [@kaikai1024]
- [Doc] Add editorconfig file. [@kaikai1024]
- [Doc] Add release guide doc. [@kaikai1024]
- [Doc] Fix 404 error of `CITAHub` Docs. [@zhouyun-zoe] [@Keith-CY]
- [Doc] Add roadmap and fix contributing docs of `CITAHub`. [@zhouyun-zoe]
- [Doc] Change CITA slogan into blockchain kernel. [@zhouyun-zoe]
- [Doc] Update the description of BlockTag. [@xiangmeiLu]
- [Doc] Fix protocol upgrade doc. [@QingYanL]
- [Doc] Set default website with zh-CN language. [@wuyuyue]

## [v0.22.0] - 2019-03-29

### Upgrade Note

The v0.22.0 version of the node configurations is compatible with the v0.21 version.
Means that you can run v0.22.0 directly using the v0.21 node configurations.
However, due to the refactoring of the network, the nodes executed with v0.22.0 are
incompatible with the original nodes (they have different node discovery and transfer protocols),
so all nodes need to be upgraded to v0.22.0 at the same time.

Following [Upgrade Instructions] to upgrade the nodes.

### New Feature Description

The new feature of integrating p2p to network service, we add discovery of the network node
when the original configuration is compatible. But we still need to make some changes to the
network configuration file definition:

The old version `network.toml` looks like:

```toml
port = 4000
enable_tls = true
id_card = 9
[[peers]]
    ip = "127.0.0.1"
    port = 4001
    common_name = "test1.cita"
[[peers]]
    ip = "127.0.0.1"
    port = 4002
```

In the version of v0.22.0, we will discard the item `id_card` and `common_name`.

In the old version, when a new node is added to the network, we need to change the item `[[peers]]` in all
nodes' `network.toml` to reconstruct the network. It is a very complicated operation.
But in v0.22.0, the item `[[peers]]` means `known nodes` in the network, you can set only one
`[[peers]]`, then it can discovery all the network nodes through a discovery protocol.

### Framework

- [Optimization] Replace std channel with crossbeam channel. [@kaikai1024] [@leeyr338]
- [Optimization] Reconfigure the parameters of rocksdb, and this can greatly reduce the `.sst` files in the database. [@jerry-yu]

### Executor

- [Fix] Executor crashes when receives staled BlockWithProof. [@ouwenkg] [@keroro520]

### Auth

- [Fix] Auth crashes when it is not ready. [@leeyr338]

### Network

- [Feature] The network service is refactored by using the p2p protocol. [@leeyr338]

### Consensus

- [Fix] Consensus goes into `panic` when timer min peek is extremely close to Instant::now(). [@KaoImin]

### RPC

- [Optimization] Update test token info. [@kaikai1024]
- [Feature] Add `from` to `body` of `getBlockByNumber` and `getBlockByHash`. [@classicalliu]
- [Fix] Fix the missing CORS header. [@yangby-cryptape]

### Scripts

- [Optimization] Format Python codes. [@ouwenkg]

### Doc

- [Doc] Update Rust SDK info in readme. [@u2]
- [Doc] More info about automatic execution. [@wangfh666]
- [Doc] Fix start cita command in log management. [@77liyan]

## [v0.21.1] - 2019-03-15

Fix the issue about high CPU usage caused by too many sst files.
Check the details at [#206]

## [v0.20.3] - 2019-03-11

Fix the issue about high CPU usage caused by too many sst files.
Check the details at [#206]

## [v0.21.0] - 2019-02-19

### Upgrade Note

Older version upgrades the v0.21 version requires node configuration modifications.

- Adding the following three configurations in each node's `executor.toml`

The old version `executor.toml`:

```toml
Journaldb_type = "archive"
Prooftype = 2
Grpc_port = 5000
```

The new version `executor.toml`:

```toml
Journaldb_type = "archive"
Prooftype = 2
Grpc_port = 5000
Genesis_path = "./genesis.json"
Statedb_cache_size = 5242880
Eth_compatibility = false
```

- Modifying cita-execuror configuration item in each node's `forever.toml`:

The old version `forever.toml`:

```toml
[[process]]
Name = "cita-executor"
Command = "cita-executor"
Args = ["-g","genesis.json","-c","executor.toml"]
Pidfile = ".cita-executor.pid"
Respawn = 3
```

The new version `forever.toml`:

```toml
[[process]]
Name = "cita-executor"
Command = "cita-executor"
Args = ["-c","executor.toml"]
Pidfile = ".cita-executor.pid"
Respawn = 3
```

After completing the above modifications, following [Upgrade Instructions].

### CITA-Framework

- [Optimization] Upgrade default rust toolchain to stable. [@yangby-cryptape]
- [Optimization] Remove useless dependencies. [@yangby-cryptape]
- [Optimization] Compact block Relay. [@u2] [@yangby-cryptape]

### Executor

- [Feature] Automatic execution. [@kaikai1024]
- [Optimization] Enable changing size of global cache in StateDB. [@EighteenZi]
- [Refactor] Decouple executor and postman [@keroro520] [@ouwenkg]
- [Configuration] Deprecate `--genesis` command option, instead place into `executor.toml`. [@keroro520]
- [Configuration] Add argument about timestamp uint in `executor.toml` to compatibility with Ethereum.[@rink1969]
- [Optimization] Change state db type to ensure safe reference. [@ouwenkg]
- [Optimization] Remove unused code in state db. [@ouwenkg]
- [Optimization] Add more tests in executor and postman. [@ouwenkg]
- [Optimization] Add block priority in postman. [@keroro520]
- [Refactor] Decouple global sysconfig from transactionOptions. [@kaikai1024]
- [Optimization] Deprecate some dangerous clone usage in block and state. [@keroro520]
- [Optimization] Remove cached latest hashes. [@rink1969]
- [Fix] Fix problem in zk privacy. [@rink1969]
- [Fix] Fix defects in snapshot. [@keroro520]

### Chain

- [Optimization] Rename crypto enum. [@rink1969]

### Auth

- [Optimization] Introduce quick check for history heights. [@rink1969]

### Network

- [Feature] Enable parsing hostname directly in network.toml. [@driftluo]
- [Fix] Fix bug for network not send all msg. [@jerry-yu]

### Consensus

- [Optimization] Add a min-heap timer. [@KaoImin]
- [Optimization] Optimize wait time for proposal, prevote and precommit. [@jerry-yu]

### RPC

- [Fix] The 'chainIdV1' in the response of getMetaData is hex string, so it should have 0x-prefix. [@yangby-cryptape]
- [Optimization] Split libproto operations from Jsonrpc. [@zeroqn]
- [Feature] Add `from` field in `Gettransaction` rpc interface. [@zeroqn]
- [Optimization] Upgrade hyper version and split `Service` and `Server`. [@zeroqn]
- [Fix] Fix `getFilterChanges` interface, the hash array returned in the case of a block filter starts from the next block. [@ouwenkg]

### System Contract

- [Feature] Change default quotaPrice to 1000000. [@ouwenkg]
- [Optimization] Take interfaces and test contracts out as a dependent submodule. [@kaikai1024]

### Scripts

- [Feature] Store their own address for each node. [@yangby-cryptape]
- [Configuration] Rename checkPermission to checkCallPermission. [@kaikai1024]
- [Feature] Check the maximum number of consensus nodes. [@rink1969]
- [Configuration] Optimize usage of backup and clean command. [@keroro520]
- [Optimization] Add exit info about creating genesis. [@kaikai1024]
- [Feature] Support start 4 nodes in docker compose. [@rink1969]

### Test

- [Optimization] Split large ci jobs. [@u2]
- [Optimization] Add test about amend operation. [@rink1969]
- [Optimization] Add test to ensure genesis compatibility. [@kaikai1024]
- [Optimization] Add test about snapshot. [@keroro520]

### Doc

- [Doc] Complete the doc of system contract interface. [@kaikai1024]
- [Doc] Update crypto type and timestamp configuration in executor.toml. [@rink1969]
- [Doc] More detail statements about cita-bft consensus. [@KaoImin]
- [Doc] Update sdk info in readme. [@zhouyun-zoe]
- [Doc] Add node command description. [@ouwenkg]
- [Doc] Build a new [documentation website]. [@zhouyun-zoe]

### Tool

- [Optimization] Split util module into standalone crates. [@yangby-cryptape]
- [Refactor] Combing the snapshot logic and rewrite snapshot_tools. [@keroro520]

## [v0.19.1] - 2019-01-31

Fix the bug of version 0.19 that ordinary nodes can't sync blocks from the consensus nodes with a special situation.

Check the details at [#201].

## [v0.20.2] - 2018-11-27

Fixed a bug that getting blockhash in solidity contract will get uncertain result.

```
pragma solidity ^0.4.24;
contract Test {
    bytes32 public hash;

    function testblockhash() public {
        hash = blockhash(block.number-1);
    }
}
```

Deploy this contract, then send transaction to call testblockhash.
Once one of the nodes receives the transaction, the chain will stop growing.

## [v0.20.1] - 2018-11-15

Fixed a bug that Network could not process domain names.

## [v0.20.0] - 2018-11-09

### Compatibility

* This new version changes the log format of BFT wal. So it is necessary for each consensus node to be upgraded one by one (the interval should be more than 30s).
* If you need to upgrade all the nodes at the same time, follow the steps below:
  * Stop all the nodes;
  * Upgrade all the nodes;
  * Use the `bft-wal` tool to manually convert the log format of BFT wal;
  * Restart all the nodes.
  ```shell
  DATA_PATH=./test-chain/0/data ./bin/bft-wal
  ```

### Protocol

* [Feature] Add the support for `v1` protocol.
  More details can be found in the document: [Protocol Upgrade From V0 to V1].

### Bootstrap

* [Optimization] Force the use of `--super_admin` to configure the administrator account when using `create_cita_config.py` to create a new chain.

### Framework

* [Upgrade] Upgrade rustc to `nightly-2018-10-05`, and update the docker image (latest image `cita/cita-run:ubuntu-18.04-20181009`).

### Executor

* [Deprecated] Deprecate the use of `delay_block_number`.
* [Refactor] Use `BlockID` explicitly in methods that require the use of `BlockID` instead of using a fuzzy Default value.
* [Refactor] Refactor duplicated codes in both of Executor and Chain.
* [Refactor] Refactor some unsafe codes.

### Chain

* [Upgrade] Upgrade RocksDB.

### Auth

* [Feature] Add the check of the version field in the transaction.

### Network

* [Refactor] Refactor network client.
* [Upgrade] Upgrade network server.
* [Feature] Support for TLS communication encryption based on self-signed certificate.
* [Fix] Parsing will stop immediately when the body of messages between nodes is too large.

### RPC

* [Fix] Fix the problem that the website returned by the [`getMetaData`] interface is incorrect.
* [Fix] The error information returned by the [`sendRawTransaction`] interface may be inconsistent when there are duplicate transactions.
* [Feature] Add the pending type in the [`BlockTag`] type.
* [Fix] The exit code caused by the configuration file exception is corrected to `2`.

### System Contract

* [Fix] Fix user authentication problem inside the group when the permission management is enabled.

### Test

* [Optimization] Optimize the efficiency of system contract testing.

### Doc

* [Docs] Add system contract interface documents.
* [Docs] Add more English document.

## [v0.19.0] - 2018-09-30

### CITA-Framework

* [refactoring] Improve the user experience of CITA scripts

### Executor

* [feature] Support superadmin to [set quota price]
* [feature] Support that the block reward can be chosen to [return to the certain address]
* [optimization] SysConfig reload based on whether there is a parameter change
* [fix] Fix loading problem of SystemConfig configuration
* [fix] Fix the situation that the transfer cannot be successful if the charge mode is enabled
* [fix] Fix the situation that account balance may overflow when transferring in charge mode

### Chain

* [feature] Add the cache_size entry to the configuration file

### Auth

* [feature] Modify the judgment logic of the transaction under emergency braking situation

### RPC

* [feature] [GetMetaData] support query economic model and protocol version number
* [optimization] Modify some ErrorMessage

### Contract

* [feature] Isolate some permissions (send_tx, create_contract)  to make them can be set separately in configuration
* [fix] Eliminate compilation warnings for system contracts
* [fix] Eliminate errors and warnings detected by [Solium] on system contracts
* [feature] Add [Emergency braking system contract]
* [feature] Add version control system contract
* [feature] Add [quota price manager system contract]

### Test

* [ci] Increase the specification check of system contracts
* [ci] Add clippy for code review
* [optimization] Clean up smart contract unit tests that are no longer maintained
* [ci] Fix the problem of sporadic stuck in JSON Mock test

### Doc

* [doc] Replace txtool with [cita-cli] in document
* [doc] Modify ‘amend’ operation related documents

## [v0.18.0] - 2018-08-30

### CITA-Framework

* [feature] Replaced sha3 with Keccak algorithm library
* [feature] New Library of China Cryptographic Algorithm
* [optimize] Remove useless code and dependencies
* [optimize] Add more CI

### Executor

* [fix] Fix potential deadlock, multi-threaded data inconsistency
* [fix] Fix state machine state homing problem
* [fix] Fix Transaction decode logic error
* [fix] Fix blacklist problems that accounts cannot be removed from blacklist automatically when they come to have tokens
* [optimize] Add the monitor of chain status
* [feature] Modify some log levels
* [fix] Automatic synchronization when the Executor state is inconsistent with Chain
* [optimize] Optimize state synchronization speed between Executor and Chain
* [feature] Add the acquisition and verification of the state certificate

### Chain

* [optimize] Add a notification of  Executor status
* [fix] Fix the problem about saving the latest proof when syncing
* [fix] Fix some usability issues in the snapshot

### Network

* [refactoring] Refactoring synchronization logic
* [feature] Output status log
* [fix] Close the connection to the deleted node when the Network configuration file is hot updated

### Bft

* [fix] Fix the problem about saving temporary proof

### Auth

* [feature] Transaction's value field validation is modified to be required to U256 or [u8;32], otherwise, an invalid value is returned.
* [fix] Transaction's to field validation is more strict, passing invalid parameters will return an error directly

### RPC

* [feature] Separate the JSON-RPC type definition library for the client to use

### System Contract

* [feature] Add cross-chain management contract to the process of state proof
* [feature] Supports batch transaction

### Doc

* [feature] Update the cross-chain document with more description about [sidechain exit mechanism].

## [v0.17.0] - 2018-07-18

### CITA-Framework

* [feature] Enable rabbitmq web management
* [fix] Merge env_cn.sh into env.sh
* [feature] Add economical model support Public-Permissioned Blockchain

### Executor

* [fix] Fix EVM lost builtin
* [fix] Fix Executor Result cache
* [feature] Support contract amend, superadmin can modify the code and data of the contract

### Chain

* [fix] Fix nodes concurrent start failed
* [fix] Fix block number go down
* [fix] Fix authorities list shuffle test
* [feature] Support set value in genesis
* [fix] Fix infinite loop triggered by sync block

### Auth

* [refactoring] Refactor auth

### Consensus

* [fix] Fix consensus stop after restart all nodes

### Contract

* [fix] Fix quota check
* [fix] Fix smart contract static call bug

### RPC

* [fix] Rename JSON-RPC methods.
* [Refactoring] Refactoring JSON-RPC types.

### Test

* [optimize] Speed up CI
* [optimize] Add solc unit test

### Doc

* [doc] Support multiversion
* [doc] Adjust table of contents

## [v0.16.0] - 2018-05-15

### CITA-Framework

* [feature] Simple cross-chain protocol.
* [feature] Add chain_id for different CITA network, to prevent cross chain from replay attack.
* [feature][WIP] Prepare for public permissioned blockchain.

### Executor

* [feature] Add global account cache.

### Chain

* [optimize] Optimize block synchronization.
* [fix] Fix pre-execution bugs.
* [fix] Fix receipt error types.

### Auth

* [fix] Fix transaction broadcasting.
* [fix] Fix transaction authentication.

### Consensus

* [fix] Fix bft process in some critical conditions.

### Contract

* [feature] Support group-based user management.

### Doc

* [doc] Add more English documents.

## [v0.15.0] - 2018-03-30

### CITA Framework

* [refactoring] Refactor libproto. Send message between services will be more efficient and easy.

### Executor

* [feature] Upgrade EVM to support new instructions. Such as RETURNDATACOPY, RETURNDATASIZE, STATICCALL and REVERT.

### Chain

* [feature] Store contract ABI into Account. So SDK can generate Java/js code even without souce code of contract.

### JSONRPC

* [refactoring] Improve code quality.

### System Contracts

* [feature] Improve role-based permission contract.

### Document

* [doc] New document site

### Toolchain

* [tool] New [CITA docker images]. We recommend to use docker now and we supply some scripts to simplify this task.

## [v0.13.0] - 2018-02-01

### CITA Framework

* [refactoring] Create new [Executor service], better transaction execution customizability.
* [refactoring] Improve message format and protocols used by microservices.
* [experimental] [Account model based zero-knowledge proof transaction.] Feature turned off by default.

### Chain

* [fix] fix memory leaking problem

### Auth

* [refactoring] Improve code quality
* [fix] fix txpool transaction deletion bug

### JSONRPC

* [doc] update documents
* [fix] fix transaction query bug

### System Contracts

* [feature] Improve role-based permission contract
* [feature] Support read-only configurations

### Toolchain

* [tool] Update txtool dependencies
* [tool] Update admintool
* [tool] Unify configurations to toml format

## [v0.12.0] - 2018-01-18

### CITA Framework

* [feature] Extract transaction pool and transaction preprocessing to new Auth service.
* [feature] Support log rotating.
* [refactoring] Move consensus service to its own repository.
* [optimization] Use clippy to check code quality.

### Consensus

* [optimization] Optimize voting process to reach consensus faster.
* [optimization] Optimize voting messages to reduce network cost.

### Chain

* [feature] Add chain resource management.
* [optimization] Preprocess consensus proposal.
* [optimization] Reduce latency in consensus message handling.
* [optimization] Optimize block processing.
* [optimization] Optimize quota management.
* [optimization] Optimize native contract execution.

### JSONRPC

* [refactoring] Refactor service, rewrite to event-driven model.
* [feature] Support WebSocket protocol.
* [feature] Support filter* API.
* [doc] Update docs.

### Network

* [refactoring] Refactor code
* [feature] New block synchronization protocol.
* [optimization] Optimize network message lock.
* [fix] fix config file watch.

### System Contracts

* [feature] Add role-based user and permission management.

### Toolchain

* [tool] Support more than 16 local variables in solidity function.
* [tool] Deployment tool for single node environment.
* [tool] Added new tool cita-forever to monitor microservices.

## [v0.10.0] - 2017-10-26

Release the first version of CITA.

[#201]: https://github.com/cryptape/cita/issues/201
[#206]: https://github.com/cryptape/cita/issues/206
[#588]: https://github.com/cryptape/cita/issues/588
[Account model based zero-knowledge proof transaction.]: https://github.com/cryptape/cita/blob/develop/cita-executor/core/src/native/zk_privacy.md
[CITA docker images]: https://hub.docker.com/r/cita/
[Emergency braking system contract]: https://docs.citahub.com/zh-CN/cita/system/emergency-brake
[Executor service]: https://github.com/cryptape/cita/tree/develop/cita-executor
[Get peer information]: https://docs.citahub.com/zh-CN/next/cita/rpc-guide/rpc#peersinfo
[Get software version]: https://docs.citahub.com/zh-CN/next/cita/rpc-guide/rpc#getversion
[GetMetaData]: https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#getmetadata
[Protocol Upgrade From V0 to V1]: https://docs.citahub.com/zh-CN/cita/protocol-upgrade/v1
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Solium]: https://github.com/duaraghav8/Solium
[Upgrade Instructions]: https://docs.citahub.com/en-US/cita/protocol-upgrade/overview
[`BlockTag`]: https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc-types#tag
[`getMetaData`]: https://docs.nervos.org/cita/#/rpc_guide/rpc?id=getmetadata&version=v0.20
[`sendRawTransaction`]: https://docs.citahub.com/zh-CN/cita/rpc-guide/rpc#sendrawtransaction
[cita-cli]: https://github.com/cryptape/cita-cli
[documentation website]: https://docs.citahub.com/en-US/cita/cita-intro
[quota price manager system contract]: https://docs.citahub.com/zh-CN/cita/system/price
[return to the certain address]: https://docs.citahub.com/zh-CN/cita/system/fee-back
[set quota price]: https://docs.citahub.com/zh-CN/cita/system/price
[sidechain exit mechanism]: https://docs.nervos.org/cita/#/crosschain/crosschain_contract_example

[Unreleased]: https://github.com/cryptape/cita/compare/v0.24.1...HEAD
[v0.24.1]: https://github.com/cryptape/cita/compare/v0.24.0...v0.24.1
[v0.24.0]: https://github.com/cryptape/cita/compare/v0.23.0...v0.24.0
[v0.23.1]: https://github.com/cryptape/cita/compare/v0.23.0...v0.23.1
[v0.23.0]: https://github.com/cryptape/cita/compare/v0.22.0...v0.23.0
[v0.22.0]: https://github.com/cryptape/cita/compare/v0.21...v0.22.0
[v0.21.1]: https://github.com/cryptape/cita/compare/v0.21...v0.21.1
[v0.21.0]: https://github.com/cryptape/cita/compare/v0.20...v0.21
[v0.20.3]: https://github.com/cryptape/cita/compare/v0.20.2...v0.20.3
[v0.20.2]: https://github.com/cryptape/cita/compare/v0.20.1...v0.20.2
[v0.20.1]: https://github.com/cryptape/cita/compare/v0.20...v0.20.1
[v0.20.0]: https://github.com/cryptape/cita/compare/v0.19...v0.20
[v0.19.1]: https://github.com/cryptape/cita/compare/v0.19...v0.19.1
[v0.19.0]: https://github.com/cryptape/cita/compare/v0.18...v0.19
[v0.18.0]: https://github.com/cryptape/cita/compare/v0.17...v0.18
[v0.17.0]: https://github.com/cryptape/cita/compare/v0.16...v0.17
[v0.16.0]: https://github.com/cryptape/cita/compare/v0.15...v0.16
[v0.15.0]: https://github.com/cryptape/cita/compare/v0.13...v0.15
[v0.13.0]: https://github.com/cryptape/cita/compare/v0.12...v0.13
[v0.12.0]: https://github.com/cryptape/cita/compare/v0.10...v0.12
[v0.10.0]: https://github.com/cryptape/cita/releases/tag/v0.10

[@77liyan]: https://github.com/77liyan
[@EighteenZi]: https://github.com/EighteenZi
[@KaoImin]: https://github.com/KaoImin
[@Kayryu]: https://github.com/Kayryu
[@Keith-CY]: https://github.com/Keith-CY
[@QingYanL]: https://github.com/QingYanL
[@YUJIAYIYIYI]: https://github.com/YUJIAYIYIYI
[@classicalliu]: https://github.com/classicalliu
[@clearloop]: https://github.com/clearloop
[@driftluo]: https://github.com/driftluo
[@jerry-yu]: https://github.com/jerry-yu
[@kaikai1024]: https://github.com/kaikai1024
[@keroro520]: https://github.com/keroro520
[@leeyr338]: https://github.com/leeyr338
[@luqz]: https://github.com/luqz
[@ouwenkg]: https://github.com/ouwenkg
[@rev-chaos]: https://github.com/rev-chaos
[@rink1969]: https://github.com/rink1969
[@u2]: https://github.com/u2
[@wangfh666]: https://github.com/wangfh666
[@wuyuyue]: https://github.com/wuyuyue
[@xiangmeiLu]: https://github.com/xiangmeiLu
[@yangby-cryptape]: https://github.com/yangby-cryptape
[@zeroqn]: https://github.com/zeroqn
[@zhouyun-zoe]: https://github.com/zhouyun-zoe
