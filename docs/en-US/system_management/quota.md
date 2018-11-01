# Quota Management

## Overview

There are two limitations to the quota value in CITA:
* `BQL(BlockQuotaLimit)` : Indicates the maximum value of the block quota. The default value is 1073741824
* `AQL(AccountQuotaLimit)` : Indicates the maximum value of the account quota. The default value is 268435456

We can manage the block and account quota consumption caps through quota management contracts:

* Set block quota limit (BQL)
* Set the account quota limit (AQL):

    - Default account quota limit
    - Set the specified account quota limit

## Operation example

> we use [cita-cli](https://github.com/cryptape/cita-cli) in the following operations.

### Block quota management operation

Make sure that your chain is running normally, and then query the default block quota:

```shell
$ scm QuotaManager getBQL
```

Output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000040000000"
}
```

Admin can modify the block quota limitation value by the following command:

```shell
scm QuotaManager setBQL --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

Query the modified block quota limitation value:

```shell
$ scm QuotaManager getBQL
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000020000000"
  "
}
```
The default block quota limitation value has been updated.

### Account Quota Management Operation

Make sure that your chain is running normally, and then query the default account quota limitation:

```shell
$ scm QuotaManager getDefaultAQL
```

Output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000010000000"
}
```

Admin can modify the account quota limitation value by the following command:

```shell
$ scm QuotaManager setDefaultAQL --quota-limit 0x0000000000000000000000000000000000000000000000000000000020000000 --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

Query the account quota limitation:

```shell
$ scm QuotaManager getDefaultAQL
```

Output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000020000000"
}
```

The default account quota limitation value has been updated.
