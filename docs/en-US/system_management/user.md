# User Management

CITA support group-based user management, with a tree-like relationship between groups, which can correspond to the organizational structure of the enterprise.

The group can be authorized using the permission management contract. Users in the group have both the group permission and their own permission.

For the group management, the scope constraint of permissions is :

* Users in a group can perform group management on this group and all subgroups of this group

Add the Group authentication in authentication process, so the whole authentication process include:

* Authenticate user permissions
* Authenticate group permissions

## Operation

> We use [cita-cli](https://github.com/cryptape/cita-cli) for the following demonstration. The operation class interface calls need to have the related permissions.

Admin creates a new user group by the follwing command：

```shell
$ scm GroupManagement newGroup \
      --origin 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009 \
      --name 7770660000000000000000000000000000000000000000000000000000000000 \
      --accounts "[e1c4021742730ded647590a1686d5c4bfcbae0b0,45a50f45cb81c8aedeab917ea0cd3c9178ebdcae]" \
      --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6
```

 The default `origin` is `0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009`.
 The hexadecimal representation of the group name `7770660000000000000000000000000000000000000000000000000000000000`.
 We will add these two users to this group: `e1c4021742730ded647590a1686d5c4bfcbae0b0`， `45a50f45cb81c8aedeab917ea0cd3c9178ebdcae`

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
    "blockNumber": "0x6922",
    "contractAddress": null,
    "cumulativeGasUsed": "0x1b8fcf",
    "errorMessage": null,
    "gasUsed": "0x1b8fcf",
    "logs": [
      {
        "address": "0xce6cd8f8562e31d44b1101986204cec34b1df025",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae",
        "logIndex": "0x0",
        "topics": [
          "0x876145257ed9001029e48f639669c6a3d20c2256585b00a716e557653ccb4813",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009",
          "0x7770660000000000000000000000000000000000000000000000000000000000"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x0"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff02000b",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae",
        "logIndex": "0x1",
        "topics": [
          "0xe676706adf1adf2871518b989e3e4ae7c1cc5bf8bb6012ecc94652f84edf4adf",
          "0x000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025",
          "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009",
          "0x7770660000000000000000000000000000000000000000000000000000000000"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x1"
      },
      {
        "address": "0xffffffffffffffffffffffffffffffffff020009",
        "blockHash": "0x0771624fa18da8380cf87238bd0dbb1e4114f2d707bdf9be6265c4ed50016960",
        "blockNumber": "0x6922",
        "data": "0x",
        "logIndex": "0x2",
        "topics": [
          "0xa016866023d98d9af30c4dd99810d92915ae7897f25baa30c8c826bf077f486b",
          "0x000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
        ],
        "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
        "transactionIndex": "0x0",
        "transactionLogIndex": "0x2"
      }
    ],
    "logsBloom": "0x00000002000000100000000000800000000000000000400004020000100000000000000000000002000000000080000000000000000000000000000000000002000000000000000000000000000080000004000000001000000000000004000000000000000000000010000000000000100000000020000000000000000000000000000000000000000000000000000000000000001000000000000040000000000000000000000000000000000000000000000000000000000000000000000400400000800004000000000000000000000000000000020010000000000000000000000000000000000000000000008000000000000001000000000000000000",
    "root": null,
    "transactionHash": "0x948de6f242b4ed2638ff4874febfd824facec1e71907154f1532ea19f78f8b21",
    "transactionIndex": "0x0"
  }
}
```
By now, we already create a new group. As you can see from `log`, the address of the new group is: `0xce6cd8f8562e31d44b1101986204cec34b1df025`。

Let's query all the group information to check if it is added successfully:

```shell
$ scm GroupManagement queryGroups
```

Get the receipt：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000ffffffffffffffffffffffffffffffffff020009000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
}
```

You can see that `0xce6cd8f8562e31d44b1101986204cec34b1df025` has been added.

Then we query the group name by the group address:

```shell
$ scm Group queryName --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x7770660000000000000000000000000000000000000000000000000000000000"
}
```

As you can see, the results are totally same with our input information of the new group. Now, let's look at the users's information in the group.

Query group users:

```shell
$ scm Group queryAccounts --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000e1c4021742730ded647590a1686d5c4bfcbae0b000000000000000000000000045a50f45cb81c8aedeab917ea0cd3c9178ebdcae"
}
```

These two users have been added sucessfully!

Because the groups are tree-type, we can also query the sub-user group information according to the parent group address. 

Query the address of the sub-user group:

```shell
$ scm Group queryChild --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001000000000000000000000000ce6cd8f8562e31d44b1101986204cec34b1df025"
}
```

Query the number of sub-user groups:

```shell
$ scm Group queryChildLength --address 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009
```

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

We can also query the information of the parent user group according to sub-user group address. 

```shell
$ scm Group queryParent --address 0xce6cd8f8562e31d44b1101986204cec34b1df025
```

Get the receipt:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x000000000000000000000000ffffffffffffffffffffffffffffffffff020009"
}
```
