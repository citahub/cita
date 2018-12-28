# 共识

CITA 的共识模块主要是保证多个节点对于交易的顺序和 Block 的内容达成一致。在众多的分布式算法中，我们实现了拜占庭容错的 CITA-BFT 共识算法。CITA-BFT 是一个两阶段投票的拜占庭容错算法。该算法主要在 Tendermint，Paxos 的基础上进行了优化，以更好地与 CITA 的微服务架构融合，利用微服务架构的横向扩展优势提升共识效率。在每个高度上，都有 N 个节点参与当前高度的共识，共识节点经过至少一轮的共识达到最终共识，当出现某些节点掉线或者网络故障等问题时，需要进行多轮共识以达到最终共识。

## 共识的架构

共识主要有MQ(消息队列)通讯模块、交易池、定时模块、WAL(write ahead log)、算法逻辑模块。

```
   +-------------+       +-------------+       +-----+
   | MQ通讯模块   |<----->|  算法逻辑模块  |<---->| WAL |
   +-------------+       +-------------+       +-----+
          ^                ^    ^
          |                |    |
          |----------------+    |
          |                     |
     +--------+           +-----------+
     | 交易池  |           | 定时模块  |
     +--------+           +-----------+
```

**MQ通讯模块**： CITA的消息通过MQ进行周转，MQ通讯模块负责订阅、发布基于MQ的消息。

**交易池**： 交易池订阅和存储交易信息，并提供交易的打包、生成Block。还进行交易的持久化，实现快速确认的功能。

**定时模块**： 提供算法定时服务。使用者向定时模块发送定时请求，定时模块在时间到达后，发送确认信息。

**WAL**： WAL提供预写日志(write ahead log)的服务，持久化各个节点的投票。用来进行节点崩溃后恢复。

**算法逻辑模块**： 分布式算法逻辑的实现模块，接受共识其它模块发送过来的信息，根据自身的算法要求，进行算法逻辑相应的处理。

## 基本前提

- 每个共识节点知道其它共识节点的公钥
- 每个共识节点发送的投票信息，都必须有自己的签名
- 共识节点根据公钥和签名可以验证消息的真实性
- 共识节点数量需要满足算法要求的基本数量

## 基本步骤

虽然分布式算法多种多样，具体落实在CITA中，基本上需要进行如下的步骤：

- 共识模块从消息队列中订阅交易信息，放入交易池。如果应用有快速确认的需求，交易池可以对交易进行持久化。
- 共识算法根据配置和算法要求，选择一个出块的节点。该出块节点把块的哈希值（Block.Hash）作为共识的主要信息通过MQ通讯模块向其它的节点进行广播。
- 当出块节点进过一轮或者多轮投票，收到算法要求的法定多数的投票返回时，向Chain模块确认出块，否则进入重新选择出块节点的计算，由下一个出块节点继续出块。
- 接收Chain返回的状态信息，作为出块成功的标志。
- 出块成功后，从交易池删除已经达成共识的交易。

## CITA-BFT 共识算法

CITA-BFT是一种专为区块链设计的高性能共识算法，基于半同步网络假设（*部分同步网络*：存在一个确定的消息传播时延上限 δ，但是 δ 未知；或者 δ 已知，但是只在某些未知的时间段才有该时延限制），CITA-BFT在保证活性和安全性（Liveness & Safety）的前提下能够容忍 1/3 的拜占庭节点。
CITA共识节点通过点对点共识消息交换协议对每一个区块交换投票信息，迅速形成多数共识。投票结果最后会被记录在区块里。CITA支持独有的低延迟技术，能够实现毫秒级交易确认延迟。

### 基本约定

1. 规定 H 为当前高度，R 为当前轮次，N 为该轮参与共识的节点数量，B 为当前区块，在每一个高度下，达成共识至少需要一轮；
2. 一个包含 +2/3 的对应于处在 <H, R> 的特定区块或者 nil（空区块）的预投票的集合，称之为锁变化证明(Proof-of-lock-change)，简称 PoLC。

