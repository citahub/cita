#getBlockNum.sh 统计两个块高度的之间(包含开始和结束)交易总数 第一个参数：开始的块高度 ，第二个参数：结束的快高度， 第三个参数：jsonrpc的ip地址

#例如：
    ./getBlockNum.sh 10 20 127.0.0.1


#benchmark.sh 创建账户、发送交易，输出交易量、入链的起始高度、时间

#例如：
#1、创建合约
    ./benchmark.sh

#2、发送合约交易，输出交易量、入链的起始高度、时间
    ./benchmark.sh config_call.json
#3、发送存证交易，输出交易量、入链的起始高度、时间
    ./benchmark.sh config_store.json

#增加随机端口设置延迟 参数1:端口  参数2:延迟多少毫秒 参数3:多少秒随机tc(延迟、丢失、重复、损坏)一次 参数4: tc设置类型(delay:延迟, loss:丢包, dup:重复, corrupt:损坏)

#例如
	./setPortDelay.sh 4000 1000 5 delay
#注意：参数4:delay时，参数2表示延迟多少毫秒; loss时,参数2表示多少几率丢失;dup时,参数2表示多少几率重复;corrupt时,参数2表示多少几率损失;
#如下表示20%的几率丢失
	./setPortDelay.sh 4000 20 5 loss




#chain性能测试
#第一个参数:1 | 2, 第二个参数是一个block中多少交易, 第三个参数: profile开始时间   第三个参数: profile运行时间
#例如
##创建合约
./chain_performance.sh 1 10000 0 10

##合约交易
./chain_performance.sh 2 10000 0 10
