# Common library of solidity

## AddressArray

Some operation about addressArray, include:

* remove: remove an address from array
* index: get the index of the address from array
* exist: check if the address in the array
* isNull: check the array of address is nul

## ContractCheck:

* isContract: check the address is an contract address

## notice

> The --allow-paths command line option for solc only works with absolute paths. It would be useful if it could be used with relative paths such as ../ and the current working directory(.).

Mode details at [issue](https://github.com/ethereum/solidity/issues/2928)

So using hard link for now. e.g.

```
$ pwd

.../cita/scripts/contracts/permission_management

```

Use `ln` command:

```
ln ../common/address_array.sol ./
```