### 状态

**Propose**：每个节点检查自己是否是 proposer。proposer 广播当前轮次的 proposal

**ProposeWait**：非提议节点对 proposal 进行基本检查并向 Auth 发送验证请求

**Prevote**：节点进行预投票

**PrevoteWait**：等待其他节点的预投票

**PrecommitAuth**：等待 Auth 返回对区块 B 的校验结果

**Precommit**：节点进行预提交

**PrecommitWait**：等待其他节点对特定 proposal 的预提交

**Commit**：提交区块 B 给 Chain 微服务和 Executor 微服务

**CommitWait**：等待 Chain 发来的最新状态（rich_status）消息

### 状态转换图
![state convert](/docs/_image/state_convert.jpg)

### 状态转换描述

#### PROPOSE <H, R> → PROPOSEWAIT <H, R>

新一轮开始时，共识节点处于 Propose<H, R> 状态，共识节点通过计算 (H+R) % N 确定本轮的 proposer<H, R>，接着重置并启动一个计时器 T0 （T0 = 3s） ：

* 如果该共识节点就是本轮的 proposer<H, R>，就广播这一轮的提议 proposal<H, R, B>
* 如果该共识节点不是本轮的 proposer<H, R>，就重置并启动一个计时器 T1（T1 = T0 * 24 / 30 * (R + 1) ）

共识节点进入 ProposeWait<H, R> 状态。

#### PROPOSEWAIT <H, R> → PREVOTE <H, R>

* 如果共识节点是 proposer<H, R> ，共识节点对自己发出的 proposal<H, R, B> 投 prevote<H, R, B>
* 如果共识节点不是 proposer<H, R> 且在 T1 内收到 proposal<H, R, B>，共识节点对该 proposal<H, R, B> 做基本检查
* * 如果 proposal<H, R, B> 通过了基本检查， 则向 Auth 发送请求验证 B 的合法性，共识节点对该  proposal<H, R, B> 投 prevote<H, R, B>
  * 如果 proposal<H, R, B> 没有通过基本检查，共识节点对 nil<H, R> 投 prevote<H, R, B>
* 如果共识节点不是 proposer<H, R> 且在 T1 内没有收到 proposer<H, R> 发出的 proposal<H, R, B>，共识节点对 nil<H, R> 投 prevote<H, R, P>

共识节点将 prevote<H, R, B> 投票保存到本地，并进入 Prevote<H, R> 状态。共识节点重置并启动一个计时器 T2 以重新广播 prevote<H, R, P> 投票。

#### PREVOTE<H, R> → PREVOTEWAIT<H, R>

共识节点收到 +2/3 的 prevote<H, R, P> 后, 进入 PrevoteWait<H, R> 状态，同时重置并启动一个计时器 T3（T3 = T0 * 1 / 30 = 0.1s）。

#### PREVOTEWAIT<H, R> → PRECOMMIT<H, R> 或者 PREVOTEWAIT<H, R> → PRECOMMITAUTH<H, R>

* 如果共识节点在 T3 内收到 +2/3 的共识节点对 proposal<H, R, B> 的 prevote<H, R, B>
* * 如果 Auth 对 B 的验证通过，共识节点对该 proposal<H, R, B> 投 precommit<H, R, B>，共识节点进入 Precommit<H, R> 状态
  * 如果 Auth 对 B 的验证不通过，共识节点对 nil<H,R> 投 precommit<H, R, B>，共识节点进入 Precommit<H, R> 状态
  * 如果 Auth 还没有返回对 B 的验证结果，共识节点重置并启动一个 T4（T4 = T0 * 1 / 30 * 15 = 1.5s）计时器，并进入 PrecommitAuth 状态
