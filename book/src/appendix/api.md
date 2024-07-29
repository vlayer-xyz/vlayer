# vlayer JSON-RPC API

vlayer exposes one RPC endpoint under `/` with the following methods:
- `v_prove`
- `v_getProofRequest`

With general format of request looking a follows.

```json
 {
    "method": "<method name>",
    "params": {
        "<params object>"
    }
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


## v_prove
`v_prove` is the core endpoint that vlayer provides, with the following format request:

```json
{
    "method": "v_prove",
    "params": {   
        "to": "<contract address>",
        "data": "0x<abi encoded calldata>",
        "chain_id": 1,
        "email": "<optional email proof structure>",
        "web": "<optional web proof structure>",
    }
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

## v_getProofRequest
To get result of `v_prove` query `v_getProofRequest`. 

```json
{
    "method": "v_getProofRequest",
    "params": {   
        "id": "<proof request hash>",
    }    
}
```

```json
{
    "jsonrpc": "0.2",
    "result": {
        "proof": "<calldata encoded Proof structure>",
        "result": "<calldata encoded result of execution>"
    }
}
```

Where `result` is an ABI encoded result of the function execution and `proof` is a Solidity `Proof` structure to prepend in verifier function.
