# 在华为鲲鹏上运行 CITA

鲲鹏是华为海思发布的基于 ARM 架构授权，由华为自主设计完成的鲲鹏芯片。

### 交叉编译环境

1. 安装 aarch64 交叉编译器

   参照华为云论坛 [如何搭建鲲鹏开发环境](https://bbs.huaweicloud.com/forum/thread-21263-1-1.html)。

    ```shell
    wget https://releases.linaro.org/components/toolchain/binaries/latest-5/aarch64-linux-gnu/gcc-linaro-5.5.0-2017.10-x86_64_aarch64-linux-gnu.tar.xz
    tar -xvf gcc-linaro-5.5.0-2017.10-x86_64_aarch64-linux-gnu.tar.xz
    mv gcc-linaro-5.5.0-2017.10-x86_64_aarch64-linux-gnu /opt/
    export PATH=/opt/gcc-linaro-5.5.0-2017.10-x86_64_aarch64-linux-gnu/bin:"${PATH}"
    ```

2. 配置 Rust 交叉编译环境

   rustup 安装 aarch64 的 target：

   ```
   rustup target add aarch64-unknown-linux-gnu
   ```

   修改 ~/.cargo/config （如果没有这个文件，则创建）写入：

   ```
   [target.aarch64-unknown-linux-gnu]
   linker = "aarch64-linux-gnu-gcc"
   ```

### 依赖库

1. openssl

   从 openssl 网站下载源码：

   ```shell
   wget https://www.openssl.org/source/openssl-1.1.1e.tar.gz
   tar zxvf openssl-1.1.1e.tar.gz
   mv openssl-1.1.1e /opt/
   ```

   交叉编译：

   ```shell
   cd /opt/openssl-1.1.1e
   CC=gcc CROSS_COMPILE=aarch64-linux-gnu- ./config no-asm shared
   ```

   注意：这里要手工修改一下生成的 Makefile，删除两处 `-m64` 的编译选项。

   编译：

   ```shell
   make
   ```

   设置两个环境变量：

   ```shell
   export OPENSSL_LIB_DIR=/opt/openssl-1.1.1e/
   export OPENSSL_INCLUDE_DIR=/opt/openssl-1.1.1e/include/
   ```

2. snappy

   snappy 可以直接下载编译好的二进制版本。

   ```shell
   wget http://mirror.archlinuxarm.org/aarch64/extra/snappy-1.1.8-1-aarch64.pkg.tar.xz
   tar Jxvf snappy-1.1.8-1-aarch64.pkg.tar.xz
   ```

   直接将对应的动态库放入交叉编译器的 lib 目录下

   ```shell
   cp usr/lib/libsnappy.so /opt/gcc-linaro-5.5.0-2017.10-x86_64_aarch64-linux-gnu/aarch64-linux-gnu/lib64/
   ```

### 编译

```shell
make aarch64_debug
```

生成的发布件在 `target/aarch64_install` 目录下。

### 运行

1. 服务器

   可以在 [华为云](https://www.huaweicloud.com/product/ecs.html) 上购买使用鲲鹏处理器的云主机，推荐型号为 kc1.xlarge.2，配置为 4c8g。

2. 操作系统

   CITA 当前支持 Ubuntu 18.04，请在创建云主机时选择该操作系统。

3. 安装依赖

   替换软件源为中科大镜像。替换 /etc/apt/sources.list 中原有的 url 为  `http://mirrors.ustc.edu.cn/ubuntu-ports`。

   注意：修改之前，最好先备份一下原有文件。

   ```shell
   sudo apt update
   sudo apt install rabbitmq-server libsnappy-dev
   ```

4. 运行 CITA

   上传前面编译好的 `aarch64_install` 目录到鲲鹏服务器。

   生成链的配置： solc 等配置工具暂时还不支持 arm，因此生成链的配置还需要在 x86 版本上进行，然后上传到 `aarch64_install` 目录里面。

   运行：

   ```shell
   ./bin/cita bebop setup test-chain/0
   ./bin/cita bebop start test-chain/0
   ```

   查看日志，确认可以正常出块：

   ```shell
   $ tail -100f test-chain/0/logs/cita-chain.log 
   2020-03-23 - 17:56:53 | cita_chain           - 107   | INFO  - CITA:chain
   2020-03-23 - 17:56:53 | cita_chain           - 108   | INFO  - Version: 20.2.0-7d346ea4
   2020-03-23 - 17:56:53 | amqp::session        - 196   | INFO  - Session initialized
   2020-03-23 - 17:56:53 | amqp::session        - 196   | INFO  - Session initialized
   2020-03-23 - 17:56:53 | core::libchain::chai - 371   | INFO  - chain config: Config { prooftype: 2 }
   2020-03-23 - 17:56:53 | core::libchain::chai - 386   | INFO  - get chain max_store_height : 0  current_height: 0
   2020-03-23 - 17:56:57 | core::libchain::chai - 1358  | INFO  - new chain status height 1, hash ad4a19a94253cdcca6da112ac84aa4aa75d59ef1436de2336106e1b7a78a409b
   2020-03-23 - 17:57:00 | core::libchain::chai - 1358  | INFO  - new chain status height 2, hash 5265652fdc861b7b505d63c8b374e43d0911f9e01f7ec464b55a054db0b808af
   ```

