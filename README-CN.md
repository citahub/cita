# <img src="https://github.com/citahub/assets/blob/master/CITA-logo.png?raw=true" width="256">

[![Circle CI Status](https://circleci.com/gh/citahub/cita.svg?branch=develop)](https://circleci.com/gh/citahub/cita)

[![All Contributors](https://img.shields.io/badge/all_contributors-54-orange.svg?style=flat-square)](#contributors)

[English](./README.md) | 简体中文

## CITA 是什么？

CITA（ Rivtower Inter-enterprise Trust Automation ）是一个面向企业级应用的支持智能合约的高性能区块链内核，
旨在为企业级区块链应用提供一个稳固、高效、灵活、可适应未来的运行平台。CITA 将区块链节点的必要功能解耦为六个微服务：RPC，Auth，Consensus，Chain，Executor，Network。各组件之间通过消息总线交换信息相互协作。通过配置和定制相应的服务，CITA 能够满足企业级用户的全部需要。

- **水平扩展性**

  在 CITA 的微服务架构中，“节点”是一个逻辑概念，有可能是一台服务器（一台服务器上面运行一组微服务），
  也有可能是一组服务器组成的集群，同时 CITA 还支持部署在云服务器上，充分利用了各种服务器硬件来提升处理能力。
  节点与节点之间通过 P2P 通信，节点内部各模块通过消息总线通信。

![](https://github.com/citahub/citahub-docs/blob/master/docs/assets/cita-assets/architecture.jpg?raw=true)

- **组件可插拔**

  松耦合的微服务架构，便于各组件将来平滑迁移至更好的算法（比如新的共识算法）或者更好的技术方案（比如新的 DB 或者新的隐私方案）；
  也有利于针对一些具体的业务场景，定制一些特定的功能。

- **高性能**

  微服务架构将 Chain 与 Executor 独立出来，Executor 仅负责计算和执行交易，Chain 负责存储交易，
  使得计算和存储分离，极大程度的提高了交易处理能力；
  编程语言采用 Rust，Rust 强调并秉持零开销抽象的理念在提供诸多高级语言特性的同时，没有引入额外的开销，性能可以媲美 C++。
  最新版本的交易性能已经可以达到 15,000+ TPS（数据来自 CITA 0.16 版本，在四台 32 核，64G 的云服务器上部署 4 个节点，每台服务器配置百兆带宽）。

- **稳定可靠**

  CITA 提供快照工具来对区块链的数据进行备份，可在较短时间内恢复链数据。
  同时，Rust 借鉴了编程语言领域最新的研究成果，针对 C++ 中最头疼的内存问题（内存泄漏，野指针）进行编译器静态检查。
  只要代码编译通过，就可以保证没有以上问题，大大提高了应用运行阶段的可靠性。

- **兼容性**

  CITA上支持使用 Solidity，Go 语言，Rust 开发智能合约，同时也支持以太坊的所有开发工具（Truffle，Zeppeling，Remix 等）。

- **跨链**

  在区块链世界里，各种各样的链在不断的涌现出来。这些链如何互相配合形成区块链网络？
  CITA 目前提供了一个简单的跨链协议来支持主链与侧链之间的通信。我们也正对跨链通信做更多的探索，旨在扩大在各种链上运行的应用程序的价值。

- **工程经验**

  CITA 现在已经在银行，证券，票据等实际生产环境中运行，这其中我们积累了大量工程经验。

## 白皮书

- [英文版](https://github.com/citahub/cita-whitepaper/blob/master/en/technical-whitepaper.md)
- [中文版](https://github.com/citahub/cita-whitepaper/blob/master/zh/technical-whitepaper.md)

## 文档

- [英文版](https://docs.citahub.com/en-US/cita/cita-intro)
- [中文版](https://docs.citahub.com/zh-CN/cita/cita-intro)

## API/SDK

CITA 支持 JSON-RPC 和 WebSocket (experimental) API/v1。

对于 CITA 的 API/v1，你可以使用任意的 HTTP 代理，或者下面的 SDK：

* [Java](https://github.com/citahub/cita-sdk-java)
* [JavaScript](https://github.com/citahub/cita-sdk-js)
* [Swift](https://github.com/citahub/cita-sdk-swift)
* [Ruby](https://github.com/citahub/cita-sdk-ruby)
* [Rust](https://github.com/citahub/cita-common/tree/develop/cita-web3)

## 社区贡献

CITA 目前仍在紧张的开发中，欢迎大家为 CITA 贡献自己的一份力量。更详细的信息可以参考[贡献指南](.github/CONTRIBUTING.md)。

## 关注我们

[Weibo](http://weibo.com/u/6307204864)

## 开源协议 [![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fcitahub%2Fcita.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fcitahub%2Fcita?ref=badge_shield)

Apache 2.0 license

## 权益归属

<img src="https://github.com/citahub/assets/blob/master/rivtower-logo-square.png?raw=true" width="256">

秘猿科技团队 :heart:

## 联系我们

邮箱： <contact@rivtower.com>

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/kaikai1024"><img src="https://avatars0.githubusercontent.com/u/8768261?v=4" width="50px;" alt="kaikai"/><br /><sub><b>kaikai</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=kaikai1024" title="Code">💻</a></td>
    <td align="center"><a href="https://twitter.com/zhangyaning1985"><img src="https://avatars0.githubusercontent.com/u/161756?v=4" width="50px;" alt="zhangyaning"/><br /><sub><b>zhangyaning</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=u2" title="Code">💻</a></td>
    <td align="center"><a href="https://yangby-cryptape.github.io/"><img src="https://avatars1.githubusercontent.com/u/30993023?v=4" width="50px;" alt="Boyu Yang"/><br /><sub><b>Boyu Yang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=yangby-citahub" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rink1969"><img src="https://avatars1.githubusercontent.com/u/1633038?v=4" width="50px;" alt="zhiwei"/><br /><sub><b>zhiwei</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=rink1969" title="Code">💻</a></td>
    <td align="center"><a href="https://www.driftluo.com"><img src="https://avatars3.githubusercontent.com/u/19374080?v=4" width="50px;" alt="漂流"/><br /><sub><b>漂流</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=driftluo" title="Code">💻</a></td>
    <td align="center"><a href="https://ouwenkg.github.io/"><img src="https://avatars0.githubusercontent.com/u/11801722?v=4" width="50px;" alt="AsceticBear"/><br /><sub><b>AsceticBear</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=ouwenkg" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jerry-yu"><img src="https://avatars2.githubusercontent.com/u/2151472?v=4" width="50px;" alt="yubo"/><br /><sub><b>yubo</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=jerry-yu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/zhouyun-zoe"><img src="https://avatars0.githubusercontent.com/u/36949326?v=4" width="50px;" alt="zhouyun-zoe"/><br /><sub><b>zhouyun-zoe</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=zhouyun-zoe" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/volzkzg"><img src="https://avatars2.githubusercontent.com/u/2860864?v=4" width="50px;" alt="Bicheng Gao"/><br /><sub><b>Bicheng Gao</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=volzkzg" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/EighteenZi"><img src="https://avatars3.githubusercontent.com/u/31607114?v=4" width="50px;" alt="lhf"/><br /><sub><b>lhf</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=EighteenZi" title="Code">💻</a></td>
    <td align="center"><a href="http://ahorn.me"><img src="https://avatars0.githubusercontent.com/u/1160419?v=4" width="50px;" alt="LinFeng Qian"/><br /><sub><b>LinFeng Qian</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=TheWaWaR" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/keroro520"><img src="https://avatars3.githubusercontent.com/u/1870648?v=4" width="50px;" alt="keroro"/><br /><sub><b>keroro</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=keroro520" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/leeyr338"><img src="https://avatars3.githubusercontent.com/u/38514341?v=4" width="50px;" alt="Yaorong"/><br /><sub><b>Yaorong</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=leeyr338" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/suyanlong"><img src="https://avatars2.githubusercontent.com/u/16421423?v=4" width="50px;" alt="suyanlong"/><br /><sub><b>suyanlong</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=suyanlong" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Keith-CY"><img src="https://avatars1.githubusercontent.com/u/7271329?v=4" width="50px;" alt="Chen Yu"/><br /><sub><b>Chen Yu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=Keith-CY" title="Code">💻</a></td>
    <td align="center"><a href="https://zhangsoledad.github.io/salon"><img src="https://avatars2.githubusercontent.com/u/3198439?v=4" width="50px;" alt="zhangsoledad"/><br /><sub><b>zhangsoledad</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=zhangsoledad" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/hezhengjun"><img src="https://avatars0.githubusercontent.com/u/30688033?v=4" width="50px;" alt="hezhengjun"/><br /><sub><b>hezhengjun</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=hezhengjun" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/zeroqn"><img src="https://avatars0.githubusercontent.com/u/23418132?v=4" width="50px;" alt="zeroqn"/><br /><sub><b>zeroqn</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=zeroqn" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/urugang"><img src="https://avatars1.githubusercontent.com/u/11461821?v=4" width="50px;" alt="urugang"/><br /><sub><b>urugang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=urugang" title="Code">💻</a></td>
    <td align="center"><a href="https://justjjy.com"><img src="https://avatars0.githubusercontent.com/u/1695400?v=4" width="50px;" alt="Jiang Jinyang"/><br /><sub><b>Jiang Jinyang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=jjyr" title="Code">💻</a></td>
    <td align="center"><a href="https://twitter.com/janhxie"><img src="https://avatars0.githubusercontent.com/u/5958?v=4" width="50px;" alt="Jan Xie"/><br /><sub><b>Jan Xie</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=janx" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jerry-sl"><img src="https://avatars0.githubusercontent.com/u/5476062?v=4" width="50px;" alt="Sun Lei"/><br /><sub><b>Sun Lei</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=jerry-sl" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/chuchenxihyl"><img src="https://avatars1.githubusercontent.com/u/23721562?v=4" width="50px;" alt="hyl"/><br /><sub><b>hyl</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=chuchenxihyl" title="Code">💻</a></td>
    <td align="center"><a href="http://terrytai.me"><img src="https://avatars3.githubusercontent.com/u/5960?v=4" width="50px;" alt="Terry Tai"/><br /><sub><b>Terry Tai</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=poshboytl" title="Code">💻</a></td>
    <td align="center"><a href="https://bll.io"><img src="https://avatars0.githubusercontent.com/u/9641495?v=4" width="50px;" alt="Ke Wang"/><br /><sub><b>Ke Wang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=kilb" title="Code">💻</a></td>
    <td align="center"><a href="http://accu.cc"><img src="https://avatars3.githubusercontent.com/u/12387889?v=4" width="50px;" alt="Mohanson"/><br /><sub><b>Mohanson</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=mohanson" title="Code">💻</a></td>
    <td align="center"><a href="https://www.jianshu.com/u/3457636b07c5"><img src="https://avatars3.githubusercontent.com/u/17267434?v=4" width="50px;" alt="quanzhan lu"/><br /><sub><b>quanzhan lu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=luqz" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/duanyytop"><img src="https://avatars1.githubusercontent.com/u/5823268?v=4" width="50px;" alt="duanyytop"/><br /><sub><b>duanyytop</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=duanyytop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/clearloop"><img src="https://avatars0.githubusercontent.com/u/26088946?v=4" width="50px;" alt="clearloop"/><br /><sub><b>clearloop</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=clearloop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/hot3246624"><img src="https://avatars3.githubusercontent.com/u/9135770?v=4" width="50px;" alt="nokodemion"/><br /><sub><b>nokodemion</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=hot3246624" title="Code">💻</a></td>
    <td align="center"><a href="http://rainchen.com"><img src="https://avatars0.githubusercontent.com/u/71397?v=4" width="50px;" alt="Rain Chen"/><br /><sub><b>Rain Chen</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=rainchen" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/daogangtang"><img src="https://avatars2.githubusercontent.com/u/629594?v=4" width="50px;" alt="Daogang Tang"/><br /><sub><b>Daogang Tang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=daogangtang" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jiangxianliang007"><img src="https://avatars1.githubusercontent.com/u/24754263?v=4" width="50px;" alt="xianliang jiang"/><br /><sub><b>xianliang jiang</b></sub></a><br /><a href="https://github.com/citahub/cita/issues?q=author%3Ajiangxianliang007" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/vinberm"><img src="https://avatars0.githubusercontent.com/u/17666225?v=4" width="50px;" alt="Nov"/><br /><sub><b>Nov</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=vinberm" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rairyx"><img src="https://avatars2.githubusercontent.com/u/5009854?v=4" width="50px;" alt="Rai Yang"/><br /><sub><b>Rai Yang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=rairyx" title="Code">💻</a></td>
    <td align="center"><a href="http://www.huwenchao.com/"><img src="https://avatars0.githubusercontent.com/u/1630721?v=4" width="50px;" alt="Wenchao Hu"/><br /><sub><b>Wenchao Hu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=huwenchao" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/Kayryu"><img src="https://avatars1.githubusercontent.com/u/35792093?v=4" width="50px;" alt="kaiyu"/><br /><sub><b>kaiyu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=Kayryu" title="Code">💻</a></td>
    <td align="center"><a href="https://ashchan.com"><img src="https://avatars2.githubusercontent.com/u/1391?v=4" width="50px;" alt="James Chen"/><br /><sub><b>James Chen</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=ashchan" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rev-chaos"><img src="https://avatars1.githubusercontent.com/u/32355308?v=4" width="50px;" alt="rev-chaos"/><br /><sub><b>rev-chaos</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=rev-chaos" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/KaoImin"><img src="https://avatars1.githubusercontent.com/u/24822778?v=4" width="50px;" alt="Eason Gao"/><br /><sub><b>Eason Gao</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=KaoImin" title="Code">💻</a></td>
    <td align="center"><a href="http://qinix.com"><img src="https://avatars1.githubusercontent.com/u/1946663?v=4" width="50px;" alt="Eric Zhang"/><br /><sub><b>Eric Zhang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=qinix" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jasl"><img src="https://avatars2.githubusercontent.com/u/1024162?v=4" width="50px;" alt="Jun Jiang"/><br /><sub><b>Jun Jiang</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=jasl" title="Code">💻</a></td>
    <td align="center"><a href="https://blog.priewienv.me"><img src="https://avatars1.githubusercontent.com/u/9765170?v=4" width="50px;" alt="PRIEWIENV"/><br /><sub><b>PRIEWIENV</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=PRIEWIENV" title="Code">💻</a></td>
    <td align="center"><a href="https://gitter.im"><img src="https://avatars2.githubusercontent.com/u/8518239?v=4" width="50px;" alt="The Gitter Badger"/><br /><sub><b>The Gitter Badger</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=gitter-badger" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/classicalliu"><img src="https://avatars3.githubusercontent.com/u/13375784?v=4" width="50px;" alt="CL"/><br /><sub><b>CL</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=classicalliu" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/programmer-liu"><img src="https://avatars2.githubusercontent.com/u/25048144?v=4" width="50px;" alt="programmer-liu"/><br /><sub><b>programmer-liu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=programmer-liu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/yejiayu"><img src="https://avatars3.githubusercontent.com/u/10446547?v=4" width="50px;" alt="Jiayu Ye"/><br /><sub><b>Jiayu Ye</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=yejiayu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/QingYanL"><img src="https://avatars3.githubusercontent.com/u/48231505?v=4" width="50px;" alt="liyanzi"/><br /><sub><b>liyanzi</b></sub></a><br /><a href="https://github.com/citahub/cita/issues?q=author%3AQingYanL" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/YUJIAYIYIYI"><img src="https://avatars0.githubusercontent.com/u/40654430?v=4" width="50px;" alt="JiaYi"/><br /><sub><b>JiaYi</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=YUJIAYIYIYI" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/timmyz"><img src="https://avatars0.githubusercontent.com/u/795528?v=4" width="50px;" alt="Timmy Zhang"/><br /><sub><b>Timmy Zhang</b></sub></a><br /><a href="#ideas-timmyz" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="https://github.com/wuyuyue"><img src="https://avatars3.githubusercontent.com/u/40381396?v=4" width="50px;" alt="Wu Yuyue"/><br /><sub><b>Wu Yuyue</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=wuyuyue" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/xiangmeiLu"><img src="https://avatars2.githubusercontent.com/u/30581938?v=4" width="50px;" alt="xiangmeiLu"/><br /><sub><b>xiangmeiLu</b></sub></a><br /><a href="https://github.com/citahub/cita/commits?author=xiangmeiLu" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/mingxiaowu"><img src="https://avatars0.githubusercontent.com/u/42978282?v=4" width="50px;" alt="mingxiaowu"/><br /><sub><b>mingxiaowu</b></sub></a><br /><a href="https://github.com/citahub/cita/issues?q=author%3Amingxiaowu" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/wangfh666"><img src="https://avatars0.githubusercontent.com/u/41322861?s=400&v=4" width="50px;" alt="wangfh666"/><br /><sub><b>wangfh666</b></sub></a><br /><a href="https://github.com/citahub/cita/issues?q=author%3Awangfh666" title="Bug reports">🐛</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
