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

* Parameters

    `address` - The new node address

* Returns

    `bool` - True, if successfully, otherwise false

* Example

```shell
$ scm NodeManager approveNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### deleteNode

删除共识节点。

* Parameters

    `address` - The node address

* Returns

    `bool` - True, if successfully, otherwise false

* Example

```shell
$ scm NodeManager deleteNode \
        --address 0x59a316df602568957f47973332f1f85ae1e2e75e \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### listNode

共识节点列表。

* Parameters

    `None`

* Returns

    `address[]` - The consensus nodes

* Example

```shell
$ scm NodeManager listNode
```

### setStake

设置共识节点 stake 。

* Parameters

    `address` - The node address to be setted
    `uint64` - The stake to be setted

* Returns

    `bool` - True, if successfully, otherwise false

* Example

```shell
$ scm NodeManager setStake \
        --address 0xae0f69a2d95146d104365e0502a0d521717ced7f \
        --stake 0000000000000000000000000000000000000000000000000000000000000002 \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

### getStatus

获取共识节点状态。

* Parameters

    `address` - The node address

* Returns

    `uint8` - 0: closed, 1: started

* Example

```shell
$ scm NodeManager getStatus --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```

### listStake

共识节点 stake 列表。

* Parameters

    None

* Returns

    `uint64[]` - The node stakes list

* Example

```shell
$ scm NodeManager listStake
```

### stakePermillage

共识节点出块权重千分比。

* Parameters

    `address` - The node address

* Returns

    `uint64` - The node stake permillage

* Example

```shell
$ scm NodeManager stakePermillage --address 0xae0f69a2d95146d104365e0502a0d521717ced7f
```
