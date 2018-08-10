# 系统保留地址列表

!> **系统保留地址更改！**
老版本用户请注意，为了让地址更加规范，最新版本的 CITA 将 0.16 版本及之前版本中无规律的系统保留地址规范到了一个地址段内。

## Ethereum Builtin

| 功能        | 起始地址（包含）                           |
|------------|--------------------------------------------|
| ecrecover | 0x0000000000000000000000000000000000000001 |
| sha256 | 0x0000000000000000000000000000000000000002 |
| ripemd160 | 0x0000000000000000000000000000000000000003 |
| identity  | 0x0000000000000000000000000000000000000004 |

## CITA Builtin

| 功能        | 起始地址（包含）                           |
|------------|--------------------------------------------|
| edrecover | 0x0000000000000000000000000000000000ff0001 |

## 保留地址段汇总

| 功能         | 起始地址（包含）                           | 终止地址（包含）                           |
|--------------|--------------------------------------------|--------------------------------------------|
| 所有保留地址 | 0xffffffffffffffffffffffffffffffffff000000 | 0xffffffffffffffffffffffffffffffffffffffff |
| 所有指令地址 | 0xffffffffffffffffffffffffffffffffff010000 | 0xffffffffffffffffffffffffffffffffff01ffff |
| 一般指令地址 | 0xffffffffffffffffffffffffffffffffff010000 | 0xffffffffffffffffffffffffffffffffff0100ff |
| Go 指令地址  | 0xffffffffffffffffffffffffffffffffff018000 | 0xffffffffffffffffffffffffffffffffff018fff |
| 所有系统合约 | 0xffffffffffffffffffffffffffffffffff020000 | 0xffffffffffffffffffffffffffffffffff02ffff |
| 一般系统合约 | 0xffffffffffffffffffffffffffffffffff020000 | 0xffffffffffffffffffffffffffffffffff0200ff |
| 权限系统合约 | 0xffffffffffffffffffffffffffffffffff021000 | 0xffffffffffffffffffffffffffffffffff0210ff |
| 原生系统合约 | 0xffffffffffffffffffffffffffffffffff030000 | 0xffffffffffffffffffffffffffffffffff03ffff |

## 已使用地址列表

| 地址                                        | 说明                            |
|--------------------------------------------|-------------------------------- |
| 0xffffffffffffffffffffffffffffffffff010000 | 存证指令                         |
| 0xffffffffffffffffffffffffffffffffff010001 | 存 ABI 指令                      |
| 0xffffffffffffffffffffffffffffffffff010002 | 修改合约内容指令                  |
| 0xffffffffffffffffffffffffffffffffff018000 | Go 合约指令                      |
| 0xffffffffffffffffffffffffffffffffff020000 | 系统参数配置                      |
| 0xffffffffffffffffffffffffffffffffff020001 | 共识节点管理                      |
| 0xffffffffffffffffffffffffffffffffff020002 | 链管理                           |
| 0xffffffffffffffffffffffffffffffffff020003 | 配额管理                         |
| 0xffffffffffffffffffffffffffffffffff020004 | 权限管理                         |
| 0xffffffffffffffffffffffffffffffffff020005 | 创建权限                         |
| 0xffffffffffffffffffffffffffffffffff020006 | 权限管理授权                     |
| 0xffffffffffffffffffffffffffffffffff020007 | 角色管理                         |
| 0xffffffffffffffffffffffffffffffffff020008 | 创建角色                         |
| 0xffffffffffffffffffffffffffffffffff020009 |                                 |
| 0xffffffffffffffffffffffffffffffffff02000a | 用户组管理                       |
| 0xffffffffffffffffffffffffffffffffff02000b | 创建用户组                        |
| 0xffffffffffffffffffffffffffffffffff02000c | admin管理                        |
| 0xffffffffffffffffffffffffffffffffff02000d | 角色授权                         |
| 0xffffffffffffffffffffffffffffffffff02000e | 批量交易                         |
| 0xffffffffffffffffffffffffffffffffff021000 | 发送交易                         |
| 0xffffffffffffffffffffffffffffffffff021001 | 新建合约                         |
| 0xffffffffffffffffffffffffffffffffff021010 | 新增权限                         |
| 0xffffffffffffffffffffffffffffffffff021011 | 删除权限                         |
| 0xffffffffffffffffffffffffffffffffff021012 | 更新权限                         |
| 0xffffffffffffffffffffffffffffffffff021013 | 设置授权                         |
| 0xffffffffffffffffffffffffffffffffff021014 | 撤销授权                         |
| 0xffffffffffffffffffffffffffffffffff021015 | 新建角色                         |
| 0xffffffffffffffffffffffffffffffffff021016 | 删除角色                         |
| 0xffffffffffffffffffffffffffffffffff021017 | 更新角色                         |
| 0xffffffffffffffffffffffffffffffffff021018 | 设置角色                         |
| 0xffffffffffffffffffffffffffffffffff021019 | 撤销角色                         |
| 0xffffffffffffffffffffffffffffffffff02101a | 新增用户组                       |
| 0xffffffffffffffffffffffffffffffffff02101b | 删除用户组                       |
| 0xffffffffffffffffffffffffffffffffff02101c | 更新用户组                       |
| 0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020 | 新增节点                         |
| 0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021 | 删除节点                         |
| 0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022 | 更新节点                         |
| 0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023 | 账户配额                         |
| 0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024 | 块配额                           |
| 0xffffffffffffffffffffffffffffffffff030000 |                                 |
| 0xffffffffffffffffffffffffffffffffff030001 | 隐私功能                         |
| 0xffffffffffffffffffffffffffffffffff030002 | 跨链功能                         |
