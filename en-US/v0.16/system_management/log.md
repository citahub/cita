# 日志管理
日志在系统调试，问题定位，甚至业务运维方面有着重要的作用。

CITA 每个微服务的日志信息都会被记录到一个单独的日志文件。

## 日志位置
CITA 日志文件位于节点文件夹下的logs目录中，每个微服务单独一个日志文件。
```
~/cita/node0$ ls logs/
cita-auth.log  cita-bft.log  cita-chain.log  cita-executor.log  cita-jsonrpc.log  cita-network.log 
```

## 日志优先级
日志优先级定义如下：
```
Error,         //error是日志分级的最高等级
Warn,
Info,
Debug,
Trace,         //trace是最低等级
```
CITA 默认日志等级为`Info`。示例如下：
```
2018-04-02T14:38:12.463454121+08:00 - INFO - pre_proc_prevote not have any thing in 9449 35
2018-04-02T14:38:13.964462688+08:00 - INFO - pre_proc_prevote height 9449,round 35 hash None locked_round None
```
当然`Info`级别以上的日志也会输出：
```
2018-04-02T13:18:18.629297896+08:00 - WARN - Buffer is not enough for payload 257 > 167.
2018-04-02T13:18:20.101335388+08:00 - ERROR - Buffer is malformed 5135603446501605376 != 16045690981097406464.
```
日志优先级可以在启动 CITA 的时候通过参数修改：
```
./env.sh ./bin/cita start node0 trace
```
这时`Trace`级别的日志也可以打印出来了：
```
2018-04-02T14:38:09.842824387+08:00 - TRACE - response block's tx hashes for height:9350
2018-04-02T14:38:09.843117154+08:00 - TRACE - response block's tx hashes for height:9351
```
CITA 支持为不同模块设置不同的优先级，这个在系统调试的时候非常有帮助。

但是为了简化使用，通过`./bin/cita`设置的时候是所有模块用统一的日志等级。

如果有系统调试的需要，用户可以临时修改`./bin/cita`中`start`函数中的如下内容：
```
50          RUST_LOG=cita_auth=${debug},cita_chain=${debug},cita_executor=${debug},cita_jsonrpc=${debug},cita_network=${debug},cita_bft=${debug},\
51  core=${debug},engine=${debug},jsonrpc_types=${debug},libproto=${debug},proof=${debug},txpool=${debug} \
52          cita-forever start > /dev/null 2>&1
```

## 日志分割
CITA 节点需要长时间持续运行，因此日志文件会越来越大，需要定期清理。

或者需要将某一段比较重要的日志单独备份。

这都会涉及到日志分割的功能。

为了适应不同场景的需要，CITA 的日志分割功能采用比较灵活的方式。

通过向进程发信号，触发日志分割和日志文件的转储，保证切换期间没有日志丢失。

对于一个节点内的多个微服务，有如下的命令封装：
```
./env.sh ./bin/cita logrotate node0
```
效果如下：
```
./node0/logs/cita-auth.log
./node0/logs/cita-auth_2018-04-02_11-34-51.log
./node0/logs/cita-chain.log
./node0/logs/cita-chain_2018-04-02_11-34-51.log
./node0/logs/cita-jsonrpc.log
./node0/logs/cita-jsonrpc_2018-04-02_11-34-51.log
./node0/logs/cita-executor.log
./node0/logs/cita-executor_2018-04-02_11-34-51。log
./node0/logs/cita-bft.log
./node0/logs/cita-bft_2018-04-02_11-34-51.log
./node0/logs/cita-network.log
./node0/logs/cita-network_2018-04-02_11-34-51.log
```
原有日志内容转移到带有当前日期的备份日志文件中，原有日志文件清空，进程继续往原有的日志文件里面写入。

可通过如下命令将备份的日志文件筛选出来：
```
find ./node*/logs | grep `date "+%Y-%m-%d"`
```
然后可以根据用户的需要，移动到专门的备份的地方，压缩保存，甚至是直接删除。

如果用户想要定时备份/清理日志，可以将上述命令设置为系统的周期任务。

更多详细的用法请参见 cron 或者 logrotate 工具的文档。
