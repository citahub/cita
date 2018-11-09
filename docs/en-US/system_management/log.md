# Log Management

Log plays an important role in system debugging, troubleshooting, and even business operation and maintenance.

Each microservice's log information is logged to an independent log file.

## Log Location

The log files are located in the logs directory under the node folder, and each microservice has a separate log file.

```
~/cita/test-chain/0$ ls logs/
cita-auth.log  cita-bft.log  cita-chain.log  cita-executor.log  cita-forever.log  cita-jsonrpc.log  cita-network.log
```

## Log Priority

The priority of the log is defined as follows:

```
Error,         //error是日志分级的最高等级
Warn,
Info,
Debug,
Trace,         //trace是最低等级
```

The default log level is `Info`. Here is an example:

```
2018-04-02T14:38:12.463454121+08:00 - INFO - pre_proc_prevote not have any thing in 9449 35
2018-04-02T14:38:13.964462688+08:00 - INFO - pre_proc_prevote height 9449,round 35 hash None locked_round None
```

Of course, logs above the `Info` level will also be printed:

```
2018-04-02T13:18:18.629297896+08:00 - WARN - Buffer is not enough for payload 257 > 167.
2018-04-02T13:18:20.101335388+08:00 - ERROR - Buffer is malformed 5135603446501605376 != 16045690981097406464.
```

The log priority can be modified when starting CITA:

```
./env.sh ./bin/cita start test-chain/0 trace
```

At this time, the log of `Trace` level can also be printed:

```
2018-04-02T14:38:09.842824387+08:00 - TRACE - response block's tx hashes for height:9350
2018-04-02T14:38:09.843117154+08:00 - TRACE - response block's tx hashes for height:9351
```

CITA supports setting different priorities for different modules, which is very helpful when debugging the system.

However, in order to simplify the use, it is a unified log level for all modules when set by `./bin/cita`.

If there is a need for system debugging, the user can temporarily modify the following content in the `start` function of `./bin/cita`:

```
50          RUST_LOG=cita_auth=${debug},cita_chain=${debug},cita_executor=${debug},cita_jsonrpc=${debug},cita_network=${debug},cita_bft=${debug},\
51  core=${debug},engine=${debug},jsonrpc_types=${debug},libproto=${debug},proof=${debug},txpool=${debug} \
52          cita-forever start > /dev/null 2>&1
```

## Log Splitting

CITA nodes need to run continuously for a long time, so the size of log file will get bigger and need to be cleaned regularly. Sometimes, you may also need to back up a certain important log separately. All these scenarios will need the log splitting operation.

To meet the needs of different scenarios, we make CITA's log splitting function more flexible. By signaling the process, trigger the log splitting and dumping of log files which ensures that no logs are lost during the switch. For multiple microservices within a node, we provide the following command:

```
./env.sh ./bin/cita logrotate test-chain/0
```

The effect is as follows:

```
./test-chain/0/logs/cita-auth.log
./test-chain/0/logs/cita-auth_2018-04-02_11-34-51.log
./test-chain/0/logs/cita-chain.log
./test-chain/0/logs/cita-chain_2018-04-02_11-34-51.log
./test-chain/0/logs/cita-jsonrpc.log
./test-chain/0/logs/cita-jsonrpc_2018-04-02_11-34-51.log
./test-chain/0/logs/cita-executor.log
./test-chain/0/logs/cita-executor_2018-04-02_11-34-51.log
./test-chain/0/logs/cita-bft.log
./test-chain/0/logs/cita-bft_2018-04-02_11-34-51.log
./test-chain/0/logs/cita-network.log
./test-chain/0/logs/cita-network_2018-04-02_11-34-51.log
```

The original log content is transferred to the backup log file with the current date. The original log file is cleared, and the process continues to be written into the original log file.

The backup log files can be filtered out by the following command:

```
find ./test-chain/*/logs | grep `date "+%Y-%m-%d"`
```

Then you can move these files to a certain location according to the user's needs, compress and save, or even delete directly.

If the user wants to periodically back up/clean the log, the above command can be set to the system's periodic task.

See the documentation of cron or logrotate for more details.