* 如果共识节点在 T3 内收到 +2/3 的共识节点对 nil<H, R> 的 prevote<H, R, P>，共识节点对 nil<H,R> 投 precommit<H, R, P>，共识节点进入 Precommit<H, R> 状态
* 如果共识节点在 T3 内没有满足以上条件，共识节点对 nil<H, R> 投 precommit<H, R, B>，共识节点进入 Precommit<H, R> 状态

如果共识节点投了 precommit<H, R, B>，便重置并启动一个 T5（T5 = T0 * 1 / 30 * 15 = 1.5s）计时器以重发 prevote<H, R, P> 和 precommit<H, R, P>。

#### PRECOMMITAUTH<H, R> → PRECOMMIT<H, R>

 如果共识节点在 T4 时间内没有收到 Auth 返回，就再次向 Auth 请求验证区块，并一直等待，直到收到 Auth 返回：

* 如果 Auth 对区块的验证通过，共识节点对 proposal<H, R, B> 投 precommit<H, R, P>，共识节点进入 Precommit<H, R> 状态
* 如果 Auth 对区块的验证不通过，共识节点对 nil<H, R> 投 prevote<H, R, B> 和 precommit<H, R, B>，共识节点进入Precommit<H, R> 状态

PRECOMMIT<H, R> → PRECOMMITWAIT<H, R>

当共识节点收到 +2/3 的 precommit<H, R, B> 后，进入 PrecommitWait<H, R> 状态，同时重置并启动一个计时器 T6（T6 = T0 * 1 / 30 = 0.1s）。

#### PRECOMMITWAIT<H, R> → COMMIT<H, R> 或者 PRECOMMITWAIT<H, R> → PREPARE<H, R + 1>

* 如果共识节点在 T6 内收到 +2/3 的共识节点对特定 proposal<H, R, B> 的 precommit<H, R, B> ，共识节点将区块 B 发送给 Executor 微服务和 Chain 微服务处理
* 如果共识节点在 T6 内收到 +2/3 的共识节点对 nil<H, R>  的 precommit<H, R, B> ，共识节点进入 Propose<H, R+1> 状态
* 如果共识节点在 T6 内没有满足以上条件，共识节点进入 Propose<H, R+1> 状态

#### COMMIT<H, R> → COMMITWAIT<H, R>

共识节点收到 Chain 发来的最新共识区块返回的消息 rich_status 后，进入 CommitWait<H, R> 状态。

#### COMMITWAIT<H, R> → PROPOSE<H + 1, 0>

共识节点等待 T0 超时，进入 Propose<H + 1, 0> 状态。

## 处理机制

### 常数

* INIT_HEIGHT ：初始块高度，设置为1
* INIT_ROUND ：初始轮次，设置为0
* TIMEOUT_RETRANSE_MULTIPLE ：超时重发常数，设置为15
* TIMEOUT_LOW_ROUND_MESSAGE_MULTIPLE ：向低轮次广播控制常数，设置为20
* THREAD_POOL_NUM ：线程池内线程数量，设置为10

### BLOCK 结构

* version ：版本号
* header ：BlockHeader 结构
* body ：BlockBody 结构

### BlockHeader 结构

* prevhash ：上一个块的哈希值
* timestamp ：Unix 时间戳
* proof ：Proof结构，出块人签名
* commit ：Commit 结构，Chain处理结果
* height ：uint64 块号

### BlockBody 结构

* transactions ：交易列表

### Commit 结构

* stateRoot ：状态 root
* transcationsRoot ：交易列表 root
* receiptsRoot ：交易回执 root

### BFT 结构

* pub_sender ：发送消息信道
* pub_recver ：接收消息信道
* timer_seter ：
* timer_notify ：
* params ：BFT 参数
* height ：当前高度
* round ：当前轮次
* step ：当前阶段
* proof ：投给特定 proposal 的投票的签名
* pre_hash ：上一个块的哈希
* votes ：投票集合
* proposals ：提议集合
* proposal ：提议的 hash
* lock_round ：锁定的轮次
* locked_vote ：锁定的投票
* locked_block ：锁定的区块
* wal_log ：日志
* send_filter ：投票查重
* last_commit_round ：上一次提交的轮次
* htime ：新的高度开始的时间
* auth_manage ：权限管理
* consensus_power ：是否可以参与共识
* unverified_message ：交易消息未确认块
* block_txs ：交易区块
* block_proof ：上一个高度的共识结果
* is_snapshot ：是否做了快照
* is_cleared ：数据是否被清理

