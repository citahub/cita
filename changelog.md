# Changelog
All notable changes to this project will be documented in this file. And this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### CITA-Framework

### Executor

- [Feature] Automatic execution by [@kaikai]
- [Optimazation] Enable changing size of global cache in StateDB by [@lhf]
- [Refactor] Decouple executor and postman [@keroro520] [@WPF]
- [Configuration] deprecate `--genesis` command option, instead place into `executor.toml`

### Chain

### Auth

### Network

### Consensus

- [Optimazation] Add a min-heap timer. [@KaoImin]
- [Optimazation] Optimize wait time for proposal, prevote and precommit. [@jerry-yu]

### RPC

- [Fix] The 'chainIdV1' in the response of getMetaData is hex string, so it should has 0x-prefix. [@yangby]

### System Contract

- [Optimazation] Change default quotaPrice to 1000000 by [@WPF]

### Scripts

- [Configuration] Store their own address for each node. [@yangby]
- [Configuration] Rename checkPermission to checkCallPermission. [@kaikai]

### Test

- [Optimazation] Split large ci jobs by [@u2]

### Doc

- [Doc] Complete the doc of system contract interface. [@kaikai]

[Unreleased]: https://github.com/cryptape/cita/compare/v0.20...HEAD

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
