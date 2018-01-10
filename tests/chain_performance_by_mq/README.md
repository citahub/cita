## 功能

通过MQ向Chain模块来发送Block来对Chain持续测试

## 方法

1. 修改节点目录下的chain.json，将权限检查和配额检查改成false
2. 启动Chain进程，或者启动单独一个节点
3. 使用命令进行压力测试: ./target/release/chain_performance_by_mq --help
4. 在node目录下启动测试进程，这样可以正确的使用.env文件

## 注意

* 调整交易配额和交易数，防止block超出block gas limit
* 创建合约/更改State时，随着State状态树变大，tps会越来越越慢
* 通过查询block中最后一个交易的receipt来确认交易是否正常执行
