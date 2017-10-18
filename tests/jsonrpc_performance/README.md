## 运行脚本

1. 切换目录，运行如下命令:

```
cd cita/tests/jsonrpc_performance
```

2. 运行如下命令: 

```
../../target/install/bin/jsonrpc_performance --config config_err_format.json
```

其中`config_err_format.json`是错误格式的请求，其他请求类似，有如下几个:
* config_correct.json(正确格式的请求)
* config_get_height.json(获取高度请求)
* config_dup.json(重复交易)
* config_signerr.json(验签错误)

## 测试结果

输出如下:

```
20171011 09:13:55～09:28:27 - INFO - test type: jsonrpc + auth + consensus(corrent), tx_num:200000, start_h: 2719, end_h: 2952, jsonrpc use time:849452 ms, tps: 235
```

其中:

* tx_num: 发送交易数
* start_h: 开始高度
* end_h: 结束高度
* jsonrpc use time: 花费时间（ms）
