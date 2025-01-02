# vlayer JSON-RPC API

vlayer exposes one RPC endpoint under `/` with the following methods:
- `v_call`
- `v_versions`
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


## v_call
`v_call` is the core endpoint that vlayer provides, with the following format request:

```json
{
    "method": "v_call",
    "params": [{ // CallParams   
        "to": "<contract address>",
        "data": "0x<abi encoded calldata>",
    }, { // CallContext
        "chain_id": "<desired chain id>",
        "gas_limit" "<maximum gas limit (default in SDK: 1_000_000)>",
    }]
}
```

and the response:

```json
{
    "jsonrpc": "2.0",
    "result": {
        "hash": "<proving hash>",
        "evm_call_result": "...",
        "proof": "<abi encoded result of preflight execution>",
    }
}
```

## v_versions
`v_versions` is the health check/versions endpoint

```json
{
    "method": "v_versions",
    "params": []
}
```

and the response:

```json
{
    "jsonrpc": "2.0",
    "result": {
        "call_guest_id": "0x8400c1983ee247ec835e565f924e13103b7a6557efd25f6b899bf9ed0c7ca491",
        "chain_guest_id": "0x9b330c5fda07d640226342a91272a661b9e51ad6713427777720bc26489dbc75",
        "semver": "1.2.3-dev-20241231-ae03fe73"
    }
}
```

## v_getProofReceipt

### Query
To get result of `v_call` query `v_getProofReceipt`. 

```json
{
    "method": "v_getProofReceipt",
    "params": {   
        "hash": "<proof request hash>",
    }
}
```

There are 5 possible `status` values:
* `queued`
* `waiting_for_chain_proof`
* `preflight`
* `proving`
* `ready`

If `status` is `ready`, the server will respond with a proof receipt.

### Queued, WaitingForChainProof, Preflight, Proving

```json
{
    "jsonrpc": "2.0",
    "result": {
        "status": "queued" | "waiting_for_chain_proof" | "preflight" | "proving",
    }
}
```

### Ready

```json
{
    "jsonrpc": "2.0",
    "result": {        
        "status": "ready",
        "receipt": {
            "data": {
                "proof": "<calldata encoded Proof structure>",
                "evm_call_result": "<calldata encoded result of execution>",
            },
            "metrics": {
                "gas": 0,
                "cycles": 0,
                "times": {
                    "preflight": 0,
                    "proving": 0,
                },
            },
        }
    }
}
```

`evm_call_result` is an ABI encoded result of the function execution and `proof` is a Solidity `Proof` structure to prepend in verifier function. Note that settlement block is only available in receipt, as we don't want to make assumption on when the the settlement block is assigned.

`metrics` contains aggregated statistics gathered during proof generation. `gas` corresponds to gas used in the preflight step, while `cycles` is the number of CPU cycles utilized during proving. Additionally, `times.preflight` and `times.proving` are both expressed in milliseconds.

### Error

```json
{
  "jsonrpc": "2.0",
  "error": {
    "message": "<error message>",
  }
}
```

## v_getChainProof

### Query

This call takes chain ID and an array of block numbers as an argument.

An example call could look like this:

```json
{
  "method": "v_getChainProof",
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

# Gas meter JSON-RPC API

## v_allocateGas

```json
{
  "method": "v_allocateGas",
  "params": [
      {
          "hash": "0xdeadbeef",
          "gas_limit": 1000000,
          "time_to_live": 3600
      }
  ]
}
```

## v_refundUnusedGas

```json
{
  "method": "v_refundUnusedGas",
  "params": [
      {
          "hash": "0xdeadbeef",
          "gas_used": 1000000,
          "computation_stage": "preflight"
      }
  ]
}
```

```json
{
  "method": "v_refundUnusedGas",
  "params": [
      {
          "hash": "0xdeadbeef",
          "gas_used": 1000000,
          "computation_stage": "proving"
      }
  ]
}
```
