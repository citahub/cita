# Changelog

After alomost three months, CITA version 0.21 is coming. In this version, we have bring several features
such as automatic contract execution, hostname parsing in network. At the same time, we did lots of work
in module code refactoring, test coverage, bug fixs.

In the following time, CITA enters a maintenance period temporarily. We plan not to add more new features and
committed to improving the quolity of code, fixing bugs, improving documentation and supplementing tests.

## [Unreleased]

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

### Tool

- [Optimization] Split util module into standalone crates. [@yangby]
- [Refactor] Combing the snapshot logic and rewrite snapshot_tools. [@keroro520]

[Unreleased]: https://github.com/cryptape/cita/compare/v0.21...HEAD

[@KaoImin]: https://github.com/KaoImin
[@WPF]: https://github.com/ouwenkg
[@driftluo]: https://github.com/driftluo
[@jerry-yu]: https://github.com/jerry-yu
[@kaikai]: https://github.com/kaikai1024
[@keroro520]: https://github.com/keroro520
[@lhf]: https://github.com/EighteenZi
[@u2]: https://github.com/u2
[@yangby]: https://github.com/yangby-cryptape
[@zhiwei]: https://github.com/rink1969
[@zhouyun-zoe]: https://github.com/zhouyun-zoe
[@zeroqn]: https://github.com/zeroqn
