# Types of JSON-RPC Parameters and Returns

## 基本类型

### `Quantity`

大整数类型。

* As Parameters

    * `0x` 开头的十六进制的字符串（仅包含 `0-9` 和 `a-f` 字符）。
    * 必须为字符串格式，即左右有双引号。
    * 不可以为空字符串。
    * 不可以 `0x` ，`0` 必须写做 `0x0` 。
    * （不建议） 目前兼容使用大写 `0X` 开头。
    * （不建议） 目前兼容使用大写 `A-F` 字符。
    * （不建议） 目前兼容十进制字符串（不用 `0x` 开头且仅含有字符 `0-9` ）。
    * （不建议） 目前兼容数据高位填充 `0` 。

    * Examples:

        * (Good) `"0xab5801a7"`
        * (Bad) `"0Xab5801a7"`
        * (Bad) `"0xAB5801A7"`
        * (Bad) `"2874671527"`
        * (Bad) `"0x0000ab5801a7"`
        * (Wrong) `0xab5801a7`
        * (Wrong) `"0x"`
        * (Wrong) `"ab5801a7"`

* As Returns

    `0x` 前缀的、紧凑型的、十六进制小写字符串。

### `Integer`

一般整数类型。

* As Parameters

    * 十进制数值。

    * Examples:

        * (Good) `2874671527`
        * (Wrong) `"2874671527"`
        * (Wrong) `"0xab5801a7"`
        * (Wrong) `0xab5801a7`

* As Returns

    * 十进制数值。

### `Data`

不定长二进制数据类型。

* As Parameters

    * `0x` 开头的十六进制的字符串（仅包含 `0-9` 和 `a-f` 字符）。
    * 必须为偶数个字符。
    * 使用 `0x` 表示空数据。
    * （不建议） 目前兼容使用大写 `0X` 开头。
    * （不建议） 目前兼容使用大写 `A-F` 字符。

    * Examples:

        * (Good) `"0x"`
        * (Bad) `"0Xab5801a7"`
        * (Bad) `"0xAB5801A7"`
        * (Wrong) `0xab5801a7`
        * (Wrong) `"0xab5801a"`
        * (Wrong) `""`
        * (Wrong) `"ab5801a7"`

* As Returns

    * `0x` 开头的十六进制的字符串（仅包含 `0-9` 和 `a-f` 字符）。

### `Data20` / `Data32`

定长二进制数据。 `Data20` 为 20 字节， `Data32` 为 32 字节。

* As Parameters

    * `0x` 开头的十六进制的定长字符串（仅包含 `0-9` 和 `a-f` 字符）。
    * 需要补 `0` 填充完整。
        * `Data20` 有 40 个字符（不包括前缀）。
        * `Data32` 有 64 个字符（不包括前缀）。
    * （不建议） 目前兼容使用大写 `0X` 开头。
    * （不建议） 目前兼容使用大写 `A-F` 字符。

    * Examples:

        * (Good) `"0x00000000000000000000000000000000ab5801a7"`
        * (Bad) `"0X00000000000000000000000000000000ab5801a7"`
        * (Bad) `"0x00000000000000000000000000000000AB5801A7"`
        * (Wrong) `"0xab5801a7"`
        * (Wrong) `0x00000000000000000000000000000000ab5801a7`

* As Returns

    * `0x` 开头的十六进制的定长字符串（仅包含 `0-9` 和 `a-f` 字符）。

### `Boolean`

布尔类型， `true` 或者 `false` 。

### `String`

字符串类型。

### `Tag`

标签类型，由特定字符串组成的枚举类型。

目前只有一个 `BlockTag` 类型。

* `BlockTag`

    * `String` `"earliest"` - for the earliest/genesis block.
    * `String` `"latest"` - for the latest mined block.

## 复合类型

### `BlockNumber`

* `Quantity | BlockTag` ： 通常可以为空，为空时默认值为 `"latest"` 。

### `CallRequest`

* `from`: `Data20` - **Optional** The address the transaction is sent from.
* `to`: `Data20` The address the transaction is directed to.
* `data`: `Data` - **Optional** Hash of the method signature and encoded parameters. For details see [Ethereum Contract ABI](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI).

### `Filter`

* `fromBlock`: `BlockNumber` - **Optional** 起始块高度。
* `toBlock`: `BlockNumber` - **Optional** 中止块高度。
* `address`: `Data20 | [Data20]` - **Optional** Contract address or a list of addresses from which logs should originate.
* `topics`: `[Data32 | [Data32]]` - **Optional** Array of `Data32` topics. Topics are order-dependent. Each topic can also be an array of DATA with "or" options.

Topics are order-dependent. A transaction with a log with topics [A, B] will be matched by the following topic filters:

* `[]` "anything"
* `[A]` "A in first position (and anything after)"
* `[null, B]` "anything in first position AND B in second position (and anything after)"
* `[A, B]` "A in first position AND B in second position (and anything after)"
* `[[A, B], [A, B]]` "(A OR B) in first position AND (A OR B) in second position (and anything after)"

### `Block`

* `version` - Version of Block(0 default)
* `hash` - The hash value of block
* `header`- Block header
    * `timestamp` - The timestamp of block
    * `prevHash`- hash value of previous block
    * `number` - 块高(十六进制)
    * `stateRoot`- 状态树根
    * `transactionsRoot` - 交易树根
    * `receiptsRoot`- 回执树根
    * `gasUsed`-  The amount of gas used in the block
    * `proof`-
        * `Tendermint`
            * `proposal`- 提议内容的hash
            * `height` - 块高(十进制)
            * `round` - 投票轮数
            * `commits`- 投票人地址和投票内容签名
    * `proposer`- 提议者
* `body`- block body
    * `transactions`- 交易列表