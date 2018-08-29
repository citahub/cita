# 系统配置

使用 CITA 搭建一条链，用户可以根据需要在区块链生成创世块时进行配置，具体配置项如下（配置在 SysConfig 系统合约实现）：

## 配置项

```solidity
    uint delayBlockNumber;                 // 权限合约以及配额管理等系统合约是在多少个块之后生效
    bool checkPermission;                  // 是否需要检查权限
    bool checkQuota;                       // 是否需要检查配额
    string chainName;                      // 区块链的名称
    uint32 chainId;                        // ChainID
    string operator;                       // 运营方
    string website;                        // 运营方网站
    uint64 blockInterval;                  // 出块间隔
    EconomicalModel economicalModel;       // 经济模型
    TokenInfo tokenInfo;                   // 原生代币信息

    enum EconomicalModel { Quota, Charge }
    // Quota 模型为配额模型，交易不需要手续费，选择此配置，CITA 以联盟链方式运行
    // Charge 模型为收费模型，交易需要手续费，选择此配置， CITA 以公有联盟链的方式运行

    struct TokenInfo {
        string name;                       // 名称
        string symbol;                     // 符号
        string avatar;                     // 符号链接
    }
```

其中只有 chainName，operator，website 这三项可以在链运行之后再进行修改，其他项均不可再修改。

## 接口

调用接口，可以参考 [sys_config.sol](https://github.com/cryptape/cita/blob/develop/scripts/contracts/src/system/sys_config.sol#L6) 合约中的 SysConfigInterface，使用方法可以参考 Solidity 的文档，以及 CITA 交易发送的文档

## 初始化

sys_config.sol 合约通过写入创世块的方式来初始化，具体可以参考 [config_tool](./chain/config_tool)
