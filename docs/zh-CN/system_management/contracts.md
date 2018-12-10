# 系统合约

CITA 链生成时，通过系统合约来生成创世块，并作为链的最基本配置。拥有权限的管理员可以发送交易修改创世块的部分配置，所以了解系统合约至关重要。
你可以在 `/scripts/contracts/src` 目录下查看所有的系统合约，当然，接下来我们会一一解释。

另外在 `test-chain/template/contracts/docs` 目录（`test-chain` 为默认链名称）提供了所有系统合约函数签名，感兴趣的朋友可以自行查阅。

<h2 class="hover-list">系统合约</h2>

* [节点管理](#节点管理)
* [配额管理](#配额管理)
* [配额价格管理](#配额价格管理)
* [权限管理](#权限管理)
* [用户管理](#用户管理)
* [批量交易](#批量交易)
* [紧急制动](#紧急制动)
* [协议号管理](#协议号管理)
* [自动执行](#自动执行)

***

### 节点管理

按照快速搭裢的步骤，生成的链默认包含四个节点。如果你需要增加或是删除节点的话，管理员可以通过发送交易来做自定义配置。

节点管理合约存放在`/scripts/contracts/src/system/node_manager.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff020001`

节点管理的相关描述及方法介绍见 [node_manager](./system_management/node)

### 配额管理

和以太坊消耗 `gas` 不一样， CITA 中消耗 `quota`, 我们把它称作 `配额`，但是作用是类似的， 你可以把 `quota` 视为 `CITA` 的 `gas`。
CITA 中关于配额，有两个限制 BQL(BlockQuotaLimit) 和 AQL(AccountQuotaLimit)，分别表示区块的配额上限和用户配额上限。管理员可以通过发交易给
配额管理合约来做自定义修改。

配额管理合约存放在 `/scripts/contracts/src/system/quota_manager.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff020003`

配额管理的相关描述及方法介绍见 [配额管理](./system_management/quota)

### 配额价格管理

通过上边的讲述，我们已经清楚的知道配额的含义。CITA 类似于高速行驶的汽车，那么 `quota` 就是消耗的汽油，当然 `quota` 也是有价格的，我们用 `quotaPrice` 来表示它。幸运的是，管理员可以通过发送交易给配额价格管理系统合约来做自定义修改。

配额管理合约存放在 `/scripts/contracts/src/system/price_management.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff020010`

配额价格管理的相关描述及方法介绍见 [配额价格管理](./system_management/price)

### 权限管理

CITA 是一个面向企业级应用的区块链平台，严格的权限管理必不可少。我们提供了完整的权限管理接口，覆盖了企业级应用最常见的权限场景。

权限管理合约存放在 `/scripts/contracts/src/system/permission_management.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff020004`

权限管理的相关描述及方法介绍见 [权限管理](./system_management/permission)

### 用户管理

CITA 为了方便对用户的管理， 我们采用基于组的管理方式，管理员可以选择对组，对组内用户进行增删改查的灵活管理。

组管理合约存放在 `/scripts/contracts/src/user_management/group_management.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff02000a`

组用户管理合约存放在 `/scripts/contracts/src/user_management/group.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff020009`

用户管理的相关描述及方法介绍见 [用户管理](./system_management/user)

### 批量交易

CITA 支持批量调用合约。

批量交易合约存放在 `/scripts/contracts/src/system/batch_tx.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff02000e`

批量交易的相关描述及方法介绍见 [批量交易](./system_management/batch_tx)

### 紧急制动

在极端情况下，管理员可以通过发送交易到紧急制动系统合约，开启紧急制动模式，只接受管理员发送的交易，屏蔽掉其他所有交易。

紧急制动合约存放在 `/scripts/contracts/src/system/emergency_brake.sol`， 地址是 `0xffffffffffffffffffffffffffffffffff02000f`

紧急制动相关描述及方法介绍见 [紧急制动](./system_management/emergency_brake)

### 协议号管理

自 CITA 诞生以来，我们致力于研发成熟稳定，功能健全的区块链平台。CITA 的性能，功能上依旧在快速迭代，考虑到未来可能存在的兼容性问题，减少对现有客户的影响，我们增加了协议号管理系统合约。

协议号管理系统合约存放在 `/scripts/contracts/src/system/version_manager.sol`， 地址是　`0xffffffffffffffffffffffffffffffffff020011`

协议号管理的相关描述及方法介绍见 [协议号管理](./system_management/version)

### 自动执行

CITA 提供一种仅供管理员使用的交易自动执行的功能。当打开自动执行开关时，管理员就可以注册一个已部署合约的函数，其在每一个块中都会自动执行。

自动执行系统合约存放在 `/scripts/contracts/src/system/auto_exec.sol`， 地址是　`0xffffffffffffffffffffffffffffffffff020013`

关描述及方法介绍见 [自动执行](./system_management/auto_exec)
