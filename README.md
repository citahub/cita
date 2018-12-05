# <img src="https://github.com/cryptape/assets/blob/master/CITA-logo.png?raw=true" width="256">

[![Build Status](https://travis-ci.org/cryptape/cita.svg?branch=develop)](https://travis-ci.org/cryptape/cita)

English | [简体中文](./README-CN.md)

## What is CITA

CITA is a fast and scalable blockchain for enterprises. CITA supports both native contract and EVM contract, by which enterprise users can build their own blockchain applications. CITA has a unique architecture which enables enterprise users to release all their computing resources.

- **Horizontal scalability**: With the microservice architecture, a logical node can be easily scaled to a cluster of servers. Node administrators can increase system capacity simply by adding more PC servers on high load. The administrator can even use dedicated servers to provide services for hot-spot accounts. Outside one node's boundary, nodes communicate with each other using P2P network; inside each node, microservices communicate with each other by messaging queue (Note this is completely different from Fabric which uses a messaging queue only as consensus process).

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-network.png?raw=true)

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-parallel.png?raw=true)

- **Customizable and Pluggable Components**: CITA's microservices are loosely coupled and their communications are only via the message queue. Hence, it‘s flexible to improve current components with better algorithms (such as new consensus algorithms) or more appropriate technical solutions (such as new DBs or new privacy solutions). Moreover, business logic is extremely complicated in enterprise applications. With CITA, you can easily customize your blockchain with the certain feature to fit your own business requirements.

- **High Performance**: In CITA, consensus and transaction execution are decoupled as separate microservices. The consensus service is only responsible for transaction ordering, which can finish independently before transaction execution, thus increase transaction processing performance. In additional, CITA also includes lots of other optimizations to fully utilize multi-cores and multi-servers' computing power. To this end, it utilizes the Rust language, a hybrid imperative/OO/functional language with an emphasis on efficiency.

- **Resiliency and Reliability**: CITA provides tools to backup blockchain data by taking snapshot, which can help you to resync the blockchain data in a short time. And through Rust’s language-level memory and thread guarantees and a disciplined approach to exception-handling, we can state with a high degree of certainty that our code cannot crash, hang or bomb-out unexpectedly.

- **Compatibility**: CITA supports the use of Solidity, Go, and Rust to develop smart contracts. It also supports all Ethereum development tools (Truffle, Zeppelin, Remix, etc.).

- **Chain Interoperability**: We perceive that independent blockchains are constantly emerging nowadays and even more in the future. How do these chains interoperate with each other to form blockchain network? CITA Support cross-chain communication by providing a simple cross-chain protocol currently. More explorations are undertaking in CITA, aiming to amplify the value of applications running on the various chains.

- **Engineering Experience**: There're many CITA networks running in banks, payment and insurance production environment, with Cryptape or CITA Integration Provider's technical support.  CITA has accumulated a lot of engineering experience.

## White Paper

For more details please check the white paper.

- [English](https://github.com/cryptape/cita-whitepaper/blob/master/en/technical-whitepaper.md)
- [Chinese](https://github.com/cryptape/cita-whitepaper/blob/master/zh/technical-whitepaper.md)

## Document

- [English](https://cryptape.github.io/cita/#/en-US/latest/index)
- [Chinese](https://cryptape.github.io/cita/)

## API/SDK

CITA supports JSON-RPC and WebSocket (experimental) API/v1.

For CITA API/v1, You can use any HTTP client, or following SDKs:

* [Java](https://github.com/cryptape/nervosj)
* [JavaScript](https://github.com/cryptape/nervos.js)
* [Swift](https://github.com/cryptape/appchain-swift)

## Contributing

CITA is still in active development. Building a blockchain platform is a huge task, we need your help. Any contribution is welcome.

Please check [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## Follow us

[Twitter](https://twitter.com/Cryptape)

[Weibo](http://weibo.com/u/6307204864)

## License

CITA is currently under the GPLv3 license. See the LICENSE file for details.

**CITA will move to Apache 2.0 license soon.**

## Credits

<img src="https://github.com/cryptape/assets/blob/master/cryptape-logo-square.png?raw=true">

CITA is created by Cryptape team with :heart:.

## Contact us

[Telegram Group](https://t.me/joinchat/E7dJKFL8xTwCe8MaiZWdhw)
