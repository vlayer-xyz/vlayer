# vlayer JSON-RPC API

vlayer exposes one RPC endpoint under `/` with the following methods:
- `v_prove`
- `v_getProofReceipt`
- `v_proveChain`

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


## v_prove
`v_prove` is the core endpoint that vlayer provides, with the following format request:

```json
{
    "method": "v_prove",
    "params": [{   
        "to": "<contract address>",
        "data": "0x<abi encoded calldata>",
        "chain_id": "<desired chain id>",
        "email": "<optional email proof structure>",
        "web": "<optional web proof structure>",
    }]
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
To get result of `v_prove` query `v_getProofReceipt`. 

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

## v_proveChain

### Query

This call takes chain ID and an array of block numbers as an argument.

An example call could look like this:

```json
{
  "method": "v_chain",
  "params": {
    "chain_id": 1,
    "block_numbers": [
      12_000_000,
      12_000_001,
      20_762_494, // This should be recent block that can be verified on-chain
    ]
  }
}
```

### Success

It returns two things:
* Sparse MPT that contains proofs for all block numbers passed as arguments.
* 𝜋 - the zk-proof that the trie was constructed correctly (invariant that all the blocks belong to the same chain is maintained).

```json
{
    "result": {
        "proof": "0x...", // ZK Proof
        "nodes": [
          "0x..." // Root node. It's hash is proven by ZK Proof
          "0x..." // Other nodes in arbitrary order
          ...
        ]
    }
}
```