### PROPOSER 选择

参与共识的节点通过计算 (H+R) % N 确定本轮的 proposer 。

### PROOF 生成

首先获取当前轮次下 precommit 投票集合并创建名为 commits 的 hashmap 。commits 收集与输入 hash 相同的投票，并计算投票数量。如果数量不满足 +2/3 则返回空，否则返回 proof。proof 的构成如下：

* proof.height ：proof 对应的高度
* proof.round ：proof 对应的轮次
* proof.proposal ：proof 对应的提议
* proof.commits ：投给 proof.proposal 的每一个投票的签名

### 并行处理

proof 是对链上区块合法性的证明，包含了当前高度 H 的区块 +2/3 的 precommit 。为了确保每个节点保存的 proof 一致，同时为了提升效率，CITA-BFT 将 commit 和 proposal 两个阶段做并行处理。当共识节点在 Commit<H - 1, R> 状态收到 Chain 微服务保存最新共识区块返回的消息后，生成 proof<H - 1> 并保存在本地，当共识节点进入 Propose<H, R'> 状态，且共识节点是 proposer<H, R'> 时，共识节点将 auth 提供的区块内容与 proof<H - 1> 一起组成区块放入 proposal 中广播出去。即高度为 H 的 proposer 区块的 proposal 中包含高度为 H - 1 的 proof。当该区块达成最终共识后，前一个区块的 proof 也同时完成了最终的共识。

### LOCK 机制

如果共识节点收到 +2/3 的对 proposal<H, R, B> 投票的 prevote<H, R, P>， 则该共识节点锁定该 proposal 的区块 B。如果该共识节点在同一高度更高轮次 R' 收到 +2/3 的对 proposal<H, R', B'> 投票的 prevote<H, R', P‘>，则该共识节点解锁区块 B，并锁定新的区块 B'。如果该共识节点在同一高度更高轮次 R' 收到 +2/3 的 对 nil<H, R'> 投票的 prevote<H, R', P’>，则该共识节点解锁之前锁定的区块 B。如果共识节点收到 auth 对锁定区块 B 的验证结果为不通过，则该共识节点解除区块 B 的锁定。

如果共识节点处于 Propose<H, R> 状态，且该节点是 proposer<H, R>，若该共识节点已锁定区块 B，则广播 proposal<H, R, B>，若该共识节点未锁定任何区块，则根据 Auth 提供的区块内容与 proof<H - 1> 组成区块 B，之后广播 proposal<H, R, B>。
如果共识节点已锁定区块 B，即使共识节点收到 proposal<H, R, B'>，仍然只对 B 投 prevote<H, R, P> 。

### 轮次跃迁机制

当共识节点状态低于 Precommit<H, R> 状态时，如果收到 +2/3 的对更高轮次 R' 的 proposal<H, R’, B> 投票的 prevote<H, R‘, P> 且 proposal<H, R’, B> 亦已收到且通过基本检查，则该共识节点锁定区块 B，并且节点的状态跃迁到  PrevoteWait<H, R'> 。

### CITA-BFT交易池操作流程

1. 交易池启动时，尝试从KV数据库恢复数据
2. 交易池订阅MQ的交易信息
3. 交易池收到交易后，持久化到KV数据库
4. 交易池收到打包请求，检查交易的有效性，输出有效交易列表
5. 交易池根据出块的交易列表，删除已经上链的交易

### CITA-BFT故障重启流程

1. 从WAL模块中，恢复某个块高度的投票信息
2. 根据恢复后的状态信息，重复投票信息
3. 进程根据当前状态，继续运行