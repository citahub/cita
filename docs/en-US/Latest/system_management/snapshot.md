# 快照

从字面上解释，“快照”这个词来源于照相、摄影等领域，后来延展到范围很广，当需要保留某件事物某个时间的状态时，就可以说对其做一次“快照”。

快照属于工具类的功能，不属于区块链本身的功能。对区块链某个高度的状态做快照，保存状态、区块等数据，然后将快照恢复到另一个节点/本节点，就可以在较短时间内同步/恢复区块链数据。

## 使用说明

snapshot_tool有两个功能：创建快照和快照恢复

- 创建快照是将当前区块链的状态等数据保存到文件中。
- 快照恢复是根据保存下来的文件将区块链数据恢复到创建快照时的状态。

命令格式如下：

```
snapshot_tool -m snapshot [-e HEIGHT]  [-f FILE]
snapshot_tool -m restore [-f FILE]
```

中括号中的选项是可选的。 

- -m是指定功能，快照或者恢复。
- -e是指定区块链高度，缺省值是区块链当前高度。
- -f 是指定快照文件的名字，如果指定为FILE，chain模块会生成FILE_chain.rlp，executor模块会生成FILE_executor.rlp，缺省时是snapshot_chain.rlp和snapshot_executor.rlp。

### 创建快照

以对100高度做快照为例（当前链高度需大于100）：

```
$ cd test-chain/0
$ ../../bin/snapshot_tool -m snapshot -e 100
$ ls snap*
snapshot_chain.rlp  snapshot_executor.rlp
```

NOTE：
由于区块链的区块数据和状态分别保存在Chain和Executor微服务中，所以需要两个模块都进行快照，生成快照文件，恢复的时候根据各自的快照文件进行恢复。

### 快照恢复

以将快照恢复到节点1为例：

1. 将上述生成的快照文件拷贝到需恢复链数据的节点目录下

    ```bash
    $ cd test-chain/1
    $ cp ../0/snapshot_chain.rlp ../0/snapshot_executor.rlp ./
    ```

2. 依快照文件快照恢复

    快照命令行工具接受恢复参数后，依据快照文件恢复数据。

    ```bash
    $ ../../bin/snapshot_tool -m restore
    ```
