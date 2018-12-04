# 共识节点管理合约接口

<h2 class="hover-list">Node Management</h2>

* [approveNode](#approveNode)
* [deleteNode](#deleteNode)
* [listNode](#listNode)
* [setStake](#setStake)
* [getStatus](#getStatus)
* [listStake](#listStake)
* [stakePermillage](#stakePermillage)

***

### approveNode

确认共识节点。

* 参数

    `address` - The new node address

* 返回值

    `bool` - True, if successfully, otherwise false

* 示例

```shell
$ scm NodeManager approveNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### deleteNode

删除共识节点。

* 参数

    `address` - The node address

* 返回值

    `bool` - True, if successfully, otherwise false

* 示例

```shell
$ scm NodeManager deleteNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### listNode

共识节点列表。

* 参数

    空

* 返回值

    `address[]` - The consensus nodes

* 示例

```shell
$ scm NodeManager listNode
```

### setStake

设置共识节点 stake 。

* 参数

    `address` - The node address to be setted.

    `uint64` - The stake to be setted.

* 返回值

    `bool` - True, if successfully, otherwise false.

* 示例

```shell
$ scm NodeManager setStake \
        --address 0xae0f69a2d95146d104365e0502a0d521717ced7f \
        --stake 0000000000000000000000000000000000000000000000000000000000000002 \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### getStatus

获取共识节点状态。

* 参数

    `address` - The node address

* 返回值

    `uint8` - 0: closed, 1: started

* 示例

```shell
$ scm NodeManager getStatus --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```

### listStake

共识节点 stake 列表。

* 参数

    空

* 返回值

    `uint64[]` - The node stakes list

* 示例

```shell
$ scm NodeManager listStake
```

### stakePermillage

共识节点出块权重千分比。

* 参数

    `address` - The node address

* 返回值

    `uint64` - The node stake permillage

* 示例

```shell
$ scm NodeManager stakePermillage --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```
