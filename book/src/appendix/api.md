# vlayer JSON-RPC API

vlayer exposes one RPC endpoint under `/` with the following methods:
- `v_call`
- `v_getProofReceipt`

With general format of request looking a follows.

```json
 {
    "method": "<method name>",
    "params": [{
        "<params object>"
    }]
 }
```

And the response format below.

```json
{
    "jsonrpc": "<version>",
    "result": {
        "<result object>"
    }
}
```


## v_call
`v_call` is the core endpoint that vlayer provides for requesting proof of computation generation, with the following format request:

```json
{
    "method": "v_call",
    "params": [
        {   
            "to": "<contract address>",
            "data": "0x<abi encoded calldata>",
        },
        {
            "block_no": "<desired block number>",
            "chain_id": "<desired chain id>",
        },
        {
             "web_proof": {
                "notary_pub_key": "<notary public key>",
                "tls_proof": "<tls proof value>",
            }
        }
    ]
}
```

and the response:

```json
{
    "jsonrpc": "0.2",
    "result": {
        "id": "<proving_hash>",
        "result": "<abi encoded result of preflight execution>"
    }
}
```

## v_getProofReceipt

### Query
To get result of `v_call` query `v_getProofReceipt`. 

```json
{
    "method": "v_getProofReceipt",
    "params": [{   
        "id": "<proof request hash>",
    }]
}
```

There are three possible results: `pending`, `success` and `error`.

### Pending

```json
{
    "jsonrpc": "0.2",
    "status": "pending",
}
```

### Success

```json
{
    "jsonrpc": "0.2",
    "status": "success",
    "result": {        
        "proof": "<calldata encoded Proof structure>",
        "data": "<calldata encoded result of execution>",
        "block_no": "<hex encoded settlement block>"
    }
}
```

`data` is an ABI encoded result of the function execution and `proof` is a Solidity `Proof` structure to prepend in verifier function. Note that settlement block is only available in receipt, as we don't want to make assumption on when the the settlement block is assigned.

### Error

```json
{
  "jsonrpc": "0.2",
  "status": "error",
  "error": {
    "message": "<error message>",
  }
}
```
