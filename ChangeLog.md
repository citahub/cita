# Changelog

All notable changes to this project will be documented in this file. And this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Framework

### Executor

### Chain

### Auth

### Network

### Consensus

- [Optimization] Replace std channel with crossbeam channel. [@kaikai]

### RPC

- [Feature] Add `from` to `body` of `getBlockByNumber` and `getBlockByHash`. [@CL]

### Forever

- [Optimization] Replace std channel with crossbeam channel. [@kaikai]

### System Contract

### Scripts

### Test

### Doc

- [Doc] Update Rust SDK info in readme. [@u2]
- [Doc] More info about automatic execution. [@wangfh666]
- [Doc] Fix start cita command in log management. [@77liyan]

### Tool

[Unreleased]: https://github.com/cryptape/cita/compare/v0.22.0...HEAD

## [v0.21.0]

### CITA-Framework

- [Optimization] Upgrade default rust toolchain to stable. [@yangby]
- [Optimization] Remove useless dependencies. [@yangby]
- [Optimization] Compact block Relay. [@u2] [@yangby]

### Executor

- [Feature] Automatic execution. [@kaikai]
- [Optimization] Enable changing size of global cache in StateDB. [@lhf]
- [Refactor] Decouple executor and postman [@keroro520] [@WPF]
- [Configuration] Deprecate `--genesis` command option, instead place into `executor.toml`. [@keroro520]
- [Configuration] Add argument about timestamp uint in `executor.toml` to compatibility with Ethereum.[@zhiwei]
- [Optimization] Change state db type to ensure safe reference. [@WPF]
- [Optimization] Remove unused code in state db. [@WPF]
- [Optimization] Add more tests in executor and postman. [@WPF]
- [Optimization] Add block priority in postman. [@keroro520]
- [Refactor] Decouple global sysconfig from transactionOptions. [@kaikai]
- [Optimization] Deprecate some dangerous clone usage in block and state. [@keroro520]
- [Optimization] Remove cached latest hashes. [@zhiwei]
- [Fix] Fix problem in zk privacy. [@zhiwei]
- [Fix] Fix defects in snapshot. [@keroro520]

### Chain

- [Optimization] Rename crypto enum. [@zhiwei]

### Auth

- [Optimization] Introduce quick check for history heights. [@zhiwei]

### Network

- [Feature] Enable parsing hostname directly in network.toml. [@driftluo]
- [Fix] Fix bug for network not send all msg. [@jerry-yu]

### Consensus

- [Optimization] Add a min-heap timer. [@KaoImin]
- [Optimization] Optimize wait time for proposal, prevote and precommit. [@jerry-yu]

### RPC

- [Fix] The 'chainIdV1' in the response of getMetaData is hex string, so it should have 0x-prefix. [@yangby]
- [Optimization] Split libproto operations from Jsonrpc. [@zeroqn]
- [Feature] Add `from` field in `Gettransaction` rpc interface. [@zeroqn]
- [Optimization] Upgrade hyper version and split `Service` and `Server`. [@zeroqn]
- [Fix] Fix `getFilterChanges` interface, the hash array returned in the case of a block filter starts from the next block. [@WPF]

### System Contract

- [Feature] Change default quotaPrice to 1000000. [@WPF]
- [Optimization] Take interfaces and test contracts out as a dependent submodule. [@kaikai]

### Scripts

- [Feature] Store their own address for each node. [@yangby]
- [Configuration] Rename checkPermission to checkCallPermission. [@kaikai]
- [Feature] Check the maximum number of consensus nodes. [@zhiwei]
- [Configuration] Optimize usage of backup and clean command. [@keroro520]
- [Optimization] Add exit info about creating genesis. [@kaikai]
- [Feature] Support start 4 nodes in docker compose. [@zhiwei]

### Test

- [Optimization] Split large ci jobs. [@u2]
- [Optimization] Add test about amend operation. [@zhiwei]
- [Optimization] Add test to ensure genesis compatibility. [@kaikai]
- [Optimization] Add test about snapshot. [@keroro520]

### Doc

- [Doc] Complete the doc of system contract interface. [@kaikai]
- [Doc] Update crypto type and timestamp configuration in executor.toml. [@zhiwei]
- [Doc] More detail statements about cita-bft consensus. [@KaoImin]
- [Doc] Update sdk info in readme. [@zhouyun-zoe]
- [Doc] Add node command description. [@WPF]
- [Doc] Build a new [documentation website](https://docs.citahub.com/en-US/cita/cita-intro). [@zhouyun-zoe]

### Tool

- [Optimization] Split util module into standalone crates. [@yangby]
- [Refactor] Combing the snapshot logic and rewrite snapshot_tools. [@keroro520]

[v0.21.0]: https://github.com/cryptape/cita/compare/v0.21...HEAD

[@77liyan]: https://github.com/77liyan
[@CL]: https://github.com/classicalliu
[@KaoImin]: https://github.com/KaoImin
[@WPF]: https://github.com/ouwenkg
[@driftluo]: https://github.com/driftluo
[@jerry-yu]: https://github.com/jerry-yu
[@kaikai]: https://github.com/kaikai1024
[@keroro520]: https://github.com/keroro520
[@lhf]: https://github.com/EighteenZi
[@u2]: https://github.com/u2
[@wangfh666]: https://github.com/wangfh666
[@yangby]: https://github.com/yangby-cryptape
[@zeroqn]: https://github.com/zeroqn
[@zhiwei]: https://github.com/rink1969
[@zhouyun-zoe]: https://github.com/zhouyun-zoe
