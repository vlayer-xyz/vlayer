# JSON Rpc Api

vlayer exposes one RPC endpoint under `/` with the following structure of the request:
```json
 {
    "method": "v_call",
    "params": [
        "<arg_object>",
        "<context>",
        "<extras>"
    ]
 }
```

and the response:
```json
{
    "result": "..."
}
```


# v_call
`v_call` is the core endpoint that vlayer provides, with the following format request:

```json
 {
    "method": "v_call",
    "params": [
        {   "from": "<from address>", # optional field
            "to": "<contract address>",
            "data": "0x<abi encoded calldata>"
        },
        {"chainId": 1, "blockNo": "latest"},
        {
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

Where `result` is an ABI encoded result of the function execution and `proof` is a Solidity `Proof` structure prepended.
