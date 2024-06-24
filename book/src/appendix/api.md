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

and response:
```json
{
    "result": "..."
}
```


# v_call
`v_call` is the core endpoint that vlayer provides, with following format request:

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
            "mail": "<base64? encoded raw mail>",
            "web": "<encoded web artifacts>",
        }
    ]
 }
```

and response:

```json
{
    "result": "...",
    "proof": "..."
}
```

Where `result` is abi encoded result of function execution and `proof` is solidity `Proof` structure prepended.
