# Quota Price Management

## Overview

When you choose `Charge` economic model in CITA, similar to gas in Ethereum, it is required to spend a certain amount of quota when sending a transaction or deploying a contract, etc.
The specific calculation method is: `Transaction fee = quotaUsed * quotaPrice`.
In order to better meet the needs of the operators, we provide an interface to set `quotaPrice`, and only administrator can set `quotaPrice` by sending a transaction.

## Operation Example

The default `quotaPrice` is 1000000, and we will demonstrates how to modify the quotaPrice by administrator using [cita-cli] (https://github.com/cryptape/cita-cli).

> The default `quotaPrice` before version 0.20 is 1

First, query the current `quotaPrice`:

```bash
$ cita-cli scm PriceManager getQuotaPrice
```

Get the output：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000f4240"
}

```

In here, we get the `quotaPrice`, which is default in hexadecimal.

Next, we change `quotaPrice` from 1000000 to 2000000:

```bash
$ cita-cli scm PriceManager setQuotaPrice \
              --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
              --price 0x00000000000000000000000000000000000000000000000000000000001e8480
```

Query again:

```bash
$ cita-cli scm PriceManager getQuotaPrice
```

Get the Output：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000001e8480"
}
```

In here, we can see that quotaPrice changed sucessfully.