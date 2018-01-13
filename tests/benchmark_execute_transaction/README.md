# 创建共识完成的block, executer执行交易，计算时间；计算stat root，统计时间；写db，统计时间


## 主要功能
当前目录运行如下命令查看：
```
../../target/release/benchmark_execute_transaction -h 
```

结果如下：
```
2018-01-16T14:13:32.687594197+08:00 INFO benchmark_execute_transaction - CITA:benchmark_execute_transaction
generate_block 0.1
Cryptape
CITA Block Chain Node powered by Rust

USAGE:
    benchmark_execute_transaction [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>          Sets a check config file
    -g, --genesis <FILE>         Sets a custom config file
    -b, --is_change_pv <true>    pv is or is‘t change
    -n, --tx_num <4000>          transation num in block
```

