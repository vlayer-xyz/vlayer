# JSON Rpc Api

# v_call

vlayer exposes one RPC endpoint under `/` with the following structure of the call:
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
