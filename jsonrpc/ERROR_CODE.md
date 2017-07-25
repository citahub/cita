# JSON RPC Error Codes

## JSON RPC Standard errors

| Code             | Possible Return message                  | Description                              |
| ---------------- | ---------------------------------------- | ---------------------------------------- |
| -32700           | Parse error                              | Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text. |
| -32600           | Invalid Request                          | The JSON sent is not a valid Request object. |
| -32601           | Method not found                         | The method does not exist / is not available. |
| -32602           | Invalid params                           | Invalid method parameter(s).             |
| -32603           | Internal error                           | Internal JSON-RPC error.                 |
| -32000 to -32099 | `Server error`. Reserved for implementation-defined server-errors. |                                          |

## Custom error 

| Code    | Possible Return message | Description |
| --------|-------------------------|-------------|
|100 | X doesn't exist    | Should be used when something which should be there is not found. (Doesn't apply to eth_getTransactionBy* and eth_getBlock*. They return a success with value `null`)
|101 | Rejected           | Should be used when an action was rejected, e.g. because of its content (too long contract code, containing wrong characters ?, should differ from `-32602` - Invalid params).
|326009 | Timeout            | Should be used when an action timedout.
|400 | 

参考文档:
1. [JSON RPC specification](http://www.jsonrpc.org/specification)

2. [Ethereum wiki/JSON RPC Error Codes Improvement Proposal](https://github.com/ethereum/wiki/wiki/JSON-RPC-Error-Codes-Improvement-Proposal)

