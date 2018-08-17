# 快照功能介绍

从字面上解释，“快照”这个词来源于照相、摄影等领域，后来延展到范围很广，当需要保留某件事物某个时间的状态时，就可以说对其对一次“快照”。  
快照属于工具类的功能，不属于区块链本身的功能，我们对区块链某个高度的状态做快照，保存状态、区块等数据，可以在较短时间内同步恢复链数据。

snapshot_tool代码位于tools/snapshot_tool目录下。

## 使用说明
snapshot_tool有2个功能：做快照和恢复快照
* 做快照是将当期区块链的状态保存下来，放在文件中
* 恢复快照是根据保存下来的文件将区块链恢复到保存时的状态。  

命令格式如下：  
```
snapshot_tool -m snapshot [-e HEIGHT]  [-f FILE]
snapshot_tool -m restore [-f FILE]
```

括号中的选项是可选的。  
-m是指定功能，是做快照，还是恢复快照。  
-e是指定区块链高度，缺省值是区块链当期高度。  
-f 是指定快照文件的名字，如果指定为FILE，chain模块会生成FILE_chain.rlp，executor模块会生成FILE_executor.rlp，缺省时是snapshot_chain.rlp和snapshot_executor.rlp。  

下面演示一下用法，对100高度做快照：
```
$ cd test-chain/0
$ ../../bin/snapshot_tool -m snapshot -e 100
$ ls snap*
snapshot_chain.rlp  snapshot_executor.rlp
$ ../../bin/snapshot_tool -m restore
```

NOTE：  
	由于区块链的数据和状态分别保存在Chain和Executor模块中，所以需要2个模块都进行快照，保存下来快照文件，恢复的时候根据各自保存的文件进行恢复。  
	Chain中主要保存Block数据，Executor主要保存State状态。


## 实现方法
snapshot通过MQ订阅和发送消息，与auth、consensus、network、chain、executor等微服务通信。   
发送消息的类型是SnapshotReq，订阅消息的类型是SnapshotResq，消息的类别根据Cmd进行区分。如下所示：
```
message SnapshotReq {
    Cmd cmd = 1;
    uint64 start_height = 2;
    uint64 end_height = 3;
    string file = 4;
    Proof proof = 5;
}

message SnapshotResp {
    Resp resp = 1;
    Proof proof = 2;
    uint64 height = 3;
    bool flag = 4;
}
enum Cmd {
    Begin = 0;
    Clear = 1;
    Snapshot = 2;
    Restore = 3;
    End = 4;
}
```

### 做快照
snapshot_tool向MQ发送Cmd = Snapshot的消息，并带有高度end_eight和文件名等消息。  
虽然auth、consensus、network都订阅了这个消息，但是对于Cmd=Snapshot的消息并不处理，只有Chain和Executor会处理。  
##### Chain
###### 整体流程  
为了防止阻塞正常的Chain处理流程，会单独起一个线程做快照，即take_snapshot。在快照结束时发送SnapshotResp信息给snapshot_tool。

###### 保存内容
快照会保存从高度1到指定高度的所有block的部分数据。不保存创世快数据的主要原因是不必要，因为恢复的时候需要区块链运作起来，所以必然会有创世快的数据。

每个块保存的内容有：header、receipts，最后的100个块还会保存block body，这是为了恢复快照之后需要给auth发送最新100个块的tx hashes。

###### 保存格式
每个块的数据会以rlp编码的形式存储，然后多个rlp存入1个block中，block的大小限制为4M，超过4M就新起一个block。这里的block是存储单位，不是区块链的block。block内有一定的形式：`parent_number + parent_hash + block rlps`。

block在存入文件时，会进行压缩，以更大程度的节省空间和降低时间。然后block的长度和hash会保存在Menifest中。

Manifest保存着很多元信息
* block_number：最后一个块的高度
* block_hash：最后一个块的hash
* state_root：最新的状态根
* block_hashes：上面那些block的hash和在文件中的偏移，hash是校验用的，当恢复快照时查看数据是否正确
* last_proof：这是为了恢复之后能够继续共识需要的

最后保存的文件内数据的格式是：`block: block: block....Menifest':Offset`。  
Manifest'比Menifest多了一点信息：每个block在文件中的长度和偏移。  
offset是Manifest’在文件中的偏移位置，这样在恢复的时候能够先拿到Manifest'，然后依次处理。

##### Executor
###### 整体流程 
跟Chain一样，也会新起一个线程来做快照，避免阻塞。在快照结束时发送SnapshotResp信息给snapshot_tool。

###### 保存内容
Executor保存的内容比Chain多一些，保存了block header和最新的一个Trie。block header的保存方法跟Chain一样，区别就是保存的数据不同，Executor只保存block header。State Trie的保存，是将最新的状态保存下来。  

###### 保存格式

State Trie中的Account取出来之后，构造成fat_rlp的形式。fat_rlp有一定的格式：`address + nonce + balance + code_hash + code + abi_hash + abi + storage kv`。  
> 一个Account如果很大的话可以形成多个fat_rlp。 

这里也有一个存储框的概念：chunk，多个fat_rlp放入一个chunk中。如果积攒起来的rlp的大小超过chunk的大小，这里是4M，可以分为多个chunk。  
> Executor的chunk和Chain的chunk不是同一个概念，更像是Chain中的Block的概念。  

chunk的数据也会经过压缩，再保存进文件中。  
文件的格式为：`chunk: chunk: chunk...block:block:block...Menifest':Offset`。  
offset为Manifest'在文件中的偏移位置。  


### 做恢复
恢复快照时需要将当前的微服务都暂停掉，处于飞行模式。  
流程是：**`begin-> restore -> clear -> end`**。
* begin是开始进入snapshot状态；
* restore是chain和executor进行恢复；
* clear是清除微服务内部缓存的数据；
* end是结束snapshot，清除snapshot标记。

begin主要是auth、consensus、network处理，开始进入特定的状态。  
restore主要是chain和executor处理，进行恢复工作。  
clear主要是auth、consensus、network处理，清理掉自己之前缓存的数据、状态等，状态从头开始。  
end是最后的结尾处理，如发送消息、清理标记等。  


恢复操作相当于将保存操作反过来。下面主要介绍restore的处理。  
##### Chain
根据文件结尾的offset取出Manifest，根据保存的格式进行恢复。
```
File =>   block: block: block....Menifest’:Offset
block =>  parent_number + parent_hash + block rlps
```
就是逐个block的恢复，feed_block_chunk -> feed_chunk -> feed_blocks -> feed 
最后替换数据库。

##### Executor
依次恢复state、block header。
也是依据文件格式，依次取出数据，放入数据库，最后替换数据。
```
State:
feed_state_chunk -> feed_chunk -> feed_state -> feed
Block: 
feed_block_chunk -> feed_chunk -> feed_blocks -> feed
```
最后替换掉数据库。

