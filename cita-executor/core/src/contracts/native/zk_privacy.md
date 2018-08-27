使用零知识证明技术实现隐私交易验证系统合约。

Warning：当前代码仅为原型验证系统，请勿用于生产环境。

此功能通过feature控制，默认关闭。

打开此功能有两种方法：
1. 修改 cita-executor/Cargo.toml。
在 \[features\] 下面 default 列表中增加 privatetx 。
2. 使用如下命令单独编译cita-executor，并替换原有的可执行文件。
```
cd cita-executor
cargo build --release --features "privatetx"
```
***
### 使用说明
1. 生成零知识证明所需的参数。
```
参见 https://github.com/cryptape/zktx
```
2. 将生成的 PARAMS 目录拷贝到 target/resource/
3. 使用 admintool 生成节点配置文件，并运行节点。
4. 运行客户端测试隐私交易。
```
参见 https://github.com/cryptape/zktx
```