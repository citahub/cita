# <img src="https://github.com/cryptape/assets/blob/master/CITA-logo.png?raw=true" width="256">

[![Circle CI Status](https://circleci.com/gh/cryptape/cita.svg?branch=develop)](https://circleci.com/gh/cryptape/cita)

[![All Contributors](https://img.shields.io/badge/all_contributors-54-orange.svg?style=flat-square)](#contributors)

English | [简体中文](./README-CN.md)

## What is CITA

CITA is a fast and scalable blockchain kernel for enterprises. CITA supports both native contract and EVM contract, by which enterprise users can build their own blockchain applications. CITA has a unique architecture which enables enterprise users to release all their computing resources.

- **Horizontal scalability**: With the microservice architecture, a logical node can be easily scaled to a cluster of servers. Node administrators can increase system capacity simply by adding more PC servers on high load. The administrator can even use dedicated servers to provide services for hot-spot accounts. Outside one node's boundary, nodes communicate with each other using P2P network; inside each node, microservices communicate with each other by messaging queue.

![](https://github.com/cryptape/citahub-docs/blob/master/docs/assets/cita-assets/architecture.jpg?raw=true)

- **Customizable and Pluggable Components**: CITA's microservices are loosely coupled and their communications are only via the message queue. Hence, it‘s flexible to improve current components with better algorithms (such as new consensus algorithms) or more appropriate technical solutions (such as new DBs or new privacy solutions). Moreover, business logic is extremely complicated in enterprise applications. With CITA, you can easily customize your blockchain with the certain feature to fit your own business requirements.

- **High Performance**: In CITA, consensus and transaction execution are decoupled as separate microservices. The consensus service is only responsible for transaction ordering, which can finish independently before transaction execution, thus increase transaction processing performance. In additional, CITA also includes lots of other optimizations to fully utilize multi-cores and multi-servers' computing power. To this end, it utilizes the Rust language, a hybrid imperative/OO/functional language with an emphasis on efficiency.

- **Resiliency and Reliability**: CITA provides tools to backup blockchain data by taking snapshot, which can help you to resync the blockchain data in a short time. And through Rust’s language-level memory and thread guarantees and a disciplined approach to exception-handling, we can state with a high degree of certainty that our code cannot crash, hang or bomb-out unexpectedly.

- **Compatibility**: CITA supports the use of Solidity, Go, and Rust to develop smart contracts. It also supports all Ethereum development tools (Truffle, Zeppelin, Remix, etc.).

- **Chain Interoperability**: We perceive that independent blockchains are constantly emerging nowadays and even more in the future. How do these chains interoperate with each other to form blockchain network? CITA Support cross-chain communication by providing a simple cross-chain protocol currently. More explorations are undertaking in CITA, aiming to amplify the value of applications running on the various chains.

- **Engineering Experience**: There're many CITA networks running in banks, payment and insurance production environment, with Cryptape or CITA Integration Provider's technical support.  CITA has accumulated a lot of engineering experience.

## Whitepaper

For more details please check the white paper.

- [English](https://github.com/cryptape/cita-whitepaper/blob/master/en/technical-whitepaper.md)
- [Chinese](https://github.com/cryptape/cita-whitepaper/blob/master/zh/technical-whitepaper.md)

## Document

- [English](https://docs.citahub.com/en-US/cita/cita-intro)
- [Chinese](https://docs.citahub.com/zh-CN/cita/cita-intro)

## API/SDK

CITA supports JSON-RPC and WebSocket (experimental) API/v1.

For CITA API/v1, You can use any HTTP client, or following SDKs:

* [Java](https://github.com/cryptape/cita-sdk-java)
* [JavaScript](https://github.com/cryptape/cita-sdk-js)
* [Swift](https://github.com/cryptape/cita-sdk-swift)
* [Ruby](https://github.com/cryptape/cita-sdk-ruby)
* [Rust](https://github.com/cryptape/cita-common/tree/develop/cita-web3)

## Contributing

CITA is still in active development. Building a blockchain platform is a huge task, we need your help. Any contribution is welcome.

Please check [CONTRIBUTING](.github/CONTRIBUTING.md) for more details.

## Follow us

[Twitter](https://twitter.com/Cryptape)

[Weibo](http://weibo.com/u/6307204864)

## License

CITA is currently under the GPLv3 license. See the LICENSE file for details.

**CITA will move to Apache 2.0 license soon.**

## Credits

<img src="https://github.com/cryptape/assets/blob/master/cryptape-logo-square.png?raw=true" width="256">

CITA is created by Cryptape team with :heart:.

## Contact us

Email: <contact@cryptape.com>

## Contributors

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore -->
<table>
  <tr>
    <td align="center"><a href="https://github.com/kaikai1024"><img src="https://avatars0.githubusercontent.com/u/8768261?v=4" width="50px;" alt="kaikai"/><br /><sub><b>kaikai</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=kaikai1024" title="Code">💻</a></td>
    <td align="center"><a href="https://twitter.com/zhangyaning1985"><img src="https://avatars0.githubusercontent.com/u/161756?v=4" width="50px;" alt="zhangyaning"/><br /><sub><b>zhangyaning</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=u2" title="Code">💻</a></td>
    <td align="center"><a href="https://yangby-cryptape.github.io/"><img src="https://avatars1.githubusercontent.com/u/30993023?v=4" width="50px;" alt="Boyu Yang"/><br /><sub><b>Boyu Yang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=yangby-cryptape" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rink1969"><img src="https://avatars1.githubusercontent.com/u/1633038?v=4" width="50px;" alt="zhiwei"/><br /><sub><b>zhiwei</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=rink1969" title="Code">💻</a></td>
    <td align="center"><a href="https://www.driftluo.com"><img src="https://avatars3.githubusercontent.com/u/19374080?v=4" width="50px;" alt="漂流"/><br /><sub><b>漂流</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=driftluo" title="Code">💻</a></td>
    <td align="center"><a href="https://ouwenkg.github.io/"><img src="https://avatars0.githubusercontent.com/u/11801722?v=4" width="50px;" alt="AsceticBear"/><br /><sub><b>AsceticBear</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=ouwenkg" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jerry-yu"><img src="https://avatars2.githubusercontent.com/u/2151472?v=4" width="50px;" alt="yubo"/><br /><sub><b>yubo</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=jerry-yu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/zhouyun-zoe"><img src="https://avatars0.githubusercontent.com/u/36949326?v=4" width="50px;" alt="zhouyun-zoe"/><br /><sub><b>zhouyun-zoe</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=zhouyun-zoe" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/volzkzg"><img src="https://avatars2.githubusercontent.com/u/2860864?v=4" width="50px;" alt="Bicheng Gao"/><br /><sub><b>Bicheng Gao</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=volzkzg" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/EighteenZi"><img src="https://avatars3.githubusercontent.com/u/31607114?v=4" width="50px;" alt="lhf"/><br /><sub><b>lhf</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=EighteenZi" title="Code">💻</a></td>
    <td align="center"><a href="http://ahorn.me"><img src="https://avatars0.githubusercontent.com/u/1160419?v=4" width="50px;" alt="LinFeng Qian"/><br /><sub><b>LinFeng Qian</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=TheWaWaR" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/keroro520"><img src="https://avatars3.githubusercontent.com/u/1870648?v=4" width="50px;" alt="keroro"/><br /><sub><b>keroro</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=keroro520" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/leeyr338"><img src="https://avatars3.githubusercontent.com/u/38514341?v=4" width="50px;" alt="Yaorong"/><br /><sub><b>Yaorong</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=leeyr338" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/suyanlong"><img src="https://avatars2.githubusercontent.com/u/16421423?v=4" width="50px;" alt="suyanlong"/><br /><sub><b>suyanlong</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=suyanlong" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Keith-CY"><img src="https://avatars1.githubusercontent.com/u/7271329?v=4" width="50px;" alt="Chen Yu"/><br /><sub><b>Chen Yu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=Keith-CY" title="Code">💻</a></td>
    <td align="center"><a href="https://zhangsoledad.github.io/salon"><img src="https://avatars2.githubusercontent.com/u/3198439?v=4" width="50px;" alt="zhangsoledad"/><br /><sub><b>zhangsoledad</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=zhangsoledad" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/hezhengjun"><img src="https://avatars0.githubusercontent.com/u/30688033?v=4" width="50px;" alt="hezhengjun"/><br /><sub><b>hezhengjun</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=hezhengjun" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/zeroqn"><img src="https://avatars0.githubusercontent.com/u/23418132?v=4" width="50px;" alt="zeroqn"/><br /><sub><b>zeroqn</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=zeroqn" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/urugang"><img src="https://avatars1.githubusercontent.com/u/11461821?v=4" width="50px;" alt="urugang"/><br /><sub><b>urugang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=urugang" title="Code">💻</a></td>
    <td align="center"><a href="https://justjjy.com"><img src="https://avatars0.githubusercontent.com/u/1695400?v=4" width="50px;" alt="Jiang Jinyang"/><br /><sub><b>Jiang Jinyang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=jjyr" title="Code">💻</a></td>
    <td align="center"><a href="https://twitter.com/janhxie"><img src="https://avatars0.githubusercontent.com/u/5958?v=4" width="50px;" alt="Jan Xie"/><br /><sub><b>Jan Xie</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=janx" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jerry-sl"><img src="https://avatars0.githubusercontent.com/u/5476062?v=4" width="50px;" alt="Sun Lei"/><br /><sub><b>Sun Lei</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=jerry-sl" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/chuchenxihyl"><img src="https://avatars1.githubusercontent.com/u/23721562?v=4" width="50px;" alt="hyl"/><br /><sub><b>hyl</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=chuchenxihyl" title="Code">💻</a></td>
    <td align="center"><a href="http://terrytai.me"><img src="https://avatars3.githubusercontent.com/u/5960?v=4" width="50px;" alt="Terry Tai"/><br /><sub><b>Terry Tai</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=poshboytl" title="Code">💻</a></td>
    <td align="center"><a href="https://bll.io"><img src="https://avatars0.githubusercontent.com/u/9641495?v=4" width="50px;" alt="Ke Wang"/><br /><sub><b>Ke Wang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=kilb" title="Code">💻</a></td>
    <td align="center"><a href="http://accu.cc"><img src="https://avatars3.githubusercontent.com/u/12387889?v=4" width="50px;" alt="Mohanson"/><br /><sub><b>Mohanson</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=mohanson" title="Code">💻</a></td>
    <td align="center"><a href="https://www.jianshu.com/u/3457636b07c5"><img src="https://avatars3.githubusercontent.com/u/17267434?v=4" width="50px;" alt="quanzhan lu"/><br /><sub><b>quanzhan lu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=luqz" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/duanyytop"><img src="https://avatars1.githubusercontent.com/u/5823268?v=4" width="50px;" alt="duanyytop"/><br /><sub><b>duanyytop</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=duanyytop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/clearloop"><img src="https://avatars0.githubusercontent.com/u/26088946?v=4" width="50px;" alt="clearloop"/><br /><sub><b>clearloop</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=clearloop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/hot3246624"><img src="https://avatars3.githubusercontent.com/u/9135770?v=4" width="50px;" alt="nokodemion"/><br /><sub><b>nokodemion</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=hot3246624" title="Code">💻</a></td>
    <td align="center"><a href="http://rainchen.com"><img src="https://avatars0.githubusercontent.com/u/71397?v=4" width="50px;" alt="Rain Chen"/><br /><sub><b>Rain Chen</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=rainchen" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/daogangtang"><img src="https://avatars2.githubusercontent.com/u/629594?v=4" width="50px;" alt="Daogang Tang"/><br /><sub><b>Daogang Tang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=daogangtang" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jiangxianliang007"><img src="https://avatars1.githubusercontent.com/u/24754263?v=4" width="50px;" alt="xianliang jiang"/><br /><sub><b>xianliang jiang</b></sub></a><br /><a href="https://github.com/cryptape/cita/issues?q=author%3Ajiangxianliang007" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/vinberm"><img src="https://avatars0.githubusercontent.com/u/17666225?v=4" width="50px;" alt="Nov"/><br /><sub><b>Nov</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=vinberm" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rairyx"><img src="https://avatars2.githubusercontent.com/u/5009854?v=4" width="50px;" alt="Rai Yang"/><br /><sub><b>Rai Yang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=rairyx" title="Code">💻</a></td>
    <td align="center"><a href="http://www.huwenchao.com/"><img src="https://avatars0.githubusercontent.com/u/1630721?v=4" width="50px;" alt="Wenchao Hu"/><br /><sub><b>Wenchao Hu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=huwenchao" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/Kayryu"><img src="https://avatars1.githubusercontent.com/u/35792093?v=4" width="50px;" alt="kaiyu"/><br /><sub><b>kaiyu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=Kayryu" title="Code">💻</a></td>
    <td align="center"><a href="https://ashchan.com"><img src="https://avatars2.githubusercontent.com/u/1391?v=4" width="50px;" alt="James Chen"/><br /><sub><b>James Chen</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=ashchan" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rev-chaos"><img src="https://avatars1.githubusercontent.com/u/32355308?v=4" width="50px;" alt="rev-chaos"/><br /><sub><b>rev-chaos</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=rev-chaos" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/KaoImin"><img src="https://avatars1.githubusercontent.com/u/24822778?v=4" width="50px;" alt="Eason Gao"/><br /><sub><b>Eason Gao</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=KaoImin" title="Code">💻</a></td>
    <td align="center"><a href="http://qinix.com"><img src="https://avatars1.githubusercontent.com/u/1946663?v=4" width="50px;" alt="Eric Zhang"/><br /><sub><b>Eric Zhang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=qinix" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/jasl"><img src="https://avatars2.githubusercontent.com/u/1024162?v=4" width="50px;" alt="Jun Jiang"/><br /><sub><b>Jun Jiang</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=jasl" title="Code">💻</a></td>
    <td align="center"><a href="https://blog.priewienv.me"><img src="https://avatars1.githubusercontent.com/u/9765170?v=4" width="50px;" alt="PRIEWIENV"/><br /><sub><b>PRIEWIENV</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=PRIEWIENV" title="Code">💻</a></td>
    <td align="center"><a href="https://gitter.im"><img src="https://avatars2.githubusercontent.com/u/8518239?v=4" width="50px;" alt="The Gitter Badger"/><br /><sub><b>The Gitter Badger</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=gitter-badger" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/classicalliu"><img src="https://avatars3.githubusercontent.com/u/13375784?v=4" width="50px;" alt="CL"/><br /><sub><b>CL</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=classicalliu" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/programmer-liu"><img src="https://avatars2.githubusercontent.com/u/25048144?v=4" width="50px;" alt="programmer-liu"/><br /><sub><b>programmer-liu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=programmer-liu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/yejiayu"><img src="https://avatars3.githubusercontent.com/u/10446547?v=4" width="50px;" alt="Jiayu Ye"/><br /><sub><b>Jiayu Ye</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=yejiayu" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/QingYanL"><img src="https://avatars3.githubusercontent.com/u/48231505?v=4" width="50px;" alt="liyanzi"/><br /><sub><b>liyanzi</b></sub></a><br /><a href="https://github.com/cryptape/cita/issues?q=author%3AQingYanL" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/YUJIAYIYIYI"><img src="https://avatars0.githubusercontent.com/u/40654430?v=4" width="50px;" alt="JiaYi"/><br /><sub><b>JiaYi</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=YUJIAYIYIYI" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/timmyz"><img src="https://avatars0.githubusercontent.com/u/795528?v=4" width="50px;" alt="Timmy Zhang"/><br /><sub><b>Timmy Zhang</b></sub></a><br /><a href="#ideas-timmyz" title="Ideas, Planning, & Feedback">🤔</a></td>
    <td align="center"><a href="https://github.com/wuyuyue"><img src="https://avatars3.githubusercontent.com/u/40381396?v=4" width="50px;" alt="Wu Yuyue"/><br /><sub><b>Wu Yuyue</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=wuyuyue" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/xiangmeiLu"><img src="https://avatars2.githubusercontent.com/u/30581938?v=4" width="50px;" alt="xiangmeiLu"/><br /><sub><b>xiangmeiLu</b></sub></a><br /><a href="https://github.com/cryptape/cita/commits?author=xiangmeiLu" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/mingxiaowu"><img src="https://avatars0.githubusercontent.com/u/42978282?v=4" width="50px;" alt="mingxiaowu"/><br /><sub><b>mingxiaowu</b></sub></a><br /><a href="https://github.com/cryptape/cita/issues?q=author%3Amingxiaowu" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/wangfh666"><img src="https://avatars0.githubusercontent.com/u/41322861?s=400&v=4" width="50px;" alt="wangfh666"/><br /><sub><b>wangfh666</b></sub></a><br /><a href="https://github.com/cryptape/cita/issues?q=author%3Awangfh666" title="Bug reports">🐛</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
