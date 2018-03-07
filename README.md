# <img src="https://github.com/cryptape/assets/blob/master/CITA-logo.png?raw=true" width="256">

[![Join the chat at https://gitter.im/cryptape/cita](https://badges.gitter.im/cryptape/cita.svg)](https://gitter.im/cryptape/cita?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) [![Build Status](https://travis-ci.org/cryptape/cita.svg?branch=master)](https://travis-ci.org/cryptape/cita)

## What is CITA

CITA is a fast and scalable blockchain for enterprises. CITA supports both native contract and EVM contract, by which enterprise users can build their own blockchain applications. CITA has an unique architecture which enables enterprise users to unlease all their computing resources.

- **Horizontal scalability**: CITA adopts a microservices architecture to boost each (logical) node’s performance.With the microservice architecture, a logical node can be easily scaled to a cluster of servers. Outside one node's bounday, nodes communicate with each other using P2P network; Inside each node, microservices communicate with each other by messaging queue. (Note this is completely different from Fabric which use a messaging queue only as consensus process)

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-network.png?raw=true)

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-parallel.png?raw=true)

- **High Performance**: In CITA, consensus and transaction execution are decoupled as separate microservices. The consensus service is only responsible for transaction ordering, which can finish independently before transaction execution, thus increase transaction processing performance. CITA also includes a lot of optimizations to fully utilize multi-cores and multi-servers's computing power.

- **Customizable and Pluggable Components**: CITA is designed to be highly customizable. CITA's microservices are decoupled from each other in the cleanest way, talk with each other by simple messages. It's easy to customize your blockchain to fit your own business requirements. For example you can replace the default consensus with more appropriate algorithms if necessary or you can replace the default executor EVM to something else as well.

- **Production Ready**: There're many CITA networks running in banks and payment gateways production environment, with Cryptape or CITA Integration Provider's technical support.

## White Paper

For more details please check the white paper.

- [English](https://github.com/cryptape/cita-whitepaper/blob/master/en/technical-whitepaper.md)
- [Chinese](https://github.com/cryptape/cita-whitepaper/blob/master/zh/technical-whitepaper.md)

## Document

[chinese](https://cryptape.github.io/cita)

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

<img src="https://github.com/cryptape/assets/blob/master/cryptape-thinking-ape.png?raw=true">

CITA is created by Cryptape LLC with :heart:.

## Contact us

WeChat Group:

<img src="https://github.com/cryptape/assets/blob/master/cryptape-wechat.jpeg?raw=true" width="250">
