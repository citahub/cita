# <img src="https://github.com/cryptape/assets/blob/master/CITA-logo.png?raw=true" width="256">

[![Build Status](https://travis-ci.com/cryptape/cita.svg?branch=develop)](https://travis-ci.com/cryptape/cita)

[English](./README.md) | 简体中文

## CITA 是什么？

CITA（ Cryptape Inter-enterprise Trust Automation ）是一个面向企业级应用的支持智能合约的区块链框架，
旨在为企业级区块链应用提供一个稳固、高效、灵活、可适应未来的运行平台。CITA 将区块链节点的必要功能解耦为六个微服务：RPC，Auth，Consensus，Chain，Executor，Network。各组件之间通过消息总线交换信息相互协作。通过配置和定制相应的服务，CITA 能够满足企业级用户的全部需要。

- **水平扩展性**

  在 CITA 的微服务架构中，“节点”是一个逻辑概念，有可能是一台服务器（一台服务器上面运行一组微服务），
  也有可能是一组服务器组成的集群，同时 CITA 还支持部署在云服务器上，充分利用了各种服务器硬件来提升处理能力。
  节点与节点之间通过P2P通信，节点内部各模块通过消息总线通信，这一点与 Fabric 仅仅在共识模块运用消息总线通信完全不同。

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-network.png?raw=true)

![](https://github.com/cryptape/cita-whitepaper/blob/master/en/cita-parallel.png?raw=true)

- **组件可插拔**

  松耦合的微服务架构，便于各组件将来平滑迁移至更好的算法（比如新的共识算法）或者更好的技术方案（比如新的DB或者新的隐私方案）；
  也有利于针对一些具体的业务场景，定制一些特定的功能。

- **高性能**

  微服务架构将 Chain 与 Executor 独立出来，Executor 仅负责计算和执行交易，Chain 负责存储交易，
  使得计算和存储分离，极大程度的提高了交易处理能力；
  编程语言采用 Rust，Rust 强调并秉持零开销抽象的理念在提供诸多高级语言特性的同时，没有引入额外的开销，性能可以媲美 C++。
  最新版本的交易性能已经可以达到 15,000+ TPS（数据来自 CITA 0.16 版本，在四台 32 核，64G 的云服务器上部署 4 个节点，每台服务器配置百兆带宽）。

- **稳定可靠**

  CITA 提供快照工具来对区块链的数据进行备份，可在较短时间内恢复链数据。
  同时，Rust 借鉴了编程语言领域最新的研究成果，针对 C++中最头疼的内存问题（内存泄漏，野指针）进行编译器静态检查。
  只要代码编译通过，就可以保证没有以上问题，大大提高了应用运行阶段的可靠性。

- **兼容性**

  CITA上支持使用 Solidity，Go 语言，Rust 开发智能合约，同时也支持以太坊的所有开发工具（Truffle，Zeppeling，Remix 等）。

- **跨链**

  在区块链世界里，各种各样的链在不断的涌现出来。这些链如何互相配合形成区块链网络？
  CITA 目前提供了一个简单的跨链协议来支持主链与侧链之间的通信。我们也正对跨链通信做更多的探索，旨在扩大在各种链上运行的应用程序的价值。

- **工程经验**

  CITA 现在已经在银行，证券，票据等实际生产环境中运行，这其中我们积累了大量工程经验。

## 白皮书

- [英文版](https://github.com/cryptape/cita-whitepaper/blob/master/en/technical-whitepaper.md)
- [中文版](https://github.com/cryptape/cita-whitepaper/blob/master/zh/technical-whitepaper.md)

## 文档

- [英文版](https://docs.citahub.com/en-US/cita/cita-intro)
- [中文版](https://docs.citahub.com/zh-CN/cita/cita-intro)

## API/SDK

CITA 支持 JSON-RPC 和 WebSocket (experimental) API/v1。

对于 CITA 的 API/v1，你可以使用任意的 HTTP 代理，或者下面的 SDK：

* [Java](https://github.com/cryptape/cita-sdk-java)
* [JavaScript](https://github.com/cryptape/cita-sdk-js)
* [Swift](https://github.com/cryptape/cita-sdk-swift)
* [Ruby](https://github.com/cryptape/cita-sdk-ruby)

## 社区贡献

CITA 目前仍在紧张的开发中，欢迎大家为 CITA 贡献自己的一份力量。更详细的信息可以参考[贡献指南](CONTRIBUTING.md)。

## 关注我们

[Twitter](https://twitter.com/Cryptape)

[Weibo](http://weibo.com/u/6307204864)

## 开源协议

GPLv3 license

## 权益归属

<img src="https://github.com/cryptape/assets/blob/master/cryptape-logo-square.png?raw=true">

秘猿科技团队 :heart:

## 联系我们

邮箱： <contact@cryptape.com>
