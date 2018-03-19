# 运行脚本

1 切换到当前目录，运行如下命令:

```shell
cd cita/tests/jsonrpc_performance
```

2 运行如下命令:

(1) 发送交易

```shell
../../target/install/bin/jsonrpc_performance --config config_correct.json
```

其中`config_correct.json`是正确格式的请求，其他请求类似，有如下几个:

* config_err_format.json(错误格式的请求)
* config_get_height.json(获取高度请求)
* config_dup.json(重复交易)
* config_signerr.json(验签错误)

## 测试结果

生成文件jsonrpc_performance.txt，内容如下:
test type: jsonrpc + auth + consensus(corrent)
                            tx_num: 20000
                            sucess: 20000
                            fail: 0
                            start_h: 67
                            end_h: 73
                            jsonrpc use time: 28435 ms
                            tps: 703
                            single tx respone time: 1421793 ns

其中:

* tx_num: 发送交易数
* start_h: 开始高度
* end_h: 结束高度
* jsonrpc use time: 花费时间（ms）
* tps: jsonrpc的tps
* single tx respone time: jsonrpc的响应时间

(2) 交易上链后，tps分析

```shell
../../target/install/bin/jsonrpc_performance --config config_correct.json --analysis=true
```
##结果

命令行输入如下：
```shell
2018-03-19T10:41:22.949292336+08:00 INFO jsonrpc_performance::send_trans - tx_num: 20000, start_h: 69, end_h: 76, use time: 34820 ms, tps: 574
```

生成文件 cita_performance.txt，内容如下：
height:67, blocknum: 0, time stamp :1521426872202, use time: 27259 ms
height:68, blocknum: 0, time stamp :1521426899461, use time: 647 ms
height:69, blocknum: 1441, time stamp :1521426900108, use time: 3001 ms
height:70, blocknum: 601, time stamp :1521426903109, use time: 5529 ms
height:71, blocknum: 2268, time stamp :1521426908638, use time: 4878 ms
height:72, blocknum: 4906, time stamp :1521426913516, use time: 4979 ms
height:73, blocknum: 3976, time stamp :1521426918495, use time: 7167 ms
height:74, blocknum: 3626, time stamp :1521426925662, use time: 9266 ms
height:75, blocknum: 3182, time stamp :1521426934928, use time: 22806 ms
height:76, blocknum: 0, time stamp :1521426948468, use time: 0 ms
tx_num: 20000, start_h: 69, end_h: 76, use time: 34820 ms, tps: 574

其中：

* height: 高度
* blocknum: block中交易数
* time stamp: block中的时间戳
* use time: 执行对应的block使用的时间
* tps: 交易发送开始到交易上链的tps

## 脚本说明

目的：创建合约、调用合约、查询recipts、调用eth_getLogs、检查发送的交易是否与block中的交易数一样
1 创建合约，并调用一次合约

```shell
./create_contract.sh
```

2 调用合约，参数1：调用合约几次   参数2：发送几次调用合约的交易

```shell
./call_contract.sh
```
