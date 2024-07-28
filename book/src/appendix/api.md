# JSON Rpc Api

vlayer exposes one RPC endpoint under `/` with the following structure of the request:
```json
 {
    "method": "v_prove",
    "params": {
        "<proveRequest object>"
    }
 }
```

and the response:
```json
{
    "result": "..."
}
```


# v_prove
`v_prove` is the core endpoint that vlayer provides, with the following format request:

```json
 {
    "method": "v_call",
    "params": [
        {   
            "to": "<contract address>",
            "data": "0x<abi encoded calldata>",
            "chain_id": 1,
            "email": "<base64? encoded raw email>",
            "web": "<encoded web artifacts>",
        }
    ]
 }
```

and the response:

```json
{
    "result": "...",
    "proof": "..."
}
```

Where `result` is an ABI encoded result of the function execution and `proof` is a Solidity `Proof` structure to prepend in verifier function.
